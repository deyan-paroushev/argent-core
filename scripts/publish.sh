#!/usr/bin/env bash
#
# publish.sh — push to a PUBLIC repository, safely.
#
# The problem with `git add . && git commit -m "quick update" && git push`:
#
#   1. `git add .` stages whatever is in the working tree. On a private repo
#      that is untidy. On a PUBLIC repo it is how a stray .env, a key, or a
#      private note becomes permanently indexed, cached by GitHub, and forked
#      by strangers. `git rm` afterwards does not remove it from history.
#
#   2. "quick update" tells a reader nothing. This repo is read by banks,
#      custodians, and accelerator reviewers who are deciding whether to trust
#      it. The commit log is part of the product.
#
#   3. Nothing checks that the documentation still agrees with the contract
#      before it goes out. Documentation that misstates the contract is worse
#      than none: it invites a reader to trust a claim, and rewards them for
#      checking.
#
# This script fixes all three. It refuses to push until it has shown you what
# it is about to publish and you have said yes.
#
# Usage:
#     ./scripts/publish.sh                    # interactive
#     ./scripts/publish.sh -m "docs: ..."     # supply the message up front
#     ./scripts/publish.sh --dry-run          # check everything, push nothing
#
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

BRANCH="${BRANCH:-main}"
REMOTE="${REMOTE:-origin}"
DRY_RUN=0
MESSAGE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    -m|--message) MESSAGE="$2"; shift 2 ;;
    -n|--dry-run) DRY_RUN=1; shift ;;
    -h|--help)    sed -n '2,30p' "$0" | sed 's/^# \?//'; exit 0 ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
done

bold()  { printf '\033[1m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }
red()   { printf '\033[31m%s\033[0m\n' "$*"; }
warn()  { printf '\033[33m%s\033[0m\n' "$*"; }
rule()  { printf '%s\n' "────────────────────────────────────────────────────────"; }

fatal() { red "STOPPED: $*"; exit 1; }

# ───────────────────────────────────────────────────────────────────────────
# 0. Sanity
# ───────────────────────────────────────────────────────────────────────────
git rev-parse --is-inside-work-tree >/dev/null 2>&1 || fatal "not a git repository"

CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$CURRENT_BRANCH" != "$BRANCH" ]]; then
  warn "You are on '$CURRENT_BRANCH', not '$BRANCH'."
  read -r -p "Push '$CURRENT_BRANCH' instead? [y/N] " a
  [[ "$a" =~ ^[Yy]$ ]] || fatal "aborted"
  BRANCH="$CURRENT_BRANCH"
fi

if [[ -z "$(git status --porcelain)" ]]; then
  green "Nothing to commit. Working tree is clean."
  exit 0
fi

# ───────────────────────────────────────────────────────────────────────────
# 1. Secret scan — BEFORE anything is staged
#
# .gitignore already covers .env, *.key, *.pem. This is the second lock: it
# catches a secret pasted into a file that IS tracked, which .gitignore cannot
# see. Cheap, and the failure it prevents is unrecoverable.
# ───────────────────────────────────────────────────────────────────────────
bold "1. Scanning for secrets"

SECRET_PATTERNS=(
  'S[A-Z0-9]{55}'                              # Stellar secret seed
  'BEGIN [A-Z ]*PRIVATE KEY'                   # PEM private key
  'sk-[A-Za-z0-9]{20,}'                        # API secret key
  'ghp_[A-Za-z0-9]{36}'                        # GitHub personal token
  'AKIA[0-9A-Z]{16}'                           # AWS access key
  '(password|passwd|secret|token|api_key)[[:space:]]*[:=][[:space:]]*["'\''][^"'\'']{8,}'
)

FOUND_SECRET=0
while IFS= read -r file; do
  [[ -f "$file" ]] || continue
  case "$file" in
    scripts/publish.sh|*.lock|*/target/*) continue ;;   # this script defines the patterns
  esac
  for pat in "${SECRET_PATTERNS[@]}"; do
    if grep -EIn "$pat" "$file" >/dev/null 2>&1; then
      red "  POSSIBLE SECRET in $file"
      grep -EIn "$pat" "$file" | head -2 | sed 's/^/      /'
      FOUND_SECRET=1
    fi
  done
done < <(git ls-files --cached --others --exclude-standard)

if [[ $FOUND_SECRET -eq 1 ]]; then
  echo
  red "A secret may be about to be published to a PUBLIC repository."
  red "Once pushed, it is in the history, in GitHub's cache, and in every fork."
  red "Rotating the credential is the only real remedy."
  echo
  read -r -p "Are these false positives? Type 'yes I checked' to continue: " a
  [[ "$a" == "yes I checked" ]] || fatal "aborted — good"
else
  green "  clean"
fi

# ───────────────────────────────────────────────────────────────────────────
# 2. Documentation must agree with the contract
# ───────────────────────────────────────────────────────────────────────────
bold "2. Checking documentation against the contract"

if [[ -f scripts/check_docs.py ]]; then
  if python3 scripts/check_docs.py >/tmp/docscheck.out 2>&1; then
    green "  $(tail -1 /tmp/docscheck.out)"
  else
    cat /tmp/docscheck.out
    echo
    fatal "documentation contradicts the contract. Fix it, or fix the docs. Do not publish it."
  fi
else
  warn "  scripts/check_docs.py not found — skipping"
fi

# ───────────────────────────────────────────────────────────────────────────
# 3. Contracts must still compile
# ───────────────────────────────────────────────────────────────────────────
bold "3. Checking the contracts build"

# Only run when Rust actually changed, and only if BUILD=1. A cargo check on a
# cold cache takes minutes, and a publish script that is slow is a publish
# script that gets bypassed — which defeats the whole point. CI compiles every
# push regardless; this is a local convenience, not the gate.
RUST_CHANGED=0
git status --porcelain contracts/ | grep -qE '\.rs$|Cargo\.toml$' && RUST_CHANGED=1

if [[ $RUST_CHANGED -eq 0 ]]; then
  green "  no Rust changes — skipping"
elif [[ "${BUILD:-0}" != "1" ]]; then
  warn "  Rust changed but BUILD=1 not set — skipping local compile (CI will catch it)"
  warn "  To check locally:  BUILD=1 ./scripts/publish.sh"
elif command -v cargo >/dev/null 2>&1; then
  if timeout 600 cargo check --manifest-path contracts/Cargo.toml --quiet 2>/tmp/cargo.out; then
    green "  contracts compile"
  else
    tail -20 /tmp/cargo.out
    fatal "contracts do not compile. Do not publish a broken build."
  fi
else
  warn "  cargo not available — skipping"
fi

# ───────────────────────────────────────────────────────────────────────────
# 4. Show exactly what is about to be published
#
# This is the step `git add .` skips, and it is the one that matters. You are
# publishing to the world. Look at it first.
# ───────────────────────────────────────────────────────────────────────────
bold "4. What will be published"
rule
git add -A
git -c color.status=always status --short | sed 's/^/  /'
rule

ADDED=$(git diff --cached --numstat | awk '{a+=$1} END {print a+0}')
DELETED=$(git diff --cached --numstat | awk '{d+=$2} END {print d+0}')
FILES=$(git diff --cached --name-only | wc -l | tr -d ' ')
echo "  $FILES file(s), +$ADDED / -$DELETED lines"
echo

NEW_FILES=$(git diff --cached --diff-filter=A --name-only)
if [[ -n "$NEW_FILES" ]]; then
  warn "  NEW files (never published before — check each one):"
  echo "$NEW_FILES" | sed 's/^/    + /'
  echo
fi

read -r -p "Review the full diff before continuing? [y/N] " a
[[ "$a" =~ ^[Yy]$ ]] && git diff --cached

# ───────────────────────────────────────────────────────────────────────────
# 5. A commit message a stranger can read
# ───────────────────────────────────────────────────────────────────────────
bold "5. Commit message"

if [[ -z "$MESSAGE" ]]; then
  echo "  Conventional prefixes: feat, fix, docs, refactor, test, chore, ci"
  echo "  Good:  docs: add design-partner briefs and contract conformance check"
  echo "  Bad:   quick update"
  echo
  read -r -p "  message: " MESSAGE
fi

[[ -n "$MESSAGE" ]] || fatal "a commit message is required"

if [[ "$MESSAGE" =~ ^(quick|update|wip|stuff|misc|fix|changes)$ ]]; then
  fatal "'$MESSAGE' tells a reader nothing. This log is read by banks and reviewers."
fi

if [[ ! "$MESSAGE" =~ ^(feat|fix|docs|refactor|test|chore|ci|build|perf)(\(.+\))?!?:\  ]]; then
  warn "  Not a conventional-commit prefix. Suggested: 'docs: $MESSAGE'"
  read -r -p "  Use it anyway? [y/N] " a
  [[ "$a" =~ ^[Yy]$ ]] || fatal "aborted"
fi

# ───────────────────────────────────────────────────────────────────────────
# 6. Commit and push
# ───────────────────────────────────────────────────────────────────────────
if [[ $DRY_RUN -eq 1 ]]; then
  echo
  green "DRY RUN — all checks passed. Nothing was committed or pushed."
  git reset >/dev/null
  exit 0
fi

bold "6. Publishing"
git commit -q -m "$MESSAGE"
green "  committed: $(git log -1 --pretty='%h %s')"

echo
warn "  About to push to $REMOTE/$BRANCH — a PUBLIC repository."
read -r -p "  Push? [y/N] " a
if [[ ! "$a" =~ ^[Yy]$ ]]; then
  warn "  Not pushed. The commit is local; 'git reset --soft HEAD~1' to undo it."
  exit 0
fi

git push "$REMOTE" "$BRANCH"
echo
green "Published to $REMOTE/$BRANCH."

URL=$(git remote get-url "$REMOTE" 2>/dev/null | sed -E 's#git@github.com:#https://github.com/#; s#\.git$##')
[[ -n "$URL" ]] && echo "  $URL"
