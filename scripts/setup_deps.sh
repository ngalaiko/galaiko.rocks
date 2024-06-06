#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail

command_exists() {
	command -v "$1" &>/dev/null
}

# Determine OS type
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	INSTALL_CMD="sudo apt-get install -y"
	PACKAGE_MANAGER_UPDATE="sudo apt-get update"
elif [[ "$OSTYPE" == "darwin"* ]]; then
	INSTALL_CMD="brew install"
	PACKAGE_MANAGER_UPDATE="brew update"
else
	echo "Unsupported OS type: $OSTYPE"
	exit 1
fi

# Update package manager
echo "Updating package manager..."
$PACKAGE_MANAGER_UPDATE

# Function to install a package if not installed
install_if_not_installed() {
	PACKAGE_NAME=$1
	INSTALL_COMMAND=$2
	if ! command_exists "$PACKAGE_NAME"; then
		echo "$PACKAGE_NAME not found. Installing..."
		$INSTALL_COMMAND "$PACKAGE_NAME"
	else
		echo "$PACKAGE_NAME is already installed."
	fi
}

# Install pandoc
install_if_not_installed "pandoc" "$INSTALL_CMD"

# Install imagemagick
install_if_not_installed "convert" "$INSTALL_CMD imagemagick"

# Install jq
install_if_not_installed "jq" "$INSTALL_CMD"

# Install yq
install_if_not_installed "yq" "$INSTALL_CMD"

# Install python3
install_if_not_installed "python3" "$INSTALL_CMD"

# Install pip if not installed
install_if_not_installed "pip3" "python3 -m ensurepip --upgrade"

# Install packages from requirements.txt if the file exists
if [[ -f "requirements.txt" ]]; then
	echo "Installing packages from requirements.txt..."
    python3 -m pip install -r requirements.txt
fi

# Install CookLang CLI
COOKCLI_URL="https://github.com/cooklang/cookcli/releases/download/v0.8.0/cook-x86_64-unknown-linux-gnu.tar.gz"
COOKCLI_NAME="cook"

if ! command_exists "$COOKCLI_NAME"; then
	if [[ "$OSTYPE" == "linux-gnu"* ]]; then
		echo "Installing CookLang CLI from URL..."
		curl -L $COOKCLI_URL -o cookcli.tar.gz
		tar -xzf cookcli.tar.gz
		sudo mv cook /usr/local/bin/
		rm cookcli.tar.gz
	elif [[ "$OSTYPE" == "darwin"* ]]; then
		echo "Installing CookLang CLI using brew..."
		brew tap cooklang/tap
		brew install cooklang
	fi
else
	echo "CookLang CLI is already installed."
fi

echo "All specified software is installed and up-to-date."
