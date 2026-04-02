# 安装

发布后可使用安装脚本：

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/win-ctl-cli/main/scripts/manage/install.sh | sh -s -- --version v0.1.1
```

本地开发可直接：

```bash
cargo build --workspace
./target/debug/win-ctl-cli --version
```
