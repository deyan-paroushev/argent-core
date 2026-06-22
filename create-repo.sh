#!/usr/bin/env bash
# Create the Argent Core open-source repo and push it to GitHub.
#
# Prerequre: the GitHub CLI (`gh`) authenticated, OR do the manual steps at the bottom.
# Run this from INSIDE the extracted argent-core/ directory.

set -euo pipefail

REPO_NAME="argent-core"
DESCRIPTION="Asset-agnostic collateral-control contracts for Soroban — the open-source core of Argent."
VISIBILITY="public"   # change to "private" if you want to stage it first

if [ ! -f README.md ] || [ ! -d contracts ]; then
  echo "ERROR: run this from inside the extracted argent-core/ directory (README.md + contracts/ must be here)."
  exit 1
fi

echo "==> Initializing git repo ..."
git init -q
git add .
git commit -q -m "Initial commit: asset-agnostic Soroban collateral-control contracts + docs (Apache-2.0)"
git branch -M main

if command -v gh >/dev/null 2>&1; then
  echo "==> Creating GitHub repo via gh and pushing ..."
  gh repo create "$REPO_NAME" --"$VISIBILITY" --source=. --description "$DESCRIPTION" --push
  echo "==> Done. Repo URL:"
  gh repo view --json url -q .url
else
  echo "==> gh CLI not found. Do the manual steps:"
  echo "    1. Create an empty repo named '$REPO_NAME' on github.com (no README, no license — we have them)."
  echo "    2. Then run:"
  echo "         git remote add origin https://github.com/<your-username>/$REPO_NAME.git"
  echo "         git push -u origin main"
fi

echo
echo "After the repo is live, use its URL in the SCF application:"
echo "  - 'GitHub URL' field (the open-source core)"
echo "  - reference it in the Technical Architecture Document field via docs/argent-architecture.md (raw or rendered link)"
