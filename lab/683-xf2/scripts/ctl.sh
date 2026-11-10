# Lab helper (row 683-XF second sitting): step 2's calls on one tb3a answer, each recorded whole in evidence/ with its exit. Usage: bash ctl.sh <label>
cd <LAB_DIR> || exit 9
L=$1; T=T-tb3-20250401-20250401.xml; A=a-$L-$T
r() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; e=$?; echo "exit $e" >> evidence/$o.txt; echo "$o: $(grep -E '^(SETS|CONTENT|ROW ORDER|ELEMENT ORDER [A-Z]|NOT BYTE|BYTE)' evidence/$o.txt | tr '\n' ' ')exit $e"; }
r sets-twtb3-$L tb.py --sets a-twtb3-$T $A
r rows-twtb3-$L rows.py a-twtb3-$T $A
r cmp-twtb3-$L cmpinfo.py a-twtb3-$T $A
r cmp-xftb3a-$L cmpinfo.py a-xftb3a-$T $A
