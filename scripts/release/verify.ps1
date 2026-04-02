$ErrorActionPreference = 'Stop'
$repoDir = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
Push-Location $repoDir
try {
  if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) { throw 'missing required command: pnpm' }
  if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { throw 'missing required command: cargo' }
  if (-not (Get-Command node -ErrorAction SilentlyContinue)) { throw 'missing required command: node' }
  pnpm install --frozen-lockfile
  cargo fmt --check
  cargo test --locked
  if (Test-Path -LiteralPath (Join-Path $repoDir 'docs/.vitepress/cache')) { Remove-Item -Recurse -Force (Join-Path $repoDir 'docs/.vitepress/cache') }
  if (Test-Path -LiteralPath (Join-Path $repoDir 'docs/.vitepress/dist')) { Remove-Item -Recurse -Force (Join-Path $repoDir 'docs/.vitepress/dist') }
  pnpm run docs:build
  & pwsh -File (Join-Path $repoDir 'scripts\docs\links.ps1')
  & pwsh -File (Join-Path $repoDir 'scripts\docs\alignment.ps1')
  & pwsh -File (Join-Path $repoDir 'scripts\docs\agent-meta.ps1')
  & pwsh -File (Join-Path $repoDir 'scripts\docs\agent-routes.ps1')
} finally { Pop-Location }
