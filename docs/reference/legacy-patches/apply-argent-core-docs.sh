#!/usr/bin/env bash
# apply-argent-core-docs.sh
#
# Aligns the PUBLIC argent-core repository documentation to the mature positioning:
# gold-backed obligation infrastructure ("One reserve. Many obligations. One
# authoritative capacity state."), written fund-neutral so the repo can be shared
# with any bank, custodian, investor, accelerator, or reviewer.
#
# DOCUMENTATION ONLY. Contracts, scripts, and CI are untouched (verified
# byte-for-byte identical to the current core).
#
# Adds four documents:
#   POSITIONING_UPDATE.md                  - records the strategic transition
#   docs/reserve-obligation-infrastructure.md - canonical product direction
#   docs/obligation-facility-profile.md    - target facility model + mapping to contracts
#   docs/DOCUMENT_STATUS_MATRIX.md         - which docs are shipped code vs direction vs research
#
# Rewrites/realigns ~28 existing documents (README, protocol, architecture,
# roadmap, risk, custody, evidence, integration, deployment, ...).
#
# Run from the argent-core REPO ROOT. Extracts, verifies links + doc/contract
# checker, stops on failure. No commit.

set -euo pipefail
TARBALL="${1:-argent-core-docs-update.tar.gz}"

[ -d "contracts" ] && [ -d "docs" ] || { echo "ERROR: run from argent-core repo root (needs contracts/ and docs/). here: $(pwd)"; exit 1; }
[ -f "$TARBALL" ] || { echo "ERROR: tarball not found: $TARBALL"; exit 1; }

echo "== snapshot contract hashes BEFORE (to prove docs-only) =="
BEFORE=$(find contracts scripts -type f 2>/dev/null | sort | xargs sha256sum 2>/dev/null | sha256sum | cut -d' ' -f1)
echo "   $BEFORE"
echo

echo "== extracting $TARBALL =="
tar -xzf "$TARBALL"; echo "   done."; echo

echo "== verify contracts/scripts UNCHANGED =="
AFTER=$(find contracts scripts -type f 2>/dev/null | sort | xargs sha256sum 2>/dev/null | sha256sum | cut -d' ' -f1)
if [ "$BEFORE" = "$AFTER" ]; then echo "   confirmed: no code or script changes."; else
  echo "   !! contracts or scripts CHANGED - this should be docs-only. Stop and inspect."; exit 1; fi
echo

echo "== sanity: new canonical documents present =="
FAIL=0
for f in POSITIONING_UPDATE.md docs/reserve-obligation-infrastructure.md docs/obligation-facility-profile.md docs/DOCUMENT_STATUS_MATRIX.md; do
  [ -f "$f" ] || { echo "  !! missing $f"; FAIL=1; }
done
grep -q "One authoritative capacity state" README.md || { echo "  !! README not repositioned"; FAIL=1; }
grep -q "secured-credit reference branch" README.md   || { echo "  !! README missing implemented-vs-target honesty"; FAIL=1; }
# fund neutrality:
grep -rqi "SCF\|Stellar Community Fund\|Integration Track\|Hub71\|SFIIP" --include="*.md" . && { echo "  !! fund-specific language found"; FAIL=1; }
[ "$FAIL" -eq 0 ] && echo "   clean." || { echo "   FAILED. Stop."; exit 1; }
echo

echo "== documentation-to-contract checker =="
python3 scripts/check_docs.py
echo

echo "== internal link check =="
python3 - <<'PY'
import re, os, glob
bad=[]; total=0
for md in glob.glob('**/*.md', recursive=True):
    base=os.path.dirname(md)
    for m in re.finditer(r'\[([^\]]+)\]\(([^)]+)\)', open(md,encoding='utf-8',errors='ignore').read()):
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

echo "== ALL GREEN. Review, then: =="
cat <<'EOF'

  git add README.md POSITIONING_UPDATE.md docs/
  git commit -m "Align core documentation to gold-backed obligation infrastructure: canonical direction, target facility profile, and a document status matrix separating shipped contracts from product direction; fund-neutral throughout; contracts unchanged"
  git push origin main

EOF
