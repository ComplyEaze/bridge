# Lab helper (row 2480-SK, from row 2480-MW's notes): which of Tally's seven side files a data-folder listing holds. Usage: bash side.sh <listing>
cd <LAB_DIR> || exit 9
p=(); a=()
for s in CmpSchMetaData.1800 Index.1800 TCMPSCHACCESS.TSF TCMPSCHMESSAGE.TSF TCMPSCHSTATE.TSF TCMPSCHUPDATE.TSF TINTMSG.TSF; do grep -q " $s\$" "$1" && p+=($s) || a+=($s); done
echo "$(tail -n +4 "$1" | wc -l) entries; side files present ${#p[@]} of 7 (${p[*]}); absent ${a[*]:-none}"
