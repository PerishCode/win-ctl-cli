$ErrorActionPreference = 'Stop'
$root = if ($args.Count -gt 0 -and $args[0]) { $args[0] } else { 'dist' }
if (-not (Test-Path -LiteralPath $root -PathType Container)) { Write-Host "artifact root not found: $root" -ForegroundColor Red; exit 1 }
$checksums = Get-ChildItem -LiteralPath $root -Recurse -File -Filter 'checksums.txt' | Sort-Object FullName
$lines = foreach ($file in $checksums) { Get-Content -LiteralPath $file.FullName }
$lines | Sort-Object -Unique | Set-Content -LiteralPath (Join-Path $root 'checksums.txt')
