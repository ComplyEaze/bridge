#!/usr/bin/env python3
# rpc.py CLIENT_ANSWER NEW_FILE (row 2399-D, a lab helper, not product code). The lab's stdio client saves each
# answer as one JSON object with the JSON-RPC message under its own key "response"; answer.py reads the message
# itself. This writes the "response" value alone to NEW_FILE (UTF-8, json.dumps with ensure_ascii False), never
# over an existing file, then parses NEW_FILE again and prints whether it equals that value. It prints both
# files' sizes and sha256. The client's file is only read.
# STOPs (exit 1): wrong arguments, a missing, empty or unparsable client file, a "response" that is absent or not
# a JSON-RPC message (an object holding "result" or "error"), NEW_FILE already present, or a copy that does not
# parse equal. It prints which of "result" and "error" the message holds.
import hashlib, json, pathlib, sys
if len(sys.argv) != 3:
    raise SystemExit("STOP: usage rpc.py CLIENT_ANSWER NEW_FILE")
source, target = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
raw = source.read_bytes() if source.is_file() else b""
if not raw:
    raise SystemExit(f"STOP: {source} is missing or empty")
try:
    saved = json.loads(raw.decode("utf-8"))
except ValueError as error:
    raise SystemExit(f"STOP: {source} does not parse ({error})")
if not isinstance(saved, dict) or "response" not in saved:
    raise SystemExit(f"STOP: {source} holds no \"response\" key")
message = saved["response"]
if not isinstance(message, dict) or not ("result" in message or "error" in message):
    raise SystemExit(f"STOP: {source}'s \"response\" is not a JSON-RPC message holding result or error")
copy = json.dumps(saved["response"], ensure_ascii=False).encode("utf-8")
try:
    with open(target, "xb") as out:
        out.write(copy)
except FileExistsError:
    raise SystemExit(f"STOP: {target} is already present")
equal = json.loads(target.read_bytes().decode("utf-8")) == saved["response"]
print(f"{source}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
print(f"{target}: {len(copy)} bytes, sha256 {hashlib.sha256(copy).hexdigest()}, equal to the response value: {equal}")
print("message holds: " + " and ".join(key for key in ("result", "error") if key in message))
if not equal:
    raise SystemExit("STOP: the copy does not parse equal to the response value")
