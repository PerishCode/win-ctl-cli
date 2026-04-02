param()
function Show-Usage { @'
Usage: smoke.ps1 --version vX.Y.Z[-beta.N]

Run a local release-smoke placeholder for a tagged release.
'@ | Write-Host }

if ($args.Count -ne 2 -or $args[0] -ne '--version') { Show-Usage; exit 1 }
$version = $args[1]
if ($version -notmatch '^v[0-9]+\.[0-9]+\.[0-9]+(-beta\.[0-9]+)?$') { Write-Host "invalid version format: $version" -ForegroundColor Red; exit 1 }
Write-Host "==> release smoke for $version"
Write-Host "PASS release_smoke version=$version"
