#!/bin/bash
# BATCH 10: CLI, SDK, TESTS, BENCHMARKS, DOCS
set -e

mkdir -p cli sdk/{typescript,python,rust} tests/{conformance/categories,property,fuzzing} benchmarks docs/api

# cli
cat > cli/build.ts << 'EOF'
export function build() { console.log('build'); }
EOF

cat > cli/run.ts << 'EOF'
export function run() { console.log('run'); }
EOF

cat > cli/audit.ts << 'EOF'
export function audit() { console.log('audit'); }
EOF

cat > cli/prove.ts << 'EOF'
export function prove() { console.log('prove'); }
EOF

cat > cli/test.ts << 'EOF'
export function test() { console.log('test'); }
EOF

cat > cli/conformance.ts << 'EOF'
export function conformance() { console.log('conformance'); }
EOF

# sdk
cat > sdk/typescript/index.ts << 'EOF'
export * from '../../lang/parser/parser.js';
EOF

cat > sdk/python/__init__.py << 'EOF'
# Python SDK placeholder
EOF

cat > sdk/rust/lib.rs << 'EOF'
pub fn run() { println!("rust sdk"); }
EOF

# tests (just a few placeholder test files)
cat > tests/conformance/categories/s0-grammar.ts << 'EOF'
// conformance: S0 grammar
EOF

cat > tests/conformance/categories/effects.ts << 'EOF'
// conformance: effects
EOF

cat > tests/property/effects.ts << 'EOF'
// property-based tests
EOF

cat > tests/fuzzing/parser.ts << 'EOF'
// fuzzing tests
EOF

# benchmarks
cat > benchmarks/benchmarks.ts << 'EOF'
// benchmarks
EOF

# docs
cat > docs/spec.md << 'EOF'
# AGENT-SEED v15.2 Specification

(Full specification will be authored separately.)
EOF

cat > docs/install.sh << 'EOF'
#!/bin/bash
echo "AGENT-SEED installer placeholder"
EOF
chmod +x docs/install.sh

cat > docs/api/reference.md << 'EOF'
# API Reference

Under construction.
EOF

echo "✅ Batch 10 complete: CLI, SDK, tests, benchmarks, docs"