#!/bin/bash
# BATCH 1: Root workspace configuration
set -e

# Create directories needed by later batches (harmless now)
mkdir -p .github/workflows
mkdir -p examples
mkdir -p tests
mkdir -p docs/book/src

# Root Cargo.toml – workspace with all dependency definitions
cat > Cargo.toml << 'EOF'
[workspace]
members = [
  "seedc",
  "seedc-cli",
  "seedvm",
  "seedpkg",
  "seedls",
  "seedfmt",
  "seeddbg",
]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
bincode = "2"
clap = { version = "4", features = ["derive"] }
thiserror = "2"
miette = { version = "7", features = ["fancy"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
rustc-hash = "2"
smallvec = "1"
bitflags = "2"
rand = "0.8"
rand_pcg = "0.3"
blake3 = "1"
hex = "0.4"
toml = "0.8"
semver = "1"
ed25519-dalek = "2"
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
tokio = { version = "1", features = ["full"] }
tower-lsp = "0.20"
lsp-types = "0.95"
ropey = "1"
proptest = "1"
criterion = "0.5"
im = "15"
rayon = "1"
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
EOF

# .gitignore
cat > .gitignore << 'EOF'
/target/
**/*.rs.bk
Cargo.lock
*.aslb
*.seed~
EOF

# README.md
cat > README.md << 'EOF'
# AGENT‑SEED v15.2

The definitive programming language for autonomous agentic systems.
See `docs/book/` for the language manual.
EOF

# LICENSE (Apache 2.0 – full text to be added later)
cat > LICENSE << 'EOF'
Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/
EOF

echo "✅ Batch 1 complete: root workspace configuration"