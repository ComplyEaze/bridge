# Lab helper (row 2480-SC, from row 2480-SK's r.sh): step 7, Part A, only after step 6 passed. For each id given (comma-separated, a part of qa,qb,ua,ub, in that order; n = 6, 7, 8, 9), each send once, each by er.sh (the window captured at once after each send and checked against evidence/gateway-refs.txt; exception (a) tested on an empty answer), each answer read by its reader before the next request: the ##LedgerName form c<n>a, c<n>b (C-v-<id>.xml, cv.py), q<n>a, q<n>b (Vm-v-<id>.xml, wx.py), w<n>a, w<n>b (Vw-v-<id>.xml, wx.py); for qa and ua also master's CHILDOF form cu<n>a, cu<n>b (C-u-<id>.xml), qu<n>a, qu<n>b (Vm-u-<id>.xml), wu<n>a, wu<n>b (Vw-u-<id>.xml); the pair's --same by the form's reader (wx.py --same when either answer is empty). Then, before the next id, each one line, recorded whole: cv.py --own, wx.py --match movement and window, and sk.py --judge census, movement and window, on the ##LedgerName answers; for qa and ua the same on the CHILDOF answers, and the form against the form (cv.py --same, wx.py --same, wx.py --same). For an empty C answer no cv.py line and no sk.py --judge census line is run (its file says JUDGE <id> census: EMPTY ANSWER, from the wx.py line); for an empty V answer no wx.py --match line. A CHILDOF JUDGE line other than OWN YES, SIBLING NO, OTHERS 0, or an empty CHILDOF answer, prints a CHILDOF FORM: line and appends it to evidence/childof-flags.txt; the row goes on. A reader's exit 3 is a result; an er.sh exit other than 0 or 3 (7: a stop), a capture none of the list (6) or a reader's exit 1 (its stop) ends it with nothing more sent. Usage: bash a.sh <ids>
cd <LAB_DIR> || exit 9
IDS=$1; [ -n "$IDS" ] || { echo "usage: bash a.sh <ids>"; exit 9; }
INV=pl,ka,qa,qb,ua,ub; CB=a-s2-C-book.xml; CA=a-s6ka-C-book.xml; VB=a-k8-Vb-book.xml
rec() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; r=$?; echo "exit $r" >> evidence/$o.txt; echo "== $o: $(grep -v '^exit' evidence/$o.txt | tail -1) exit $r"; [ $r = 1 ] && { echo "$o: a stop of $1, step 7, id $id"; exit 1; }; return 0; }
empty() { [ -e evidence/empty-$1.txt ] && grep -q '^EXCEPTION HOLDS' evidence/empty-$1.txt; }
for x in qa:6 qb:7 ua:8 ub:9; do
  id=${x%%:*}; n=${x#*:}; case ",$IDS," in *",$id,"*) ;; *) continue;; esac
  FORMS="c:C-v-$id.xml:cv q:Vm-v-$id.xml:wx w:Vw-v-$id.xml:wx"
  case $id in qa|ua) FORMS="$FORMS cu:C-u-$id.xml:cv qu:Vm-u-$id.xml:wx wu:Vw-u-$id.xml:wx";; esac
  for p in $FORMS; do
    k=${p%%:*}; rest=${p#*:}; F=${rest%%:*}; O=${rest#*:}
    for s in a b; do
      L=$k$n$s
      bash er.sh $L $F > evidence/er-$L.out 2>&1; rc=$?
      if [ $rc = 0 ]; then R=wx; else R=$O; fi
      echo "== er.sh $L $F exit $rc: $(grep -ho 'ANSWER: .*\|EMPTY.*' evidence/$R-$L.txt 2>/dev/null | head -1)"
      [ $rc = 0 ] || [ $rc = 3 ] || { echo "$L: er.sh exit $rc: stop at step 7, id $id"; exit 7; }
      [ "$(tail -n 1 evidence/$R-$L.txt)" = "exit 1" ] && { echo "$L: a stop of its reader, step 7, id $id"; exit 1; }
      h=$(sha256sum shots/$L-after.png | cut -c1-64)
      grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
    done
    if empty ${k}${n}a || empty ${k}${n}b; then rec same-$k$n wx.py --same a-$k${n}a-$F a-$k${n}b-$F; else rec same-$k$n $O.py --same a-$k${n}a-$F a-$k${n}b-$F; fi
  done
  for f in "c::C-v" "u:u:C-u"; do
    t=${f%%:*}; rest=${f#*:}; u=${rest%%:*}; CF=${rest#*:}
    [ $t = u ] && case $id in qa|ua) ;; *) continue;; esac
    if empty c$u${n}a; then
      echo "not run: c$u${n}a is empty (evidence/wx-c$u${n}a.txt); JUDGE $id census: EMPTY ANSWER" | tee evidence/own-$u$n.txt evidence/judge-c$u$n.txt
    else
      rec own-$u$n cv.py --own $id a-c$u${n}a-$CF-$id.xml $CB $CA $INV
      rec judge-c$u$n sk.py --judge $id census a-c$u${n}a-$CF-$id.xml $VB
    fi
    for v in "q:movement:Vm" "w:window:Vw"; do
      l=${v%%:*}; rest=${v#*:}; fm=${rest%%:*}; pf=${rest#*:}; [ $t = u ] && VF=$pf-u-$id.xml || VF=$pf-v-$id.xml
      if empty $l$u${n}a; then echo "not run: $l$u${n}a is empty (evidence/wx-$l$u${n}a.txt)" > evidence/match-$l$u$n.txt; else rec match-$l$u$n wx.py --match $id $fm a-$l$u${n}a-$VF $VB; fi
      rec judge-$l$u$n sk.py --judge $id $fm a-$l$u${n}a-$VF $VB
    done
  done
  case $id in qa|ua)
    if empty c${n}a || empty cu${n}a; then rec ff-c$n wx.py --same a-c${n}a-C-v-$id.xml a-cu${n}a-C-u-$id.xml; else rec ff-c$n cv.py --same a-c${n}a-C-v-$id.xml a-cu${n}a-C-u-$id.xml; fi
    rec ff-q$n wx.py --same a-q${n}a-Vm-v-$id.xml a-qu${n}a-Vm-u-$id.xml
    rec ff-w$n wx.py --same a-w${n}a-Vw-v-$id.xml a-wu${n}a-Vw-u-$id.xml
    for g in census:cu movement:qu window:wu; do
      fm=${g%%:*}; l=${g#*:}; j=$(grep -v '^exit' evidence/judge-$l$n.txt | tail -1)
      case "$j" in *": OWN YES, SIBLING NO, OTHERS 0") ;; *) echo "CHILDOF FORM: $id $fm: $j" | tee -a evidence/childof-flags.txt;; esac
    done;;
  esac
done
exit 0
