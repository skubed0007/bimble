@echo off
setlocal

REM Set installation directory
set INSTALL_DIR=%ProgramFiles%\bimble

REM Check if the directory exists
if exist "%INSTALL_DIR%" (
    echo Removing %INSTALL_DIR%
    rmdir /s /q "%INSTALL_DIR%"
    echo Uninstallation complete.
) else (
    echo Directory %INSTALL_DIR% does not exist. Nothing to uninstall.
)

REM Optionally, you might want to remove the PATH entry if it's not needed anymore.
REM This requires more advanced scripting and caution to avoid modifying PATH unintentionally.

endlocal
