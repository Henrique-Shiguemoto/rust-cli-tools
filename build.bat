:: Script created by CHATGPT

@echo off
setlocal enabledelayedexpansion

set "srcFolder=src"

if not exist "%srcFolder%" (
    echo The 'src' folder does not exist.
    exit /b
)

for /r "%srcFolder%" %%f in (*.rs) do (
    echo Compiling: %%f
    rustc "%%f"
)

del *.pdb

echo Finished!
