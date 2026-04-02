$ErrorActionPreference = 'Stop'
$repoDir = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
@('app','docs','scripts/release','scripts/docs','scripts/manage') | ForEach-Object { if (-not (Test-Path (Join-Path $repoDir $_) -PathType Container)) { Write-Host "FAIL agent_routes missing $_" -ForegroundColor Red; exit 1 } }
Write-Host 'PASS agent_routes'
