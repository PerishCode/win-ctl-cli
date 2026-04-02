## Summary

- Publish the current beta `win-ctl-cli` release assets.
- Include prerelease binaries, checksums, and packaged skills.
- Use this template when a beta is intentionally distributed for validation.

## Validation

- `cargo fmt --check`
- `cargo test`
- `pnpm run docs:build`
- `pwsh -File scripts/release/verify.ps1`
