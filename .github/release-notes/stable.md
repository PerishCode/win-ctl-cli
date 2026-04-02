## Summary

- Publish the current stable `win-ctl-cli` release assets.
- Include release binaries, checksums, and packaged skills.
- Keep release notes concise and focused on repository-level changes.

## Validation

- `cargo fmt --check`
- `cargo test`
- `pnpm run docs:build`
- `bash scripts/release/verify.sh`
