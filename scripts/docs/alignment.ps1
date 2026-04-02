$ErrorActionPreference = 'Stop'
$repoDir = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$failures = 0
function Pass($m){ Write-Host "PASS $m" }
function Fail($m){ Write-Host "FAIL $m" -ForegroundColor Red; $script:failures++ }
function Check-Contains($label,$file,$pattern){ if (Select-String -Path $file -Pattern $pattern -Quiet) { Pass $label } else { Fail $label } }
Check-Contains 'readme_links_zh_readme' (Join-Path $repoDir 'README.md') 'README\.zh-CN\.md'
Check-Contains 'readme_has_docs_site_link' (Join-Path $repoDir 'README.md') 'https://win-ctl-cli\.pages\.dev/'
Check-Contains 'readme_mentions_install' (Join-Path $repoDir 'README.md') 'how-to/install'
Check-Contains 'readme_mentions_use_profiles' (Join-Path $repoDir 'README.md') 'how-to/use-profiles'
Check-Contains 'readme_mentions_scoreboard' (Join-Path $repoDir 'README.md') 'explanation/win-ctl-cli-score/native'
Check-Contains 'readme_zh_mentions_install' (Join-Path $repoDir 'README.zh-CN.md') 'how-to/install'
Check-Contains 'readme_zh_mentions_use_profiles' (Join-Path $repoDir 'README.zh-CN.md') 'how-to/use-profiles'
Check-Contains 'readme_zh_mentions_scoreboard' (Join-Path $repoDir 'README.zh-CN.md') 'explanation/win-ctl-cli-score/native'
if ((Test-Path (Join-Path $repoDir 'docs/index.md')) -and (Test-Path (Join-Path $repoDir 'docs/zh-CN/index.md'))) { Pass 'docs_root_mirror_present' } else { Fail 'docs_root_mirror_present' }
if ($failures -gt 0) { exit 1 }
