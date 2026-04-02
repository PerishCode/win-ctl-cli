# win-ctl-cli

Seal the control surface.

Deterministic repository skeleton for a Windows control CLI with explicit docs, release, and governance flows.

Current public launch line: `0.1.0-beta.0`. Beta install and update examples below pin that tag explicitly.

Chinese docs entrypoint: [README.zh-CN.md](README.zh-CN.md).

## 10-second value

Clone one repository, run one validation path, and confirm the skeleton is wired end to end.

## 60-second verification path

Unix-like shell:

```bash
# 1) install the current beta
curl -fsSL https://raw.githubusercontent.com/PerishCode/win-ctl-cli/main/scripts/manage/install.sh | sh -s -- --version v0.1.0-beta.0

# 2) verify the binary is reachable
win-ctl-cli --version

# 3) run the placeholder command path
win-ctl-cli
```

PowerShell 7:

```powershell
# 1) install the current beta
irm https://raw.githubusercontent.com/PerishCode/win-ctl-cli/main/scripts/manage/install.ps1 | pwsh -Command - --version v0.1.0-beta.0

# 2) verify the binary is reachable
win-ctl-cli --version

# 3) run the placeholder command path
win-ctl-cli
```

Expected output:

```text
win-ctl-cli 0.1.0-beta.0
win-ctl-cli: repository skeleton
```

## Fit / Not fit

Fit when you need:

- A Rust CLI repository with explicit release automation
- A docs surface with English and Chinese mirrors
- Minimal branch protection and repeatable validation paths

Not fit when you need:

- A finished command product
- Rich domain behavior already implemented
- A broad monorepo with many app/packages members

## Direct links

- Install: [docs/how-to/install.md](docs/how-to/install.md)
- Use the skeleton: [docs/how-to/use-profiles.md](docs/how-to/use-profiles.md)
- FAQ: [docs/explanation/faq.md](docs/explanation/faq.md)
- Scoreboard: [docs/explanation/win-ctl-cli-score/native.md](docs/explanation/win-ctl-cli-score/native.md)
- Changelog: [CHANGELOG.md](CHANGELOG.md)

Installed paths:

- Binary: `~/.win-ctl-cli/bin/win-ctl-cli`
- Symlink: `~/.local/bin/win-ctl-cli`

## Common commands

```bash
# run the placeholder binary
win-ctl-cli

# show version
win-ctl-cli --version

# check and install the current beta explicitly
win-ctl-cli self-update --check --version v0.1.0-beta.0
win-ctl-cli self-update --version v0.1.0-beta.0
```

## Docs

- Site: https://win-ctl-cli.pages.dev/
- Chinese README: [README.zh-CN.md](README.zh-CN.md)
- Install: [docs/how-to/install.md](docs/how-to/install.md)
- Use the skeleton: [docs/how-to/use-profiles.md](docs/how-to/use-profiles.md)
- FAQ: [docs/explanation/faq.md](docs/explanation/faq.md)
- Scoreboard: [docs/explanation/win-ctl-cli-score/native.md](docs/explanation/win-ctl-cli-score/native.md)
- Changelog: [CHANGELOG.md](CHANGELOG.md)

## Validation

Unix-like shell:

```bash
cargo fmt --check
cargo test
pnpm run docs:build
bash scripts/docs/links.sh
bash scripts/docs/alignment.sh
bash scripts/docs/agent-meta.sh
bash scripts/docs/agent-routes.sh
bash scripts/release/smoke.sh --version v0.1.0-beta.0
```

PowerShell 7:

```powershell
cargo fmt --check
cargo test
pnpm run docs:build
pwsh -File scripts/docs/links.ps1
pwsh -File scripts/docs/alignment.ps1
pwsh -File scripts/docs/agent-meta.ps1
pwsh -File scripts/docs/agent-routes.ps1
pwsh -File scripts/release/smoke.ps1 --version v0.1.0-beta.0
```

## Troubleshooting Fast Path

Run these before filing an issue:

```bash
win-ctl-cli --version
win-ctl-cli
pnpm run docs:build
```

If one command fails, include the exact command and output in your issue.

## Project Signals

- Releases: https://github.com/PerishCode/win-ctl-cli/releases
- Changelog: https://github.com/PerishCode/win-ctl-cli/releases
- Docs site: https://win-ctl-cli.pages.dev/
