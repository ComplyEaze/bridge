# Lab helper (2377-L Part B, not product code): sends one sitting-2 request by its label and file name, taking the
# sha256 from the lines the kit printed (evidence/d1-fixed.txt and evidence/d3-top3.txt), never typed. Extra arguments
# (--bound N, --timing) go to send2377b.py.
# Usage: bash send2.sh <label> <request name without .utf16le.xml> [--bound N] [--timing]
cd <LAB_DIR> || exit 9
h=$(cat evidence/d1-fixed.txt evidence/d3-top3.txt 2>/dev/null | awk -v f="$2.utf16le.xml" '$1==f {print $3}')
[ -n "$h" ] || { echo "no printed sha256 for $2"; exit 8; }
l="$1"; r="$2"; shift 2
python -I send2377b.py "$l" "req2/$r.utf16le.xml" "$h" "$@"
