#!/bin/bash

# SD Copy - Quick GitHub Push Script
# This script helps you push to GitHub

echo "🚀 SD Copy - GitHub Push Helper"
echo "================================"
echo ""

# Check if we're in the right directory
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository"
    echo "Run this from: /Users/olefredrikschreuder/dev/SD-Copy"
    exit 1
fi

echo "✅ Git repository detected"
echo ""

# Check if remote already exists
if git remote | grep -q "origin"; then
    echo "ℹ️  Remote 'origin' already exists:"
    git remote get-url origin
    echo ""
    read -p "Do you want to update it? (y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        read -p "Enter GitHub repository URL: " REPO_URL
        git remote set-url origin "$REPO_URL"
        echo "✅ Remote updated"
    fi
else
    echo "No remote configured yet."
    echo ""
    echo "First, create a repository on GitHub.com:"
    echo "  1. Go to https://github.com/new"
    echo "  2. Repository name: SD-Copy"
    echo "  3. Don't initialize with README"
    echo "  4. Click 'Create repository'"
    echo ""
    read -p "Enter your GitHub username: " USERNAME

    if [ -z "$USERNAME" ]; then
        echo "❌ Username required"
        exit 1
    fi

    echo ""
    echo "Choose authentication method:"
    echo "  1) HTTPS (easier, requires token)"
    echo "  2) SSH (requires SSH key setup)"
    read -p "Choice (1 or 2): " AUTH_CHOICE

    if [ "$AUTH_CHOICE" = "1" ]; then
        REPO_URL="https://github.com/$USERNAME/SD-Copy.git"
    elif [ "$AUTH_CHOICE" = "2" ]; then
        REPO_URL="git@github.com:$USERNAME/SD-Copy.git"
    else
        echo "❌ Invalid choice"
        exit 1
    fi

    git remote add origin "$REPO_URL"
    echo "✅ Remote added: $REPO_URL"
fi

echo ""
echo "Ready to push!"
echo ""
read -p "Push to GitHub now? (y/n) " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "Pushing to GitHub..."
    git branch -M main
    git push -u origin main

    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ Successfully pushed to GitHub!"
        echo ""
        echo "View your repository at:"
        git remote get-url origin | sed 's/\.git$//' | sed 's/git@github.com:/https:\/\/github.com\//'
    else
        echo ""
        echo "❌ Push failed. Common issues:"
        echo ""
        echo "HTTPS Authentication:"
        echo "  - You need a Personal Access Token, not your password"
        echo "  - Get one at: https://github.com/settings/tokens"
        echo "  - Select scope: repo (all)"
        echo ""
        echo "SSH Authentication:"
        echo "  - Check if you have SSH key: ls ~/.ssh/id_*.pub"
        echo "  - Add to GitHub: https://github.com/settings/keys"
        echo ""
        echo "Try again after fixing authentication."
    fi
else
    echo ""
    echo "Cancelled. To push manually later:"
    echo "  git push -u origin main"
fi

echo ""
echo "Done!"

