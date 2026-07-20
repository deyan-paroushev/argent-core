#!/usr/bin/env bash
# apply-argent-core-shared-gold-infra.sh
#
# Adds docs/shared-gold-infrastructure-and-argent.md and realigns surrounding
# documentation.
#
# Why this matters now: the World Gold Council and BCG have announced "Gold as a
# Service", shared infrastructure connecting physical custody with digital gold
# product issuance, and are explicitly calling for innovators inside and outside
# the gold industry to contribute. Their published rationale includes gold
# becoming "deployable capital, enabling new use cases like pledging gold as
# collateral for borrowing". This document positions Argent as complementary
# rather than competing:
#
#   "Gold infrastructure proves the reserve.
#    Argent governs what the bank may issue against it."
#
# The document is labelled market and interoperability analysis and explicitly
# disclaims any partnership, endorsement, production integration, or current
# support for pooled or tokenised gold.
#
# All third-party claims verified against LBMA/WGC published sources:
#   - 100% of Good Delivery refiners onboarded to the GBI Database (start 2026)
#   - voluntary Country of Origin reporting launched 30 April 2026
#   - custodians onboarded and reporting aggregated vault holdings by Dec 2026
#   - bar-level reporting named as the intended next step
#   - ownership data remains with custodians (hence the gap Argent fills)
#
# DOCUMENTATION ONLY. Contracts, scripts and CI untouched.
# Run from the argent-core REPO ROOT. No commit.

set -euo pipefail
TARBALL="${1:-argent-core-shared-gold-infra.tar.gz}"

[ -d contracts ] && [ -d docs ] || { echo "ERROR: run from argent-core repo root. here: $(pwd)"; exit 1; }
[ -f "$TARBALL" ] || { echo "ERROR: tarball not found: $TARBALL"; exit 1; }

echo "== verifying tarball is documentation only =="
OFF="$(tar -tzf "$TARBALL" | grep -Ev '(^README\.md$|^POSITIONING_UPDATE\.md$|^docs/)' || true)"
if [ -n "$OFF" ]; then echo "  !! non-documentation paths:"; echo "$OFF" | sed 's/^/     /'; exit 1; fi
echo "   confirmed."; echo

echo "== extracting $TARBALL =="
tar -xzf "$TARBALL"; echo "   done."; echo

echo "== sanity =="
FAIL=0
F=docs/shared-gold-infrastructure-and-argent.md
[ -f "$F" ] || { echo "  !! missing $F"; FAIL=1; }
grep -q "does not claim a partnership, endorsement" "$F" 2>/dev/null || { echo "  !! disclaimer missing"; FAIL=1; }
grep -q "Gold infrastructure proves the reserve" "$F" 2>/dev/null || { echo "  !! positioning line missing"; FAIL=1; }
grep -q "shared-gold-infrastructure-and-argent" docs/DOCUMENT_STATUS_MATRIX.md || { echo "  !! matrix not updated"; FAIL=1; }
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
  git commit -m "Add shared gold infrastructure analysis: position Argent as complementary to GBI, Pooled Gold Interests and Gold as a Service; gold infrastructure proves the reserve, Argent governs what the bank may issue against it; contracts unchanged"
  git push origin main

EOF
