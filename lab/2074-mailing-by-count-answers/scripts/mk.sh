#!/bin/sh
# Lab helper (#2074-L): one raw MARKS read by C3's curl line, kept as raw/<label>.answer.xml, read by C4.
cd <LAB_DIR> && curl.exe -sS -w " %{http_code} %{size_download}" --data-binary @raw/marks-batch.xml -H "Content-Type: text/xml; charset=utf-16" -D raw/$1.headers http://127.0.0.1:9001 -o raw/$1.answer.xml; echo -n " $(date -u +%H:%M:%S)Z "; python -I <lab folder>/rc070/r3/read.py marks raw/$1.answer.xml a30f23c3-4819-4334-96ab-ffa6fba70976
