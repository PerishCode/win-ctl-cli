$ErrorActionPreference = 'Stop'
$out = $args[0]
$files = @($args | Select-Object -Skip 1)
if (-not $out -or $files.Count -eq 0) { Write-Host 'usage: checksum.ps1 <output> <file> [file ...]' -ForegroundColor Red; exit 1 }

$outDir = Split-Path -Parent $out
if ($outDir) { New-Item -ItemType Directory -Force -Path $outDir | Out-Null }
Set-Content -LiteralPath $out -Value ''
foreach ($file in $files) {
  if (-not (Test-Path -LiteralPath $file -PathType Leaf)) { Write-Host "missing file for checksum: $file" -ForegroundColor Red; exit 1 }
  $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash.ToLowerInvariant()
  Add-Content -LiteralPath $out -Value "$hash *$([System.IO.Path]::GetFileName($file))"
}
