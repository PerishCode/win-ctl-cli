# win-ctl-cli

Seal the control surface.

一个用于 Windows 控制类 CLI 的可复用仓库骨架，包含明确的文档、发布和治理流程。

当前公开发布线为 `0.1.0`，下面的安装和更新示例都显式固定到这个稳定 tag。

English canonical README: [README.md](README.md).

## 10 秒价值句

克隆一个仓库，跑一条验证链路，并确认骨架从文档到发布脚本都已连通。

## 60 秒验证路径

Unix-like shell：

```bash
# 1) 安装当前 beta
curl -fsSL https://raw.githubusercontent.com/PerishCode/win-ctl-cli/main/scripts/manage/install.sh | sh -s -- --version v0.1.0

# 2) 验证二进制已可用
win-ctl-cli --version

# 3) 运行占位命令路径
win-ctl-cli
```

PowerShell 7：

```powershell
# 1) 安装当前 beta
irm https://raw.githubusercontent.com/PerishCode/win-ctl-cli/main/scripts/manage/install.ps1 | pwsh -Command - --version v0.1.0

# 2) 验证二进制已可用
win-ctl-cli --version

# 3) 运行占位命令路径
win-ctl-cli
```

期望输出：

```text
win-ctl-cli 0.1.0
win-ctl-cli: repository skeleton
```

## 适用 / 不适用边界

适用场景：

- 需要一个带发布自动化的 Rust CLI 仓库骨架
- 需要英文/中文镜像文档面
- 需要最小主干保护和可重复验证链路

不适用场景：

- 你要的是一个已经完成业务命令的产品
- 你需要复杂领域逻辑已就绪
- 你需要大型多成员 monorepo

## 直达链接

- 安装：[docs/zh-CN/how-to/install.md](docs/zh-CN/how-to/install.md)
- 使用骨架：[docs/zh-CN/how-to/use-profiles.md](docs/zh-CN/how-to/use-profiles.md)
- FAQ：[docs/zh-CN/explanation/faq.md](docs/zh-CN/explanation/faq.md)
- Scoreboard：[docs/zh-CN/explanation/win-ctl-cli-score/native.md](docs/zh-CN/explanation/win-ctl-cli-score/native.md)
- Changelog：[CHANGELOG.md](CHANGELOG.md)

安装路径：

- 二进制：`~/.win-ctl-cli/bin/win-ctl-cli`
- 软链接：`~/.local/bin/win-ctl-cli`

## 常用命令

```bash
# 运行占位二进制
win-ctl-cli

# 查看版本
win-ctl-cli --version

# 显式检查并安装当前 beta
win-ctl-cli self-update --check --version v0.1.0
win-ctl-cli self-update --version v0.1.0
```

## 文档

- 文档站点：https://win-ctl-cli.pages.dev/
- 英文 README：[README.md](README.md)
- 安装指南：[docs/zh-CN/how-to/install.md](docs/zh-CN/how-to/install.md)
- 使用骨架：[docs/zh-CN/how-to/use-profiles.md](docs/zh-CN/how-to/use-profiles.md)
- FAQ：[docs/zh-CN/explanation/faq.md](docs/zh-CN/explanation/faq.md)
- Scoreboard：[docs/zh-CN/explanation/win-ctl-cli-score/native.md](docs/zh-CN/explanation/win-ctl-cli-score/native.md)
- Changelog：[CHANGELOG.md](CHANGELOG.md)

## 验证

Unix-like shell：

```bash
cargo fmt --check
cargo test
pnpm run docs:build
bash scripts/docs/links.sh
bash scripts/docs/alignment.sh
bash scripts/docs/agent-meta.sh
bash scripts/docs/agent-routes.sh
bash scripts/release/smoke.sh --version v0.1.0
```

PowerShell 7：

```powershell
cargo fmt --check
cargo test
pnpm run docs:build
pwsh -File scripts/docs/links.ps1
pwsh -File scripts/docs/alignment.ps1
pwsh -File scripts/docs/agent-meta.ps1
pwsh -File scripts/docs/agent-routes.ps1
pwsh -File scripts/release/smoke.ps1 --version v0.1.0
```

## 故障排查（快速定位）

提交 issue 前，先执行这 3 条命令：

```bash
win-ctl-cli --version
win-ctl-cli
pnpm run docs:build
```

如果命令失败，请把命令和完整输出一起贴到 issue。

## 项目链接

- Releases：https://github.com/PerishCode/win-ctl-cli/releases
- 变更记录：https://github.com/PerishCode/win-ctl-cli/releases
- 文档站点：https://win-ctl-cli.pages.dev/
