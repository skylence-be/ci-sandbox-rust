# ci-sandbox-rust

Minimal Rust project used to validate the Skylence CI shape on GitHub-hosted
runners. The private product repos run the same staged pipeline (format,
clippy, cargo-deny, tests, workflow lint) on self-hosted hardware; this
sandbox exists so the hosted-runner variant has visible, public run results.

## Local git hooks (lefthook)

`lefthook.yml` runs the same gates as CI, locally, before code leaves your
machine. Install once per clone (hooks live in `.git/hooks`, which git never
clones):

```
brew install lefthook   # or: cargo install lefthook / go install github.com/evilmartians/lefthook@latest
lefthook install
```

- **pre-commit** (instant): `cargo fmt --all --check`.
- **pre-push** (mirrors CI, serial to bound RAM): clippy `-D warnings`
  (also enforces `missing_docs`), `cargo deny check`, and
  `cargo llvm-cov --fail-under-lines 100`.

Prereqs on PATH: `cargo`, `rustfmt`/`clippy` components, `cargo-deny`,
`cargo-llvm-cov`. Skip the heavy coverage step ad hoc with
`LEFTHOOK_EXCLUDE=coverage lefthook run pre-push`. On RAM-constrained hosts,
comment the `coverage` command out and rely on the CI gate instead.

Not a product. Expect force-pushes and churn.
