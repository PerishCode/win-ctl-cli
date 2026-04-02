$ErrorActionPreference = 'Stop'
$repoDir = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$skillsDir = Join-Path $repoDir 'skills'
$version = if ($args.Count -gt 0 -and $args[0]) { $args[0] } else { $null }
if (-not $version) {
  $cargoToml = Join-Path $repoDir 'app\Cargo.toml'
  foreach ($line in Get-Content -LiteralPath $cargoToml) { if ($line -match '^version\s*=') { $version = 'v' + (($line -replace '"','' -replace '^version\s*=\s*','').Trim()); break } }
}
if (-not $version.StartsWith('v')) { $version = "v$version" }
if (-not (Test-Path -LiteralPath $skillsDir -PathType Container)) { Write-Host "skills directory not found: $skillsDir" -ForegroundColor Red; exit 1 }
New-Item -ItemType Directory -Force -Path (Join-Path $repoDir 'dist') | Out-Null
$out = Join-Path $repoDir "dist/skill-$version.zip"
if (Test-Path -LiteralPath $out) { Remove-Item -Force $out }
Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [System.IO.Compression.ZipFile]::Open($out, 'Create')
try {
  $manifest = [System.Text.Encoding]::UTF8.GetBytes((@{ version = $version; format = 1; root = '.' } | ConvertTo-Json -Depth 4) + "`n")
  $entry = $zip.CreateEntry('skill-manifest.json')
  $stream = $entry.Open(); $stream.Write($manifest,0,$manifest.Length); $stream.Dispose()
  Get-ChildItem -LiteralPath $skillsDir -Recurse -File | ForEach-Object { [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, $_.FullName, $_.FullName.Substring($skillsDir.Length + 1)) | Out-Null }
} finally { $zip.Dispose() }
Write-Host "Built $out"
