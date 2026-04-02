# Install

Use the release install script when a tagged release exists:

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/win-ctl-cli/main/scripts/manage/install.sh | sh -s -- --version v0.1.2
```

For local development:

```bash
cargo build --workspace
./target/debug/win-ctl-cli --version
```
