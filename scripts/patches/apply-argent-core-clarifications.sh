#!/usr/bin/env bash
#
# apply-argent-core-clarifications.sh
#
# Applies three documentation clarifications to argent-core:
#
#   1. Synthetic-data-only is a governance invariant, not a contract control.
#   2. The pilot nullifier domain is custodian-owned and custodian-keyed.
#   3. Production uses the separate batch-anchor contract, not a retrofit of
#      the transparent reference contracts.
#
# No contract source is modified.
#
# Run from the repository root (the directory containing contracts/ and docs/).

set -euo pipefail

if [[ ! -d contracts ]] || [[ ! -d docs ]]; then
  echo "ERROR: run this from the argent-core repository root." >&2
  echo "Expected to find contracts/ and docs/ here." >&2
  exit 1
fi

SPEC="docs/confidential-control-and-public-integrity.md"
ROADMAP="docs/product-roadmap.md"

if [[ ! -f "$SPEC" ]]; then
  echo "ERROR: $SPEC not found." >&2
  echo "This patch applies on top of the confidential-integrity update." >&2
  exit 1
fi

# Locate the patch payload. The script may be run either from inside the
# extracted argent-core-patch/ folder, or copied to the repo root with the
# extracted folder still present.
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if [[ -f "$SCRIPT_DIR/docs/confidential-control-and-public-integrity.md" ]] \
   && [[ "$SCRIPT_DIR" != "$(pwd)" ]]; then
  SRC="$SCRIPT_DIR/docs"
elif [[ -f "argent-core-patch/docs/confidential-control-and-public-integrity.md" ]]; then
  SRC="argent-core-patch/docs"
else
  echo "ERROR: cannot find the patch payload." >&2
  echo "Extract the tarball first:  tar -xzf argent-core-clarifications.tar.gz" >&2
  echo "Then run:                   ./argent-core-patch/apply-argent-core-clarifications.sh" >&2
  exit 1
fi

echo "==> backing up files being changed"
cp "$SPEC" "${SPEC}.bak"
cp "$ROADMAP" "${ROADMAP}.bak"

echo "==> applying documentation changes from $SRC"
cp -f "$SRC/confidential-control-and-public-integrity.md" "$SPEC"
cp -f "$SRC/product-roadmap.md" "$ROADMAP"

echo "==> verifying contract source untouched"
if git rev-parse --git-dir >/dev/null 2>&1; then
  if ! git diff --quiet -- contracts/ 2>/dev/null; then
    echo "WARNING: contracts/ shows changes. This patch should not touch them." >&2
  else
    echo "    contracts/ clean"
  fi
else
  echo "    not a git repository; skipping contract diff check"
fi

echo "==> running repository documentation check"
python3 scripts/check_docs.py

echo "==> checking internal links"
python3 - <<'PYEOF'
import re, os, glob, sys
bad = []
for f in glob.glob('**/*.md', recursive=True):
    if f.startswith('argent-core-patch/'):
        continue
    d = os.path.dirname(f)
    for m in re.finditer(r'\]\(([^)#]+\.md)(#[^)]*)?\)', open(f).read()):
        t = m.group(1)
        if t.startswith('http'):
            continue
        p = os.path.normpath(os.path.join(d, t))
        if not os.path.exists(p):
            bad.append((f, t))
if bad:
    print("BROKEN LINKS: %d" % len(bad))
    for f, t in bad:
        print("   %s -> %s" % (f, t))
    sys.exit(1)
print("internal links OK")
PYEOF

echo
echo "==> done"
echo
echo "Review the diff:"
echo "   git diff $SPEC"
echo "   git diff $ROADMAP"
echo
echo "Remove backups once satisfied:"
echo "   rm ${SPEC}.bak ${ROADMAP}.bak"
