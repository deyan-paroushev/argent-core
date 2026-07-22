#!/usr/bin/env bash
# apply-argent-core-whitepaper-aligned.sh
#
# Sharpens the shared-gold-infrastructure analysis against the FULL World Gold
# Council / BCG white paper (previously written from the lite paper and press
# release). No new documents; 18 existing docs refined.
#
# The central change is a precise assurance boundary, taken from the WGC's own
# stated limitation that "the assurance would only relate to the physical gold
# and its legal entitlements":
#
#   A reserve platform may establish, within its scope:
#     physical backing, custody, ownership/entitlement, reconciliation, redemption
#
#   Argent must separately establish:
#     bank eligibility, security interest and control, facility and sublimit
#     treatment, purpose-bound reservation, bank-obligation state, claim,
#     reimbursement, release or enforcement state
#
# Also adds an important caution: upstream asset-layer fungibility must never be
# read as permission to transfer, reuse or re-pledge an Argent reservation.
# Facility capacity stays purpose-bound and non-transferable.
#
# And notes honestly that the white paper's illustrated collateral use case is a
# funded loan against locked gold, which validates the implemented secured-credit
# branch while leaving guarantees and documentary credits as a distinct layer.
#
# All third-party claims verified verbatim against gold.org published pages.
# The document retains its disclaimer of any partnership, endorsement or
# production integration.
#
# DOCUMENTATION ONLY. Contracts, scripts and CI untouched.
# Run from the argent-core REPO ROOT. No commit.

set -euo pipefail
TARBALL="${1:-argent-core-whitepaper-aligned.tar.gz}"

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
grep -q "does not claim a partnership, endorsement" "$F" 2>/dev/null || { echo "  !! disclaimer lost"; FAIL=1; }
grep -q "within its defined scope" "$F" 2>/dev/null || { echo "  !! scoped assurance framing missing"; FAIL=1; }
grep -q "Upstream fungibility must never be interpreted" "$F" 2>/dev/null || { echo "  !! fungibility caution missing"; FAIL=1; }
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
  git commit -m "Align shared-gold-infrastructure analysis to the full WGC/BCG white paper: scope-limited reserve assurance, precise bank-facility boundary, non-transferable reservations under upstream fungibility; contracts unchanged"
  git push origin main

EOF
