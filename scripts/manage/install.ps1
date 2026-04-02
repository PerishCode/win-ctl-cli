$ErrorActionPreference = 'Stop'
$repo = $env:WIN_CTL_CLI_INSTALL_REPO
if (-not $repo) { $repo = 'PerishCode/win-ctl-cli' }
$installRoot = Join-Path $HOME '.win-ctl-cli'
$binDir = Join-Path $installRoot 'bin'
$binPath = Join-Path $binDir 'win-ctl-cli.exe'
$localBinDir = Join-Path $HOME '.local/bin'
$linkPath = Join-Path $localBinDir 'win-ctl-cli.exe'

function Usage {
@'
Usage: install.ps1 [--version vX.Y.Z|X.Y.Z]

Install strategy:
  - binary: ~/.win-ctl-cli/bin/win-ctl-cli.exe
  - local copy: ~/.local/bin/win-ctl-cli.exe
'@ | Write-Host
}

function Normalize-Version([string]$raw){ if ($raw.StartsWith('v')) { $raw } else { "v$raw" } }

function Get-Target {
  $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
  switch ($arch) {
    'x64' { 'x86_64-pc-windows-msvc' }
    'arm64' { 'aarch64-pc-windows-msvc' }
    default { throw "unsupported platform architecture: $arch" }
  }
}

function Get-Checksum {
  param([Parameter(Mandatory)][string]$ChecksumsFile,[Parameter(Mandatory)][string]$AssetName)
  foreach ($line in Get-Content -LiteralPath $ChecksumsFile) {
    if ($line -match '^([0-9a-fA-F]{64})\s+\*?(.+)$' -and $Matches[2] -eq $AssetName) { return $Matches[1].ToLowerInvariant() }
  }
  return $null
}

function Get-Sha256 {
  param([Parameter(Mandatory)][string]$Path)
  (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash.ToLowerInvariant()
}

$version = $null
for ($i = 0; $i -lt $args.Count; $i++) {
  switch ($args[$i]) {
    '--version' { $version = $args[$i + 1]; $i++ }
    '-h' { Usage; exit 0 }
    '--help' { Usage; exit 0 }
    default { Write-Host "unknown argument: $($args[$i])" -ForegroundColor Red; Usage; exit 1 }
  }
}
if ($version) { $version = Normalize-Version $version } else { Write-Host 'missing required --version' -ForegroundColor Red; Usage; exit 1 }

if (-not (Get-Command curl.exe -ErrorAction SilentlyContinue)) { throw 'missing required command: curl.exe' }
if (-not (Get-Command tar -ErrorAction SilentlyContinue)) { throw 'missing required command: tar' }

$target = Get-Target
$asset = "win-ctl-cli-$version-$target.tar.gz"
$baseUrl = if ($env:WIN_CTL_CLI_INSTALL_BASE_URL) { "$($env:WIN_CTL_CLI_INSTALL_BASE_URL.TrimEnd('/'))/$version" } else { "https://github.com/$repo/releases/download/$version" }
$assetUrl = "$baseUrl/$asset"
$checksumsUrl = "$baseUrl/checksums.txt"

$tmpRoot = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.IO.Path]::GetRandomFileName())
$tmpDir = New-Item -ItemType Directory -Force -Path $tmpRoot
try {
  Write-Host "Downloading $assetUrl"
  & curl.exe -fsSL -o (Join-Path $tmpDir.FullName $asset) $assetUrl
  & curl.exe -fsSL -o (Join-Path $tmpDir.FullName 'checksums.txt') $checksumsUrl

  $expected = Get-Checksum -ChecksumsFile (Join-Path $tmpDir.FullName 'checksums.txt') -AssetName $asset
  if (-not $expected) { throw "checksum not found for $asset" }
  $actual = Get-Sha256 -Path (Join-Path $tmpDir.FullName $asset)
  if ($actual -ne $expected) { throw "checksum mismatch: expected $expected, got $actual" }

  New-Item -ItemType Directory -Force -Path $binDir, $localBinDir | Out-Null
  try {
    tar -xzf (Join-Path $tmpDir.FullName $asset) -C $tmpDir.FullName
  } catch {
    throw 'failed to extract release archive with tar'
  }

  $extracted = @(
    Join-Path $tmpDir.FullName 'win-ctl-cli.exe'
    Join-Path $tmpDir.FullName 'win-ctl-cli'
  ) | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1
  if (-not $extracted) { throw 'release archive missing win-ctl-cli binary' }

  Copy-Item -Force $extracted $binPath
  try { Copy-Item -Force $binPath $linkPath } catch { Write-Host "WARN local link copy failed, binary still installed at $binPath" -ForegroundColor Yellow }
  Write-Host "Installed win-ctl-cli $version to $binPath"
  Write-Host "Linked $linkPath -> $binPath"
  if (($env:PATH -split ';') -notcontains $localBinDir) { Write-Host "Note: $localBinDir is not in PATH." }
} finally {
  Remove-Item -Recurse -Force $tmpDir.FullName -ErrorAction SilentlyContinue
}
