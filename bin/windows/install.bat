@echo off
setlocal

REM Set installation directory
set INSTALL_DIR=%ProgramFiles%\bimble

REM Create the directory if it doesn't exist
if not exist "%INSTALL_DIR%" (
    echo Creating directory %INSTALL_DIR%
    mkdir "%INSTALL_DIR%"
)

REM Copy files to the installation directory
echo Copying files to %INSTALL_DIR%
xcopy /s /y . "%INSTALL_DIR%"

REM Add installation directory to PATH
setx PATH "%PATH%;%INSTALL_DIR%"

echo Installation complete. You may need to restart your command prompt for changes to take effect.
