. "$PSScriptRoot\common.ps1"

function Show-Usage {
  @'
Usage:
  accept.ps1 stable <tag>
  accept.ps1 beta <version>
'@ | Write-Host
}

$mode = $args[0]
$inputVersion = $args[1]
if (-not $mode -or -not $inputVersion) { Show-Usage; Fail-Release 'missing mode or version' }

$version = Normalize-VersionTag $inputVersion
$crateVersion = Get-CargoVersion
if (-not $crateVersion) { Fail-Release 'failed to parse package.version from Cargo.toml' }

switch ($mode) {
  'stable' {
    if ($crateVersion -like '*-*') { Fail-Release "stable releases require non-prerelease Cargo.toml version, got $crateVersion" }
    $expectedTag = "v$crateVersion"
    if ($version -ne $expectedTag) { Fail-Release "tag/version mismatch: got $version, expected $expectedTag" }
  }
  'beta' {
    if ($version -notmatch '^v[0-9]+\.[0-9]+\.[0-9]+-beta\.[0-9]+$') { Fail-Release 'version must match vX.Y.Z-beta.N' }
    $expectedTag = "v$crateVersion"
    if ($version -ne $expectedTag) { Fail-Release "beta version mismatch: got $version, expected $expectedTag" }
  }
  default { Show-Usage; Fail-Release "unknown mode: $mode" }
}

$version
