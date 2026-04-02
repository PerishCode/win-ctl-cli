$script:ScriptDir = Split-Path -Parent $PSCommandPath
$script:RepoDir = (Resolve-Path (Join-Path $script:ScriptDir '..\..')).Path
$script:AppDir = Join-Path $script:RepoDir 'app'

function Fail-Release {
  param([Parameter(Mandatory)][string]$Message)
  Write-Host "FAIL release $Message" -ForegroundColor Red
  exit 1
}

function Normalize-VersionTag {
  param([Parameter(Mandatory)][string]$Raw)
  if ($Raw.StartsWith('v')) { $Raw } else { "v$Raw" }
}

function Get-ReleaseTarget {
  $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
  switch ($arch) {
    'x64' { 'x86_64-pc-windows-msvc' }
    'arm64' { 'aarch64-pc-windows-msvc' }
    default { Fail-Release "unsupported platform architecture: $arch" }
  }
}

function Get-CargoVersion {
  $cargoToml = Join-Path $script:AppDir 'Cargo.toml'
  $inPackage = $false
  foreach ($line in Get-Content -LiteralPath $cargoToml) {
    if ($line -match '^\[package\]') { $inPackage = $true; continue }
    if ($inPackage -and $line -match '^\[') { break }
    if ($inPackage -and $line -match '^version\s*=') {
      return ($line -replace '"', '' -replace '^version\s*=\s*', '').Trim()
    }
  }
  return $null
}
