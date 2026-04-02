. "$PSScriptRoot\common.ps1"

$version = $args[0]
$target = $args[1]
$tools = @($args | Select-Object -Skip 2)
if (-not $version -or -not $target -or $tools.Count -eq 0) { Fail-Release 'usage: package.ps1 <version> <target> <tool> [tool ...]' }
if ($target -notmatch '^(x86_64|aarch64)-.*windows.*$') { Fail-Release "Windows package.ps1 only supports Windows targets, got $target" }

Push-Location $script:RepoDir
try {
  New-Item -ItemType Directory -Force -Path (Join-Path $script:RepoDir 'dist') | Out-Null
  if (-not (Get-Command tar -ErrorAction SilentlyContinue)) { Fail-Release 'missing required command: tar' }
  foreach ($tool in $tools) {
    cargo build --locked --release --target $target --bin $tool
    $built = Join-Path $script:RepoDir ("target/{0}/release/{1}.exe" -f $target, $tool)
    if (-not (Test-Path -LiteralPath $built -PathType Leaf)) {
      $built = Join-Path $script:RepoDir ("target/{0}/release/{1}" -f $target, $tool)
    }
    if (-not (Test-Path -LiteralPath $built -PathType Leaf)) { Fail-Release "built artifact not found for $tool" }
    $staging = Join-Path $script:RepoDir 'dist\staging'
    New-Item -ItemType Directory -Force -Path $staging | Out-Null
    $stagedName = 'win-ctl-cli.exe'
    Copy-Item -Force $built (Join-Path $staging $stagedName)
    tar -C $staging -czf (Join-Path $script:RepoDir "dist/$tool-$version-$target.tar.gz") $stagedName
    Remove-Item -Recurse -Force $staging
  }
} finally { Pop-Location }
