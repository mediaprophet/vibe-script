param(
    [string]$EngineRoot = (Join-Path $PSScriptRoot '../../../qualia-27062026'),
    [string]$SourceLock = (Join-Path $PSScriptRoot 'engine-source.json')
)
$ErrorActionPreference = 'Stop'
$lock = Get-Content -LiteralPath $SourceLock -Raw | ConvertFrom-Json
if (-not (Test-Path -LiteralPath $EngineRoot -PathType Container)) { throw "Engine source missing: $EngineRoot" }
$root = (Resolve-Path -LiteralPath $EngineRoot).Path
if ($lock.revision -notmatch '^[0-9a-f]{40}$') { throw 'Engine revision must be a full immutable Git commit' }
$actual = git -c "safe.directory=$root" -C $root rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw 'Cannot inspect engine revision' }
if ($actual.Trim() -ne $lock.revision) { throw "Engine revision mismatch: expected $($lock.revision); actual $actual" }
foreach ($relative in $lock.requiredPaths) {
    if (-not (Test-Path -LiteralPath (Join-Path $root $relative) -PathType Leaf)) { throw "Required engine source missing: $relative" }
    git -c "safe.directory=$root" -C $root ls-files --error-unmatch -- $relative | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "Required engine source is uncommitted: $relative" }
}
$dirty = git -c "safe.directory=$root" -C $root status --porcelain --untracked-files=all -- Cargo.toml Cargo.lock crates/vibe crates/vibescript-abi
if ($LASTEXITCODE -ne 0) { throw 'Cannot inspect engine source cleanliness' }
if ($dirty) { throw 'Engine dependency sources are dirty; reproducible CI requires committed source' }
if (-not $lock.qualified) { throw "Unqualified engine source: $($lock.blocker)" }
Write-Output "Verified engine $actual at $root"
