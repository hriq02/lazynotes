#!/bin/bash
cd "$(dirname "$0")" # Change to the directory where the script is located
read -p "Enter your commit message: " commit_message
git add .
git commit -m "$commit_message"
git push origin HEAD