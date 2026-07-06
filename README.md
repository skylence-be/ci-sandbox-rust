# ci-sandbox-rust

Minimal Rust project used to validate the Skylence CI shape on GitHub-hosted
runners. The private product repos run the same staged pipeline (format,
clippy, cargo-deny, tests, workflow lint) on self-hosted hardware; this
sandbox exists so the hosted-runner variant has visible, public run results.

Not a product. Expect force-pushes and churn.
