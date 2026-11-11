# Lab helper (row 2480-SC, from row 2480-SK's er.sh, with C-u-, Vm-u- and Vw-u- added as ROW.md asks and the run lead's word (a) covers): one send of step 7 or 8 under exception (a). Sends by send.sh (with the window captured at once after it, <label>-after; send.sh reads an answer with bytes by nm.py, cv.py or wx.py). If run.py passes (bytes came back), this exits 3. If run.py stops, the exception is tested: the headers file must hold a status line of 200 and CONTENT-LENGTH: 0, and the answer file must be 0 bytes or absent (wx.py reads an absent file as no bytes); then GET /status once (must answer 200), then wx.py alone on the empty answer, whatever its form (evidence/wx-<label>.txt), and this exits 0 with evidence/empty-<label>.txt holding EXCEPTION HOLDS. Anything else exits 7 (a stop). Usage: bash er.sh <label> <file>
cd <LAB_DIR> || exit 9
L=$1; F=$2; A=a-$L-$F; H=$A.headers; E=evidence/empty-$L.txt
case "$F" in L-lo-*) O=nm;; C-v-*|C-u-*) O=cv;; Vm-v-*|Vm-u-*|Vw-v-*|Vw-u-*) O=wx;; *) echo "$F is not an L-lo-, C-, Vm- or Vw- request of steps 7 and 8"; exit 9;; esac
SHOT=$L-after bash send.sh $L $F; rc=$?
[ $rc = 0 ] && { echo "$L: answered with bytes (evidence/$O-$L.txt)"; exit 3; }
[ $rc = 7 ] || { echo "$L: send.sh exit $rc, nothing sent or a check failed"; exit 7; }
{
  echo "$L $F: run.py stopped (log-$L.txt: $(tail -1 log-$L.txt | cut -f4-5))"
  [ -e "$A" ] && echo "answer file $(stat -c %s "$A") bytes" || echo "answer file absent"
  [ -e "$H" ] && { echo "headers file:"; tr -d '\r' < "$H" | sed 's/^/  /'; } || echo "headers file absent"
} > $E
st=$(tr -d '\r' < "$H" 2>/dev/null | grep -E '^HTTP/' | tail -1 | awk '{print $2}')
cl=$(tr -d '\r' < "$H" 2>/dev/null | grep -i '^content-length:' | tail -1 | awk '{print $2}')
sz=$(stat -c %s "$A" 2>/dev/null || echo 0)
if [ "$st" = 200 ] && [ "$cl" = 0 ] && [ "$sz" = 0 ]; then
  echo "EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes" >> $E
else
  echo "EXCEPTION DOES NOT HOLD (status $st, CONTENT-LENGTH $cl, bytes $sz): stop" >> $E; cat $E; exit 7
fi
s=$(curl.exe -sS -w 'status=%{http_code}, bytes=%{size_download}' -o /dev/null http://127.0.0.1:9001/status); c=$?
echo "$(date -u +%FT%T.%3NZ) GET /status after $L: curl $c \"$s\"" >> $E
echo "$(date -u +%FT%T.%3NZ) $L GET /status curl $c $s" >> evidence/sends-utc.txt
[ $c = 0 ] && case "$s" in status=200,*) ;; *) false;; esac || { echo "GET /status not 200: stop" >> $E; cat $E; exit 7; }
python -I -X utf8 wx.py $A > evidence/wx-$L.txt 2>&1; echo "exit $?" >> evidence/wx-$L.txt
cat $E evidence/wx-$L.txt; exit 0
