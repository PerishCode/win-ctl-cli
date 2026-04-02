$ErrorActionPreference = 'Stop'
$out = $args[0]
$patterns = @($args | Select-Object -Skip 1)
if (-not $out -or $patterns.Count -eq 0) { Write-Host 'usage: checksum.ps1 <output> <file> [file ...]' -ForegroundColor Red; exit 1 }

$files = @()
foreach ($pattern in $patterns) {
  $matched = @(Get-ChildItem -Path $pattern -File -ErrorAction SilentlyContinue)
  if ($matched.Count -gt 0) {
    $files += $matched | ForEach-Object { $_.FullName }
  } elseif (Test-Path -LiteralPath $pattern -PathType Leaf) {
    $files += (Resolve-Path -LiteralPath $pattern).Path
  } else {
    Write-Host "missing file for checksum: $pattern" -ForegroundColor Red
    exit 1
  }
}

$outDir = Split-Path -Parent $out
if ($outDir) { New-Item -ItemType Directory -Force -Path $outDir | Out-Null }
Set-Content -LiteralPath $out -Value ''
foreach ($file in $files) {
  $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash.ToLowerInvariant()
  Add-Content -LiteralPath $out -Value "$hash *$([System.IO.Path]::GetFileName($file))"
}
