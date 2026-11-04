# Lab helper (1029-R, not product code): step 3a and 3b of one batch. build <b>: checks the batch file's sha256 against
# the sheet's SHA256.txt, sends build_import_xml with the file's array unchanged, prints batch id, count and warnings.
# post <b> <batch_id> <count> <tag>: starts box.ps1 (watching for Bridge's box) and boxcheck.py (writes Yes only when the
# box names this company, GUID, batch and count), then sends post_import and waits for its answer.
# Usage: bash step3.sh build r03 | bash step3.sh post r03 <batch_id> 3 r03-p1
set -u; R=<LAB_DIR>; cd $R; G=2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab
if [ "$1" = build ]; then
  b=$2; h=$(sha256sum sheet/batches/$b.json | cut -c1-64); w=$(grep " batches/$b.json\$" sheet/SHA256.txt | cut -c1-64)
  [ "$h" = "$w" ] || { echo "HALT: $b.json sha256 $h differs from SHA256.txt"; exit 2; }
  echo "$b.json sha256 $h = SHA256.txt"
  python -I -X utf8 -c "import json,sys; v=json.load(open(sys.argv[1],encoding='utf-8')); json.dump({'company_guid': sys.argv[2], 'vouchers': v}, open(sys.argv[3],'w',encoding='utf-8'), ensure_ascii=False)" sheet/batches/$b.json $G args-build-$b.json
  python -I -X utf8 call.py run $b-build build_import_xml args-build-$b.json
else
  b=$2; id=$3; n=$4; tag=$5
  printf '{"company_guid":"%s","batch_id":"%s"}' $G $id > args-post-$b.json
  powershell -NoProfile -File $R/box.ps1 -RowDir '<LAB_DIR>' -Tag $tag -SeeWithin 120 -AnswerBy 70 > box/$tag-ps.out 2>&1 &
  sleep 2
  python -I -X utf8 boxcheck.py $tag $id $n > /dev/null 2>&1 &
  python -I -X utf8 call.py run $tag post_import args-post-$b.json
  wait
  echo "--- box log tail"; tail -4 box/$tag-box.txt; echo "--- check"; cat box/$tag-check.out | head -9
fi
