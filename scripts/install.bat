@echo off
REM Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

echo Installing AS Lang...

REM Check for required tools
where python >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Python is required but not installed.
    echo Please install Python and try again.
    exit /b 1
)

where pip >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: pip is required but not installed.
    echo Please install pip and try again.
    exit /b 1
)

where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust is required but not installed.
    echo Please visit https://rustup.rs/ to install Rust.
    exit /b 1
)

where cmake >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: CMake is required but not installed.
    echo Please install CMake and try again.
    exit /b 1
)

where go >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Go is required but not installed.
    echo Please install Go and try again.
    exit /b 1
)

where julia >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Julia is required but not installed.
    echo Please install Julia and try again.
    exit /b 1
)

REM Create virtual environment
echo Creating Python virtual environment...
python -m venv .venv
call .venv\Scripts\activate.bat

REM Install Python dependencies
echo Installing Python dependencies...
python -m pip install --upgrade pip
pip install -r requirements.txt
pip install setuptools-rust wheel

REM Build Rust components
echo Building Rust components...
cargo build --release

REM Setup language bindings directories
echo Setting up language bindings...
mkdir src\bindings\rust\src 2>nul
mkdir src\bindings\cpp\src 2>nul
mkdir src\bindings\go\src 2>nul
mkdir src\bindings\julia\src 2>nul
mkdir src\bindings\wasm\src 2>nul

REM Build C++ components
echo Building C++ components...
cd src\bindings\cpp
cmake .
cmake --build . --config Release
cd ..\..\..

REM Build Go components
echo Building Go components...
cd src\bindings\go
go mod init aslang/go_ops
go build -buildmode=c-shared -o go_ops.dll
cd ..\..\..

REM Build Julia components
echo Building Julia components...
cd src\bindings\julia
julia -e "using Pkg; Pkg.activate(\".\"); Pkg.instantiate()"
cd ..\..\..

REM Install the package
echo Installing AS Lang...
pip install -e .

REM Create command line link
echo Creating command line link...
mkdir "%USERPROFILE%\AppData\Local\Microsoft\WindowsApps" 2>nul
copy /Y "target\release\aslang.exe" "%USERPROFILE%\AppData\Local\Microsoft\WindowsApps\aslang.exe"

echo Installation complete!
echo Please ensure %USERPROFILE%\AppData\Local\Microsoft\WindowsApps is in your PATH.
echo You can now use 'aslang' command to run AS Lang programs. 