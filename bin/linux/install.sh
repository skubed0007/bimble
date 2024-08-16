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
