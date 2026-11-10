#!/usr/bin/env python3
# answer.py ANSWER.json: reads one saved `vouchers` tool answer (the whole JSON-RPC result or its content, as the
# stdio client saved it) and prints its size and sha256, whether it is an error (with the error's code and cause),
# the window block's `census.requests` and `census.ms`, `census_reused` (its `read_at` and every other field) or
# `census_reused absent`, the number of window parts, the number of items, and the items' voucher numbers in the
# order given. With `--same-items A B` it prints whether the two answers' `items` arrays are equal as JSON
# (parsed, not text), and STOPs if either is an error answer or holds no `items` list. It records and never decides.
# STOPs (exit 1): the file is empty or above 4,000,000 bytes, does not parse, or holds no structuredContent object.
# An error answer is a result (exit 0, printed in full below `ERROR`), not a failure of this script.
import hashlib, json, pathlib, sys
BOUND = 4_000_000
def load(path):
    path = pathlib.Path(path)
    raw = path.read_bytes() if path.is_file() else b""
    if not raw or len(raw) > BOUND:
        raise SystemExit(f"STOP: {path} is missing, empty or above {BOUND} bytes")
    try:
        value = json.loads(raw.decode("utf-8"))
    except ValueError as error:
        raise SystemExit(f"STOP: {path} does not parse ({error})")
    body = value.get("result", value) if isinstance(value, dict) else None
    structured = body.get("structuredContent") if isinstance(body, dict) else None
    if not isinstance(structured, dict):
        raise SystemExit(f"STOP: {path} holds no structuredContent object")
    return raw, body, structured
if sys.argv[1] == "--same-items":
    compared = []
    for path in sys.argv[2:4]:
        _, body, structured = load(path)
        result = structured.get("result", {})
        if body.get("isError") is True or result.get("error") or structured.get("error"):
            raise SystemExit(f"STOP: {path} is an error answer, so its items cannot be compared")
        if not isinstance(result.get("items"), list):
            raise SystemExit(f"STOP: {path} holds no items list, so its items cannot be compared")
        compared.append(result["items"])
    ia, ib = compared
    print(f"items equal: {ia == ib} ({len(ia)} and {len(ib)} items)")
    raise SystemExit(0)
raw, body, structured = load(sys.argv[1])
print(f"{sys.argv[1]}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
result = structured.get("result", {})
error = result.get("error") or structured.get("error")
if body.get("isError") is True or error:
    print("ERROR", json.dumps(error, sort_keys=True)[:2000])
window = result.get("window") or (error or {}).get("window") or {}
census = window.get("census", {})
print(f"census.requests {census.get('requests')}\tcensus.ms {census.get('ms')}")
reused = window.get("census_reused")
print("census_reused " + (json.dumps(reused, sort_keys=True) if reused is not None else "absent"))
print(f"window parts {len(window.get('parts', []))}")
items = result.get("items")
if isinstance(items, list):
    print(f"items {len(items)}: " + ", ".join(str(item.get("voucher_number")) for item in items[:200]))
else:
    print("items absent")
