# Lab helper (2377-L Part B, not product code): sends one sitting-1 request by its label and file name, taking the
# sha256 from the lines the kit printed (evidence/b1-fixed-snapshot.txt and evidence/b1-writes.txt), never typed.
# Usage: bash send1.sh <label> <request name without .utf16le.xml>
cd <LAB_DIR> || exit 9
h=$(cat evidence/b1-fixed-snapshot.txt evidence/b1-writes.txt | awk -v f="$2.utf16le.xml" '$1==f {print $3}')
[ -n "$h" ] || { echo "no printed sha256 for $2"; exit 8; }
python -I send2377b.py "$1" "req1/$2.utf16le.xml" "$h"
