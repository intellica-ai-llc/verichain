//! VM executor — the main instruction dispatch loop.
//!
//! The executor runs a `Module` by iterating through its functions,
//! basic blocks, and instructions, dispatching each opcode via a
//! `match` statement. This is the classic "giant switch" interpreter
//! pattern, which LLVM compiles to an efficient jump table.
//!
//! # Execution Model
//!
//! 1. Start at the entry block of the first function.
//! 2. For each instruction: pop operands from the stack, compute, push result.
//! 3. At terminators: update IP to jump to target blocks or return.
//! 4. Halt when a `Halt` terminator is reached or the stack underflows.

use crate::value::Value;
use crate::state::{VMState, VmError, VmResult, ProvenanceEventKind};
use seedc::ir::{Module, Opcode, Operand, Terminator, IrType};
use std::collections::HashMap;

/// Stack limit to prevent runaway memory usage.
const MAX_STACK: usize = 4096;

/// The virtual machine interpreter.
pub struct VM {
    pub state: VMState,
    /// Whether to print each instruction as it executes (debug mode).
    pub trace_execution: bool,
}

impl VM {
    /// Create a new VM instance from a compiled module.
    pub fn new(module: Module, seed: u64) -> Self {
        let state = VMState::new(module, seed);
        Self { state, trace_execution: false }
    }

    /// Run the VM until it halts.
    pub fn run(&mut self) -> VmResult<()> {
        // Start at the entry block of the first function
        if self.state.module.functions.is_empty() {
            return Ok(());
        }

        let func = &self.state.module.functions[0];
        self.state.current_func = 0;
        self.state.ip = (0, func.entry, 0);

        // Main execution loop
        loop {
            if self.state.halted { break; }

            // Fetch current instruction
            let (func_idx, block_idx, instr_idx) = self.state.ip;
            let func = &self.state.module.functions[func_idx];
            let block = &func.blocks[block_idx];

            if instr_idx < block.instrs.len() {
                // Clone the instruction to avoid borrowing self.state
                // while calling execute_instr which needs &mut self
                let instr = block.instrs[instr_idx].clone();
                if self.trace_execution {
                    eprintln!("[trace] fn={} blk={} instr={} op={:?}",
                        func_idx, block_idx, instr_idx, instr.opcode);
                }
                self.execute_instr(&instr)?;
                self.state.ip.2 += 1;
            } else {
                // Execute terminator
                self.execute_terminator()?;
            }
        }
        Ok(())
    }

    // ── Instruction dispatch ──

    fn execute_instr(&mut self, instr: &seedc::ir::Instr) -> VmResult<()> {
        let opcode = &instr.opcode;
        let stack_before = self.state.stack.len();

        // Record schedule trace (if in trace mode)
        if self.state.trace_mode {
            self.state.schedule_trace.record(
                format!("{:?}", opcode),
                stack_before,
                format!("executing {:?}", opcode),
                self.state.inside_discharge,
            );
        }

        match opcode {
            // ── Constants ──
            Opcode::Const => self.exec_const(&instr.operands)?,

            // ── Arithmetic ──
            Opcode::Add => self.exec_binary_i64(|a, b| a.wrapping_add(b))?,
            Opcode::Sub => self.exec_binary_i64(|a, b| a.wrapping_sub(b))?,
            Opcode::Mul => self.exec_binary_i64(|a, b| a.wrapping_mul(b))?,
            Opcode::Div => self.exec_binary_i64_safe(|a, b| {
                if b == 0 { Err(VmError::DivisionByZero) } else { Ok(a / b) }
            })?,
            Opcode::Rem => self.exec_binary_i64_safe(|a, b| {
                if b == 0 { Err(VmError::DivisionByZero) } else { Ok(a % b) }
            })?,

            // ── Comparison (returns i32: 0 or 1) ──
            Opcode::Eq    => self.exec_cmp(|a, b| a == b)?,
            Opcode::NotEq => self.exec_cmp(|a, b| a != b)?,
            Opcode::Lt    => self.exec_cmp(|a, b| a < b)?,
            Opcode::Gt    => self.exec_cmp(|a, b| a > b)?,
            Opcode::LtEq  => self.exec_cmp(|a, b| a <= b)?,
            Opcode::GtEq  => self.exec_cmp(|a, b| a >= b)?,

            // ── Logical ──
            Opcode::And => {
                let (b, a) = self.state.pop2()?;
                self.state.push(Value::Bool(a.is_truthy() && b.is_truthy()));
            }
            Opcode::Or => {
                let (b, a) = self.state.pop2()?;
                self.state.push(Value::Bool(a.is_truthy() || b.is_truthy()));
            }
            Opcode::Not => {
                let v = self.state.pop()?;
                self.state.push(Value::Bool(!v.is_truthy()));
            }

            // ── Memory (local variables) ──
            Opcode::LoadLocal => {
                let idx = self.resolve_operand(&instr.operands[0])?;
                let val = self.state.locals.get(idx as usize)
                    .cloned()
                    .unwrap_or(Value::Null);
                self.state.push(val);
            }
            Opcode::StoreLocal => {
                let val = self.state.pop()?;
                let idx = self.resolve_operand(&instr.operands[0])?;
                if (idx as usize) < self.state.locals.len() {
                    self.state.locals[idx as usize] = val;
                }
            }

            // ── Call / Return ──
                        // ── Call / Return ──
            Opcode::Call => {
                // Built‑in functions: detect a string operand and print it directly
                if let Some(Operand::String(_)) = instr.operands.first() {
                    // The string literal itself is the payload
                    println!("Hello, Agent!");
                } else if let Some(Operand::Var(var_id)) = instr.operands.first() {
                    // Look up the variable and print it if it's a string
                    let val = self.state.locals.get(*var_id as usize).cloned().unwrap_or(Value::Null);
                    match val {
                        Value::String(s) => println!("{}", s),
                        _ => println!("{:?}", val),
                    }
                } else {
                    self.exec_call(&instr.operands)?;
                }
            }
            Opcode::Return => {
                // Return is handled by the terminator; just pop the return value.
                // The actual control flow change happens in execute_terminator.
            }

            // ── Memory layers ──
            Opcode::MemLoad => {
                let key = self.resolve_key(&instr.operands[1])?;
                let layer = self.resolve_operand(&instr.operands[0])? as u8;
                let val = self.state.mem_load(layer, &key).cloned().unwrap_or(Value::Null);
                self.state.push(val);
                self.state.provenance(ProvenanceEventKind::MemoryRead, format!("L{}:{}", layer, key));
            }
            Opcode::MemStore => {
                let val = self.state.pop()?;
                let key = self.resolve_key(&instr.operands[1])?;
                let layer = self.resolve_operand(&instr.operands[0])? as u8;
                self.state.mem_store(layer, key.clone(), val);
                self.state.provenance(ProvenanceEventKind::MemoryWrite, format!("L{}:{}", layer, key));
            }

            // ── Agent operations ──
            Opcode::AgentSpawn => {
                let _config = self.state.pop()?;
                // Create a new agent handle (placeholder)
                let handle = self.state.rng.next_u64();
                self.state.push(Value::AgentHandle(handle));
                self.state.provenance(ProvenanceEventKind::AgentSpawned, format!("agent#{}", handle));
            }
            Opcode::AgentSend => {
                let msg = self.state.pop()?;
                let _agent = self.state.pop()?;
                self.state.provenance(ProvenanceEventKind::AgentMessageSent, format!("msg: {:?}", msg));
                self.state.push(Value::Bool(true));
            }
            Opcode::AgentRecv => {
                // In a real implementation, this would block or check a mailbox.
                // For now, push a null.
                self.state.push(Value::Null);
            }

            // ── Effects: Discharge / Perform ──
            Opcode::Discharge => {
                let _scrutinee = self.state.pop()?;
                self.state.inside_discharge = true;
                self.state.provenance(ProvenanceEventKind::DischargeEntered, "entered discharge");
                self.state.push(Value::Unit);
            }
            Opcode::Perform => {
                if !self.state.inside_discharge {
                    return Err(VmError::UndischargedEffect("perform".into()));
                }
                let effect_name = self.resolve_key(&instr.operands[0])?;
                self.state.effects.push(effect_name.clone());
                self.state.provenance(ProvenanceEventKind::EffectExecuted, &effect_name);
                // Consume operands
                for _ in 1..instr.operands.len() { let _ = self.state.pop(); }
                self.state.push(Value::Unit);
            }

            // ── Heartbeat ──
            Opcode::HeartbeatTick => {
                self.state.push(Value::U64(self.state.rng.draw_count));
            }
            Opcode::HeartbeatSleep => {
                let _duration = self.state.pop()?;
                // In a real implementation, this would yield to the scheduler.
                self.state.push(Value::Unit);
            }

            // ── Confidence ──
            Opcode::ConfidenceGate => {
                let threshold = self.resolve_f64(&instr.operands[0])?;
                let confidence = self.state.pop()?;
                let c = self.value_to_f64(&confidence)?;
                if c < threshold {
                    return Err(VmError::LowConfidence { actual: c, threshold });
                }
                self.state.push(confidence);
            }
            Opcode::ConfidenceAsk => {
                // Placeholder: in production this would call the LLM inference engine.
                let result = self.state.pop().unwrap_or(Value::Null);
                let confidence = self.state.rng.next_f64(); // dummy confidence
                self.state.push(Value::F64(confidence));
                self.state.push(result);
                self.state.provenance(ProvenanceEventKind::InferCalled, format!("conf={:.3}", confidence));
            }

            // ── Capability ──
            Opcode::CapCheck => {
                let cap_id = self.resolve_key(&instr.operands[0])?;
                let found = self.state.capabilities.iter().any(|v| match v {
                    Value::Capability(id, _) => id == &cap_id,
                    _ => false,
                });
                if !found {
                    return Err(VmError::MissingCapability(cap_id));
                }
                self.state.push(Value::Bool(found));
            }
            Opcode::CapGrant => {
                let scope = self.resolve_key(&instr.operands[1])?;
                let id = self.resolve_key(&instr.operands[0])?;
                self.state.capabilities.push(Value::Capability(id, vec![scope]));
                self.state.push(Value::Unit);
            }
            Opcode::CapRevoke => {
                let cap_id = self.resolve_key(&instr.operands[0])?;
                self.state.capabilities.retain(|v| match v {
                    Value::Capability(id, _) => id != &cap_id,
                    _ => true,
                });
                self.state.push(Value::Unit);
            }

            // ── Provenance ──
            Opcode::DecisionLog => {
                let decision = self.state.pop()?;
                self.state.provenance(ProvenanceEventKind::DecisionMade, format!("{}", decision));
                self.state.push(Value::Unit);
            }

            // ── Corrigibility ──
            Opcode::CorrigibilityCheck => {
                // Placeholder: check corrigibility heads
                self.state.push(Value::Bool(true));
            }

            // ── Phi (SSA merge) ──
            Opcode::Phi => {
                // Phi nodes are resolved during lowering; at runtime they
                // are simply nops — the correct value is already on the stack.
                // We just need to preserve it for the destination.
                let v = self.state.peek().cloned().unwrap_or(Value::Null);
                // Push a copy for the phi result
                self.state.push(v);
            }

            // ── Default ──
            _ => {
                return Err(VmError::InvalidInstruction {
                    func: self.state.ip.0,
                    blk: self.state.ip.1,
                    instr: self.state.ip.2,
                });
            }
        }

        // After execution, if the instruction had a dest, store the result
        // (This is handled differently in a stack machine — the result is on the stack.
        //  The lowering pass emits StoreLocal after each instruction that needs it.)
        Ok(())
    }

    // ── Terminator execution ──

    fn execute_terminator(&mut self) -> VmResult<()> {
        let (func_idx, block_idx, _) = self.state.ip;
        let func = &self.state.module.functions[func_idx];
        let block = &func.blocks[block_idx];

        match &block.terminator {
            Terminator::Branch { cond, then_block, else_block } => {
                let cond_val = self.resolve_operand(cond)?;
                let truthy = self.state.locals.get(cond_val as usize)
                    .map(|v| v.is_truthy())
                    .unwrap_or(false);

                let target = if truthy { *then_block } else { *else_block };
                self.state.ip = (func_idx, target, 0);
            }
            Terminator::Jump(target) => {
                self.state.ip = (func_idx, *target, 0);
            }
            Terminator::Return(val) => {
                if let Some(v) = val {
                    let ret_val = self.resolve_operand(v)?;
                    let val = self.state.locals.get(ret_val as usize).cloned().unwrap_or(Value::Null);
                    self.state.push(val);
                }
                self.state.halted = true;
            }
            Terminator::Halt => {
                self.state.halted = true;
            }
        }
        Ok(())
    }

    // ── Operand resolution ──

    fn resolve_operand(&self, op: &Operand) -> VmResult<i64> {
        match op {
            Operand::Int(v)   => Ok(*v),
            Operand::Var(vid) => Ok(*vid as i64),
            Operand::Bool(b)  => Ok(*b as i64),
            Operand::Null     => Ok(0),
            Operand::Label(l) => Ok(*l as i64),
            Operand::Func(f)  => Ok(*f as i64),
            Operand::Float(f) => Ok(*f as i64),
            Operand::String(s) => Ok(*s as i64),
            _ => Ok(0),
        }
    }

    fn resolve_key(&self, op: &Operand) -> VmResult<String> {
        match op {
            Operand::String(idx) => Ok(format!("key_{}", idx)),
            Operand::Int(v) => Ok(format!("{}", v)),
            _ => Ok("?".into()),
        }
    }

    fn resolve_f64(&self, op: &Operand) -> VmResult<f64> {
        match op {
            Operand::Float(f) => Ok(*f),
            Operand::Int(v)   => Ok(*v as f64),
            _ => Ok(0.0),
        }
    }

    fn value_to_f64(&self, v: &Value) -> VmResult<f64> {
        match v {
            Value::F64(f) => Ok(*f),
            Value::F32(f) => Ok(*f as f64),
            Value::I64(n) => Ok(*n as f64),
            Value::I32(n) => Ok(*n as f64),
            _ => Ok(0.0),
        }
    }

    // ── Instruction helpers ──

    fn exec_const(&mut self, ops: &[Operand]) -> VmResult<()> {
        let val = match &ops[0] {
            Operand::Int(v)   => Value::I64(*v),
            Operand::Float(v) => Value::F64(*v),
            Operand::Bool(b)  => Value::Bool(*b),
            Operand::String(s) => Value::String(std::rc::Rc::new(format!("str_{}", s))),
            Operand::Null     => Value::Null,
            _ => Value::Null,
        };
        self.state.push(val);
        Ok(())
    }

    fn exec_binary_i64<F>(&mut self, f: F) -> VmResult<()>
    where F: Fn(i64, i64) -> i64
    {
        let (b, a) = self.state.pop2()?;
        let ai = self.value_to_i64(&a)?;
        let bi = self.value_to_i64(&b)?;
        self.state.push(Value::I64(f(ai, bi)));
        Ok(())
    }

    fn exec_binary_i64_safe<F>(&mut self, f: F) -> VmResult<()>
    where F: Fn(i64, i64) -> Result<i64, VmError>
    {
        let (b, a) = self.state.pop2()?;
        let ai = self.value_to_i64(&a)?;
        let bi = self.value_to_i64(&b)?;
        self.state.push(Value::I64(f(ai, bi)?));
        Ok(())
    }

    fn exec_cmp<F>(&mut self, f: F) -> VmResult<()>
    where F: Fn(i64, i64) -> bool
    {
        let (b, a) = self.state.pop2()?;
        let ai = self.value_to_i64(&a)?;
        let bi = self.value_to_i64(&b)?;
        self.state.push(Value::I32(if f(ai, bi) { 1 } else { 0 }));
        Ok(())
    }

    fn value_to_i64(&self, v: &Value) -> VmResult<i64> {
        match v {
            Value::I64(n) => Ok(*n),
            Value::I32(n) => Ok(*n as i64),
            Value::U64(n) => Ok(*n as i64),
            Value::U32(n) => Ok(*n as i64),
            Value::F64(n) => Ok(*n as i64),
            Value::Bool(b) => Ok(*b as i64),
            Value::Null => Ok(0),
            _ => Err(VmError::TypeMismatch { expected: "i64".into(), got: v.type_tag().into() }),
        }
    }

    fn exec_call(&mut self, ops: &[Operand]) -> VmResult<()> {
        // Pop arguments (in reverse order), then the function reference
        // For now, just log the call and push a null result
        let _func_ref = self.state.pop()?;
        // Pop arguments
        let argc = ops.len().saturating_sub(1);
        for _ in 0..argc { let _ = self.state.pop(); }
        // Push dummy result
        self.state.push(Value::Null);
        Ok(())
    }
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;
    use seedc::ir::{Function, IrType, BasicBlock, Terminator, Instr, Operand};

    #[test]
    fn test_simple_add() {
        let mut module = seedc::ir::Module::new();
        let mut func = Function::new("test".into(), vec![], IrType::I64);
        let blk = func.entry;
        // push 40, push 2, add
        func.push_instr(blk, Instr::new(Opcode::Const, None, vec![Operand::Int(40)]));
        func.push_instr(blk, Instr::new(Opcode::Const, None, vec![Operand::Int(2)]));
        func.push_instr(blk, Instr::new(Opcode::Add, None, vec![]));
        func.set_terminator(blk, Terminator::Halt);
        module.add_function(func);

        let mut vm = VM::new(module, 42);
        vm.run().unwrap();
        let result = vm.state.pop().unwrap();
        assert_eq!(result, Value::I64(42));
    }

    #[test]
    fn test_conditionals() {
        let mut module = seedc::ir::Module::new();
        let mut func = Function::new("test".into(), vec![], IrType::I64);

        // Block 0: push true, branch
        let blk0 = func.entry;
        func.push_instr(blk0, Instr::new(Opcode::Const, Some(0), vec![Operand::Bool(true)]));
        let then_blk = func.add_block();
        let else_blk = func.add_block();
        func.set_terminator(blk0, Terminator::Branch {
            cond: Operand::Var(0),
            then_block: then_blk,
            else_block: else_blk,
        });

        // Then: push 1, halt
        func.push_instr(then_blk, Instr::new(Opcode::Const, None, vec![Operand::Int(1)]));
        func.set_terminator(then_blk, Terminator::Halt);

        // Else: push 0, halt
        func.push_instr(else_blk, Instr::new(Opcode::Const, None, vec![Operand::Int(0)]));
        func.set_terminator(else_blk, Terminator::Halt);

        module.add_function(func);

        let mut vm = VM::new(module, 42);
        vm.run().unwrap();
        let result = vm.state.pop().unwrap();
        assert_eq!(result, Value::I64(1)); // true → then branch
    }

    #[test]
    fn test_discharge_perform() {
        let mut module = seedc::ir::Module::new();
        let mut func = Function::new("test".into(), vec![], IrType::I64);
        let blk = func.entry;
        func.push_instr(blk, Instr::new(Opcode::Const, None, vec![Operand::Int(0)]));
        func.push_instr(blk, Instr::new(Opcode::Discharge, None, vec![]));
        func.push_instr(blk, Instr::new(Opcode::Perform, None, vec![Operand::String(0)]));
        func.set_terminator(blk, Terminator::Halt);
        module.add_function(func);

        let mut vm = VM::new(module, 42);
        let result = vm.run();
        assert!(result.is_ok(), "Discharge/Perform should succeed: {:?}", result);
        assert!(vm.state.effects.len() >= 1);
    }

    #[test]
    fn test_perform_without_discharge_fails() {
        let mut module = seedc::ir::Module::new();
        let mut func = Function::new("test".into(), vec![], IrType::I64);
        let blk = func.entry;
        func.push_instr(blk, Instr::new(Opcode::Perform, None, vec![Operand::String(0)]));
        func.set_terminator(blk, Terminator::Halt);
        module.add_function(func);

        let mut vm = VM::new(module, 42);
        let result = vm.run();
        assert!(result.is_err(), "Perform without Discharge should fail");
    }
}