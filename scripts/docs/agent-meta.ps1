$ErrorActionPreference = 'Stop'
$repoDir = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
Select-String -Path (Join-Path $repoDir 'AGENTS.md') -Pattern '^# AGENTS' -Quiet | Out-Null
Select-String -Path (Join-Path $repoDir 'AGENTS.md') -Pattern '## Core Principle' -Quiet | Out-Null
Select-String -Path (Join-Path $repoDir 'AGENTS.md') -Pattern '## Directory Conventions' -Quiet | Out-Null
Write-Host 'PASS agent_meta'
