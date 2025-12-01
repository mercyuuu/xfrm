@echo off
setlocal enabledelayedexpansion

echo.
echo ================================
echo    Installing xfrm for Windows
echo ================================
echo.

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Cargo is not installed.
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo [1/3] Building xfrm in release mode...
cargo build --release

if not exist "target\release\xfrm.exe" (
    echo [ERROR] Build failed
    pause
    exit /b 1
)

set "INSTALL_DIR=%USERPROFILE%\.cargo\bin"
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

echo [2/3] Installing to %INSTALL_DIR%...
copy /Y "target\release\xfrm.exe" "%INSTALL_DIR%\xfrm.exe" >nul

if %errorlevel% neq 0 (
    echo [ERROR] Installation failed
    pause
    exit /b 1
)

echo.
echo ✓ xfrm installed successfully!
echo.
echo Binary location: %INSTALL_DIR%\xfrm.exe

for %%A in ("%INSTALL_DIR%\xfrm.exe") do set size=%%~zA
set /a size_mb=!size! / 1048576
echo Binary size: !size_mb! MB
echo.

echo [3/3] Checking dependencies...
set MISSING_DEPS=

where ffmpeg >nul 2>nul
if %errorlevel% neq 0 (
    set "MISSING_DEPS=!MISSING_DEPS! ffmpeg"
)

where magick >nul 2>nul
if %errorlevel% neq 0 (
    where convert >nul 2>nul
    if %errorlevel% neq 0 (
        set "MISSING_DEPS=!MISSING_DEPS! imagemagick"
    )
)

where pandoc >nul 2>nul
if %errorlevel% neq 0 (
    set "MISSING_DEPS=!MISSING_DEPS! pandoc"
)

if not "!MISSING_DEPS!"=="" (
    echo.
    echo [WARNING] Missing dependencies:!MISSING_DEPS!
    echo.
    echo Install them with:
    echo   - ffmpeg: https://ffmpeg.org/download.html
    echo   - imagemagick: https://imagemagick.org/script/download.php
    echo   - pandoc: https://pandoc.org/installing.html
    echo.
    echo Or use a package manager like Chocolatey:
    echo   choco install!MISSING_DEPS!
    echo.
    echo Or use Scoop:
    echo   scoop install!MISSING_DEPS!
) else (
    echo ✓ All dependencies are installed!
)

echo.
echo ================================
echo    Installation Complete!
echo ================================
echo.
echo Try it out:
echo   xfrm --help
echo   xfrm input.mp4 output.mp3
echo.

pause
