#!/bin/bash

# Exit the script if any command fails
set -e

# Define variables for paths
TARGET_DIR="target"
BIN_DIR="bin"
LINUX_TARGET="x86_64-unknown-linux-musl"
WINDOWS_TARGET="x86_64-pc-windows-gnu"
LINUX_BIN="$TARGET_DIR/$LINUX_TARGET/release/bimble"
WINDOWS_BIN="$TARGET_DIR/$WINDOWS_TARGET/release/bimble.exe"
BIN_LINUX="$BIN_DIR/linux/bimble"
BIN_WINDOWS="$BIN_DIR/windows/bimble.exe"
LB_BJB="./lb.bjb"
WB_BJB="./wb.bjb"
INSTALL_LINUX="$BIN_DIR/linux/install.sh"
INSTALL_WINDOWS="$BIN_DIR/windows/install.bat"
UNINSTALL_LINUX="$BIN_DIR/linux/uninstall.sh"
UNINSTALL_WINDOWS="$BIN_DIR/windows/uninstall.bat"

# Build the project for different targets
cargo build --release --target $LINUX_TARGET
cargo build --release --target $WINDOWS_TARGET

# Create directories
rm -rf $BIN_DIR
mkdir -p $BIN_DIR/linux
mkdir -p $BIN_DIR/windows

# Copy binaries and other files
cp $LINUX_BIN $BIN_LINUX
cp $WINDOWS_BIN $BIN_WINDOWS
cp $LB_BJB $BIN_DIR/linux
cp $WB_BJB $BIN_DIR/linux
cp $LB_BJB $BIN_DIR/windows
cp $WB_BJB $BIN_DIR/windows

# Generate install.sh for Linux
cat << 'EOF' > $INSTALL_LINUX
#!/bin/bash

# Exit the script if any command fails
set -e

# Define the installation directory
INSTALL_DIR="/usr/local/bin/BB"

# Create the installation directory if it doesn't exist
if [ ! -d "$INSTALL_DIR" ]; then
    echo "Creating directory $INSTALL_DIR"
    sudo mkdir -p "$INSTALL_DIR"
fi

# Copy files to the installation directory
echo "Copying files to $INSTALL_DIR"
sudo cp -r ./* "$INSTALL_DIR/"

# Add the installation directory to PATH in user profile files
PROFILE_FILES=("$HOME/.profile" "$HOME/.bashrc" "$HOME/.zshrc")

for PROFILE in "${PROFILE_FILES[@]}"; do
    if [ -f "$PROFILE" ]; then
        if ! grep -q "$INSTALL_DIR" "$PROFILE"; then
            echo "Adding $INSTALL_DIR to PATH in $PROFILE"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$PROFILE"
        fi
    fi
done

echo "Installation complete. You can now run 'bimble' from anywhere."
EOF

# Make install.sh executable
chmod +x $INSTALL_LINUX

# Generate uninstall.sh for Linux
cat << 'EOF' > $UNINSTALL_LINUX
#!/bin/bash

# Exit the script if any command fails
set -e

# Define the installation directory
INSTALL_DIR="/usr/local/bin/BB"

# Define user profile files
PROFILE_FILES=("$HOME/.profile" "$HOME/.bashrc" "$HOME/.zshrc")

# Remove the installation directory from PATH in user profile files
for PROFILE in "${PROFILE_FILES[@]}"; do
    if [ -f "$PROFILE" ]; then
        # Backup the original file
        cp "$PROFILE" "$PROFILE.bak"
        
        # Remove lines containing INSTALL_DIR
        grep -v "$INSTALL_DIR" "$PROFILE.bak" > "$PROFILE"
        
        # Remove the backup file
        rm "$PROFILE.bak"
    fi
done

# Remove the installation directory and its contents
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing $INSTALL_DIR"
    sudo rm -rf "$INSTALL_DIR"
    echo "Uninstallation complete."
else
    echo "Directory $INSTALL_DIR does not exist. Nothing to uninstall."
fi
EOF

# Make uninstall.sh executable
chmod +x $UNINSTALL_LINUX

# Generate install.bat for Windows
cat << 'EOF' > $INSTALL_WINDOWS
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
EOF

# Generate uninstall.bat for Windows
cat << 'EOF' > $UNINSTALL_WINDOWS
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
EOF

echo "Build, install, and uninstall scripts generated successfully."
