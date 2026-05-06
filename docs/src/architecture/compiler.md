
Compiler Pipeline
Lexer → Parser → Type Checker → Effect/Taint Checker → Lowering → IR Verifier → .aslb
IR uses SSA form with explicit Discharge/Perform instructions.
seedc --emit-grammar --stratum S0 --format gbnf exports grammar for constrained decoding.
