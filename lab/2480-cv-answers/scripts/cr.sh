# Lab helper (row 2480-CV, from row 2480-NX's lr.sh): one C-u- or C-v- census send, under the run lead's empty-answer exception (a) at steps 3, 6 and 9. Sends by send.sh (with the window captured at once after it, <label>-after). If run.py passes (bytes came back), cv.py has run on it in send.sh and this exits 3. If run.py stops, the exception is tested: the headers file must hold a status line of 200 and CONTENT-LENGTH: 0, and the answer file must be 0 bytes; then GET /status once (must answer 200), then cv.py on the empty answer. Anything else exits 7 (a stop). Usage: bash cr.sh <label> <file>
cd <LAB_DIR> || exit 9
L=$1; F=$2; A=a-$L-$F; H=$A.headers; E=evidence/empty-$L.txt
case "$F" in C-u-*|C-v-*) ;; *) echo "$F is not a C-u- or C-v- request"; exit 9;; esac
SHOT=$L-after bash send.sh $L $F; rc=$?
[ $rc = 0 ] && { echo "$L: answered with bytes (evidence/cv-$L.txt)"; exit 3; }
[ $rc = 7 ] || { echo "$L: send.sh exit $rc, nothing sent or a check failed"; exit 7; }
{
  echo "$L $F: run.py stopped (log-$L.txt: $(tail -1 log-$L.txt | cut -f4-5))"
  [ -e "$A" ] && echo "answer file $(stat -c %s "$A") bytes" || echo "answer file absent"
  [ -e "$H" ] && { echo "headers file:"; tr -d '\r' < "$H" | sed 's/^/  /'; } || echo "headers file absent"
} > $E
st=$(tr -d '\r' < "$H" 2>/dev/null | grep -E '^HTTP/' | tail -1 | awk '{print $2}')
cl=$(tr -d '\r' < "$H" 2>/dev/null | grep -i '^content-length:' | tail -1 | awk '{print $2}')
sz=$(stat -c %s "$A" 2>/dev/null || echo none)
if [ "$st" = 200 ] && [ "$cl" = 0 ] && [ "$sz" = 0 ]; then
  echo "EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes" >> $E
else
  echo "EXCEPTION DOES NOT HOLD (status $st, CONTENT-LENGTH $cl, bytes $sz): stop" >> $E; cat $E; exit 7
fi
s=$(curl.exe -sS -w 'status=%{http_code}, bytes=%{size_download}' -o /dev/null http://127.0.0.1:9001/status); c=$?
echo "$(date -u +%FT%T.%3NZ) GET /status after $L: curl $c \"$s\"" >> $E
echo "$(date -u +%FT%T.%3NZ) $L GET /status curl $c $s" >> evidence/sends-utc.txt
[ $c = 0 ] && case "$s" in status=200,*) ;; *) false;; esac || { echo "GET /status not 200: stop" >> $E; cat $E; exit 7; }
python -I -X utf8 cv.py $A > evidence/cv-$L.txt 2>&1; echo "exit $?" >> evidence/cv-$L.txt
cat $E evidence/cv-$L.txt
