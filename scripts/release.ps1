Param(
    [string]$Target = "x86_64-pc-windows-gnu"
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$repo = Resolve-Path (Join-Path $root "..")
Set-Location $repo

if (-not (Test-Path env:CC)) { $env:CC = "C:\msys64\mingw64\bin\gcc.exe" }
if (-not (Test-Path env:AR)) { $env:AR = "C:\msys64\mingw64\bin\ar.exe" }

Write-Host "Building termatama for $Target..." -ForegroundColor Cyan
& cargo clean
& cargo build --release --target $Target

$exe = Join-Path $repo "target\$Target\release\termatama.exe"
if (-not (Test-Path $exe)) {
    throw "Build succeeded but $exe not found"
}

$distDir = Join-Path $repo "dist"
$pkgDir = Join-Path $distDir "termatama-windows"
if (Test-Path $pkgDir) { Remove-Item -Recurse -Force $pkgDir }
New-Item -ItemType Directory -Force -Path $pkgDir | Out-Null

Copy-Item $exe $pkgDir
Copy-Item (Join-Path $repo "README.md") $pkgDir
Copy-Item (Join-Path $repo "roms\README.md") $pkgDir -Force

$zipPath = Join-Path $distDir "termatama-windows.zip"
if (Test-Path $zipPath) { Remove-Item $zipPath }
Write-Host "Packaging $zipPath" -ForegroundColor Cyan
Compress-Archive -Path (Join-Path $pkgDir '*') -DestinationPath $zipPath

Write-Host "Done. Binary: $exe" -ForegroundColor Green
Write-Host "Zip:    $zipPath" -ForegroundColor Green
Write-Host "Place your ROM at roms\\tama.b next to the exe after unpacking." -ForegroundColor Yellow
