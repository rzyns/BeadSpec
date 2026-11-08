[CmdletBinding()]
param([ValidateRange(1, 9999)][int]$Revision = 1)

$ErrorActionPreference = 'Stop'
if (![Environment]::Is64BitOperatingSystem -or $env:OS -ne 'Windows_NT') {
    throw 'Build this package on 64-bit Windows with the x86_64-pc-windows-msvc Rust toolchain.'
}
$repoRoot = Split-Path $PSScriptRoot -Parent
if ((& rustc -vV) -notcontains 'host: x86_64-pc-windows-msvc') {
    throw 'Select the x86_64-pc-windows-msvc Rust toolchain before building.'
}
Push-Location $repoRoot
try {
    $version = (Get-Content src-tauri/tauri.conf.json -Raw | ConvertFrom-Json).version + "-rzyns.$Revision"
    & bun run tauri build --no-bundle --config src-tauri/tauri.scoop.conf.json
    if ($LASTEXITCODE -ne 0) { throw 'Tauri build failed.' }

    $releaseDir = Join-Path $repoRoot 'src-tauri/target/release'
    $outputDir = Join-Path $repoRoot 'dist'
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
    $archive = Join-Path $outputDir "beadspec_${version}_windows_x64.zip"
    $files = @(
        (Join-Path $releaseDir 'beadspec.exe'),
        (Join-Path $repoRoot 'LICENSE-MIT'),
        (Join-Path $repoRoot 'LICENSE-APACHE')
    )
    Compress-Archive -LiteralPath $files -DestinationPath $archive -Force
    $hash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
    "$hash  $(Split-Path $archive -Leaf)" | Set-Content -LiteralPath "$archive.sha256" -Encoding ascii
    Write-Output $archive
    Write-Output $hash
}
finally {
    Pop-Location
}
