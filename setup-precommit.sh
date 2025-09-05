#!/bin/bash

# Setup script for pre-commit hooks
set -e

echo "🔧 Setting up pre-commit hooks for leptos-forms project..."

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "📦 Installing pre-commit..."
    if command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        pip install pre-commit
    elif command -v brew &> /dev/null; then
        brew install pre-commit
    else
        echo "❌ Please install pre-commit manually: https://pre-commit.com/#installation"
        exit 1
    fi
else
    echo "✅ pre-commit is already installed"
fi

# Install the git hook scripts
echo "🔗 Installing git hook scripts..."
pre-commit install

# Install commit-msg hook for conventional commits (optional)
pre-commit install --hook-type commit-msg

# Update pre-commit hooks to latest versions
echo "🔄 Updating pre-commit hooks to latest versions..."
pre-commit autoupdate

# Run pre-commit on all files to test
echo "🧪 Testing pre-commit hooks on all files..."
pre-commit run --all-files

echo "✅ Pre-commit hooks setup complete!"
echo ""
echo "📋 What was installed:"
echo "  • Rust formatting (rustfmt)"
echo "  • Rust linting (clippy)"
echo "  • Rust compilation check"
echo "  • File formatting (prettier)"
echo "  • Markdown linting"
echo "  • YAML/JSON/TOML validation"
echo "  • Security scanning (detect-secrets)"
echo "  • Trailing whitespace removal"
echo "  • End-of-file fixes"
echo "  • Merge conflict detection"
echo "  • Large file detection"
echo ""
echo "🚀 You can now commit with confidence! The hooks will run automatically."
