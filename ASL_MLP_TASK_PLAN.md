🚀 AGENT‑SEED MLP – Tailored 3‑Day Plan
Day 1 – Unified CLI & Distribution
1.1 Create a unified seed binary
We don’t need a shell‑script wrapper; we can make seedc-cli the single entry point and embed seedvm as a library. This is cleaner, cross‑platform, and avoids subprocess overhead.

Task: Modify seedc-cli to accept run, trace, and prove subcommands, and call seedvm library functions directly.

Changes:

Add seedvm as a dependency in seedc-cli/Cargo.toml:

toml
seedvm = { path = "../seedvm" }
In seedc-cli/src/main.rs, add the new subcommands (Run, Trace, Prove) to the existing CLI enum.
The cmd_run handler now:

Compiles the source with seedc::compile(&source)?
Writes the .aslb to a temp file (or just keeps it in memory)
Calls seedvm::run_bytes(&binary, seed)? directly
Prints the final state summary (exit code, provenance events, schedule steps)
I will provide the exact code changes if you want, but the above is the recipe.

Acceptance: cargo build -p seedc-cli produces a single binary called seed (rename in Cargo.toml [[bin]] name to seed). Running seed run examples/hello.seed prints Hello, Agent!.

1.2 GitHub Actions release workflow
File: .github/workflows/release.yml (create it)

Content: The release workflow from the external plan is almost perfect. I’ll adapt it to build the single seed binary instead of separate seedc/seedvm ones.

Key differences:

Only build one binary: seed (from seedc-cli crate).

Asset naming: seed-{target}.

No need for the shell‑script wrapper.

After this file is pushed, git tag v0.1.0 && git push --tags will produce downloadable binaries.

1.3 Docker image
File: Dockerfile (root)

dockerfile
FROM alpine:latest
RUN apk add --no-cache libc6-compat
COPY target/release/seed /usr/local/bin/seed
ENTRYPOINT ["seed"]
Build & push manually for now (we can automate later):

bash
docker build -t agentseed/seedc:latest .
docker push agentseed/seedc:latest
1.4 npm package @agentseed/cli
We already have the seedpkg crate, but we can add a simple cli/npm/ directory as described. The package.json, bin/seed.js, and scripts/download.js from the external plan are directly applicable. No changes needed.

Extra: Ensure the npm package downloads the single seed binary (not separate seedc/seedvm). The asset naming in the release workflow must match what download.js expects.

Day 2 – Documentation, Examples & Landing Page
2.1 Build and publish the mdBook
Your docs/ directory already has a book.toml and SUMMARY.md. All we need is:

Install mdbook locally: cargo install mdbook

Run mdbook build docs to generate static HTML.

Add the GitHub Actions workflow (docs.yml) to deploy to gh-pages branch.

Acceptance: After push to main, https://agentseedlanguage-cpu.github.io/agentseed/ shows the book.

2.2 Add an examples README and a couple of new demos
examples/README.md (short index of the examples)

examples/research.seed (simple infer<T> usage)

examples/discharge.seed (demonstrates discharge with confidence gate)

Keep hello.seed and agent.seed as they are.

2.3 Auto‑generated API reference
Add cargo doc --no-deps to the docs.yml workflow and copy the output into the book. That gives us docs.agentseed.org/api/.

2.4 Landing page
Create a simple index.html in the root of the gh-pages branch (or in docs/landing/). The landing page should contain:

One‑line install: npm install -g @agentseed/cli

Quick demo: seed run hello.seed → Hello, Agent!

Link to the book, API, GitHub, Discord.

A short asciinema recording (we can record one locally and include a link).

Day 3 – Launch, Community & Polish
3.1 Publish the npm package
bash
cd cli/npm
npm publish --access public
3.2 Launch blog post
Write a post for dev.to / Medium. The structure is ready in the external plan; we can reuse the abstract from your paper and the language overview.

3.3 Post to HN, Reddit, etc.
Prepare a Show HN post, a Reddit post, and a tweet. Schedule them for the same day.

3.4 Community channels
Create a Discord server (or use GitHub Discussions first).

Add links to README, book, and landing page.

3.5 Final polish
Ensure seed run works end‑to‑end without needing a separate seedvm binary.

Run the full test suite on the unified binary: cargo test --workspace.

Tag a release v0.1.0 and push it, triggering the workflow.

What This Plan Achieves
By the end of Day 3, we will have:

One command to rule them all: seed build/run/check/emit-ir from a single binary.

Zero‑friction install: npm install -g @agentseed/cli (and later Homebrew, Docker).

Professional documentation: mdBook + API reference live on GitHub Pages.

Compelling demos: hello.seed, agent.seed, research.seed, discharge.seed.

Public presence: landing page, launch blog, HN/Reddit posts, Discord.

All of this is feasible without touching the core compiler or VM — it’s purely integration and distribution work. We can resume Phase C immediately after, with a project that already looks alive to the outside world.
