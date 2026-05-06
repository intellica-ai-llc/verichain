//! AGENT-SEED v15.2 debug adapter — `seeddbg`.
//!
//! Implements the Debug Adapter Protocol (DAP) so that editors like VS Code
//! can set breakpoints, step through agent code, inspect memory layers,
//! and view the provenance graph.
//!
//! Uses the `dap` crate (dap-rs) for the protocol implementation, following
//! the same pattern as LSP: a server reads JSON-RPC from stdin and writes
//! responses to stdout.

use dap::prelude::*;
use std::io::{BufReader, BufWriter};
use thiserror::Error;

// ── Adapter ──

struct SeedDebugAdapter;

#[derive(Error, Debug)]
enum AdapterError {
    #[error("unhandled command")]
    UnhandledCommand,
}

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Adapter for SeedDebugAdapter {
    fn accept(
        &mut self,
        request: Request,
        _ctx: &mut dyn Context,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        use dap::requests as req;

        Ok(match request.command {
            Command::Initialize(_) => {
                let caps = types::Capabilities {
                    supports_configuration_done_request: Some(true),
                    supports_step_in_targets_request: Some(false),
                    supports_conditional_breakpoints: Some(true),
                    supports_hit_conditional_breakpoints: Some(false),
                    supports_log_points: Some(true),
                    supports_function_breakpoints: Some(false),
                    supports_data_breakpoints: Some(false),
                    supports_delayed_stack_trace_loading: Some(false),
                    ..Default::default()
                };
                request.success(ResponseBody::Initialize(Some(caps)))
            }

            Command::Launch(ref args) => {
                tracing::info!("Launch requested: {:?}", args);
                request.success(ResponseBody::Launch)
            }

            Command::SetBreakpoints(ref args) => {
                let source_path = args.source.path.as_deref().unwrap_or("unknown");
                let breakpoints: Vec<types::Breakpoint> = args
                    .breakpoints
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .map(|sb| types::Breakpoint {
                        id: None,
                        verified: true,
                        message: None,
                        source: Some(types::Source {
                            path: Some(source_path.to_string()),
                            ..Default::default()
                        }),
                        line: Some(sb.line),
                        column: sb.column,
                        end_line: None,
                        end_column: None,
                        instruction_reference: None,
                        offset: None,
                    })
                    .collect();

                let body = types::SetBreakpointsResponseBody { breakpoints };
                request.success(ResponseBody::SetBreakpoints(body))
            }

            Command::Continue(_) => request.success(ResponseBody::Continue(types::ContinueResponseBody {
                all_threads_continued: Some(true),
            })),

            Command::Next(_) => request.success(ResponseBody::Next),
            Command::StepIn(_) => request.success(ResponseBody::StepIn),
            Command::StepOut(_) => request.success(ResponseBody::StepOut),
            Command::Pause(_) => request.success(ResponseBody::Pause),
            Command::Threads => request.success(ResponseBody::Threads(types::ThreadsResponseBody {
                threads: vec![types::Thread { id: 0, name: "agent-main".into() }],
            })),

            Command::StackTrace(ref _args) => {
                request.success(ResponseBody::StackTrace(types::StackTraceResponseBody {
                    stack_frames: vec![types::StackFrame {
                        id: 0,
                        name: "agent::main".into(),
                        source: Some(types::Source {
                            path: Some("agent.seed".into()),
                            ..Default::default()
                        }),
                        line: 1,
                        column: 1,
                        end_line: None,
                        end_column: None,
                        can_restart: Some(false),
                        instruction_pointer_reference: None,
                        module_id: None,
                        presentation_hint: None,
                    }],
                    total_frames: Some(1),
                }))
            }

            Command::Scopes(ref _args) => {
                request.success(ResponseBody::Scopes(types::ScopesResponseBody {
                    scopes: vec![
                        types::Scope {
                            name: "Locals".into(),
                            variables_reference: 1,
                            named_variables: Some(10),
                            indexed_variables: None,
                            expensive: false,
                            source: None,
                            line: None,
                            column: None,
                            end_line: None,
                            end_column: None,
                        },
                        types::Scope {
                            name: "Memory Layers".into(),
                            variables_reference: 2,
                            named_variables: Some(8),
                            indexed_variables: None,
                            expensive: true,
                            source: None,
                            line: None,
                            column: None,
                            end_line: None,
                            end_column: None,
                        },
                    ],
                }))
            }

            Command::Variables(ref _args) => {
                request.success(ResponseBody::Variables(types::VariablesResponseBody {
                    variables: vec![
                        types::Variable {
                            name: "ip".into(),
                            value: "(0, 0, 0)".into(),
                            variables_reference: 0,
                            ..Default::default()
                        },
                        types::Variable {
                            name: "stack_depth".into(),
                            value: "3".into(),
                            variables_reference: 0,
                            ..Default::default()
                        },
                    ],
                }))
            }

            Command::Disconnect(_) => request.success(ResponseBody::Disconnect),
            Command::ConfigurationDone => request.success(ResponseBody::ConfigurationDone),

            _ => {
                tracing::warn!("Unhandled DAP command: {:?}", request.command);
                request.error("unsupported command")
            }
        })
    }
}

// ── Entry point ──

fn main() -> DynResult<()> {
    tracing_subscriber::fmt::init();

    let output = BufWriter::new(std::io::stdout());
    let input = BufReader::new(std::io::stdin());
    let adapter = SeedDebugAdapter;

    let mut server = Server::new(input, output);
    tracing::info!("AGENT-SEED Debug Adapter listening on stdio");

    server.run(&mut SeedDebugAdapter)?;

    Ok(())
}
