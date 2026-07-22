#!/usr/bin/env bash
# apply-argent-core-capacity-privacy.sh
#
# Adds two canonical design documents to the public argent-core repository and
# realigns the surrounding docs:
#
#   docs/capacity-reservation-and-deliverability.md
#     Available vs issuable capacity. An asset can be valuable and uncommitted
#     and still not be usable for a specific obligation at a specific time.
#     Reservation lifecycle, concurrency, preflight decisions, external finality.
#
#   docs/selective-disclosure-and-institutional-privacy.md
#     Sharing enough to authorize, verify and supervise without exposing the
#     full reserve, facility or transaction book. Data classes, role-specific
#     visibility, evidence protection, selective-disclosure maturity path.
#
# Both are labelled "not yet the implemented contract surface" and appear in the
# DOCUMENT_STATUS_MATRIX as "Canonical direction + target profile".
#
# DOCUMENTATION ONLY. Contracts, scripts and CI are untouched.
# Run from the argent-core REPO ROOT. No commit.

set -euo pipefail
TARBALL="${1:-argent-core-capacity-privacy.tar.gz}"

[ -d contracts ] && [ -d docs ] || { echo "ERROR: run from argent-core repo root. here: $(pwd)"; exit 1; }
[ -f "$TARBALL" ] || { echo "ERROR: tarball not found: $TARBALL"; exit 1; }

echo "== verifying tarball is documentation only =="
OFF="$(tar -tzf "$TARBALL" | grep -Ev '(^README\.md$|^POSITIONING_UPDATE\.md$|^docs/)' || true)"
if [ -n "$OFF" ]; then echo "  !! non-documentation paths:"; echo "$OFF" | sed 's/^/     /'; exit 1; fi
echo "   confirmed: only README.md, POSITIONING_UPDATE.md and docs/"
echo

echo "== extracting $TARBALL =="
tar -xzf "$TARBALL"; echo "   done."; echo

echo "== sanity =="
FAIL=0
for f in docs/capacity-reservation-and-deliverability.md docs/selective-disclosure-and-institutional-privacy.md; do
  [ -f "$f" ] || { echo "  !! missing $f"; FAIL=1; }
  grep -q "not yet the implemented contract surface" "$f" 2>/dev/null || { echo "  !! $f missing status label"; FAIL=1; }
done
grep -q "capacity-reservation-and-deliverability" docs/DOCUMENT_STATUS_MATRIX.md || { echo "  !! matrix not updated"; FAIL=1; }
grep -q "secured-credit reference branch" README.md || { echo "  !! README honesty statement missing"; FAIL=1; }
if grep -rqi "Stellar Community Fund\|Integration Track\|Hub71\|SFIIP" --include="*.md" . ; then
  echo "  !! fund-specific language found"; FAIL=1
fi
[ "$FAIL" -eq 0 ] && echo "   clean." || { echo "   FAILED. Stop."; exit 1; }
echo

echo "== documentation-to-contract checker =="
python3 scripts/check_docs.py </dev/null
echo

echo "== internal link check =="
python3 - </dev/null <<'PY'
import re, os, glob
bad=[]; total=0
for md in glob.glob('**/*.md', recursive=True):
    base=os.path.dirname(md)
    try: text=open(md,encoding='utf-8',errors='ignore').read()
    except Exception: continue
    for m in re.finditer(r'\[([^\]]+)\]\(([^)]+)\)', text):
        link=m.group(2).split('#')[0].strip()
        if not link or link.startswith(('http','mailto:')): continue
        total+=1
        if not os.path.exists(os.path.normpath(os.path.join(base,link))): bad.append((md,link))
print(f"   checked {total} internal links")
if bad:
    print(f"   BROKEN: {len(bad)}")
    for f,l in bad[:10]: print(f"     {f} -> {l}")
    raise SystemExit(1)
print("   all resolve.")
PY
echo

echo "== confirm git sees ONLY documentation changes =="
if command -v git >/dev/null 2>&1 && git rev-parse --git-dir >/dev/null 2>&1; then
  NON="$(git status --porcelain -- contracts scripts .github 2>/dev/null || true)"
  if [ -n "$NON" ]; then echo "  !! changes outside docs:"; echo "$NON" | sed 's/^/     /'; exit 1; fi
  echo "   confirmed: contracts/, scripts/ and .github/ unmodified."
else
  echo "   (not a git repo, skipping)"
fi
echo

echo "== ALL GREEN. Review, then: =="
cat <<'EOF'

  git add README.md POSITIONING_UPDATE.md docs/
  git commit -m "Add capacity reservation/deliverability and selective-disclosure privacy design specifications; realign surrounding documentation; contracts unchanged"
  git push origin main

EOF
