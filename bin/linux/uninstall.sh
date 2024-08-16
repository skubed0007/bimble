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
