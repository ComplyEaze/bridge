#!/usr/bin/env python3
# mkenv.py RUNDIR DATADIR: makes one launch's folders for client.py and writes its launch-env.json, for row
# 2162-SALES-Q. Both folders must not exist yet (a launch never reuses one). It makes DATADIR (empty), RUNDIR, and
# in RUNDIR the empty folders inbox and answers, then writes RUNDIR\launch-env.json by json.dumps (no shell, so no
# escape in a path is rewritten), reads the bytes back, and prints them and their sha256.
# The keys are row 2569-R's and row 2440-CP's, every launch of this row the same but for the data folder:
#   BRIDGE_TALLY_HOST 127.0.0.1, BRIDGE_TALLY_PORT 9001, BRIDGE_AGENT_DATA_DIR DATADIR with a trailing separator,
#   BRIDGE_TERMS_ACCEPTED true, BRIDGE_AGENT_ENABLE_WRITES true.
# Nothing else: BRIDGE_AGENT_REDACTION, BRIDGE_AGENT_MAX_BYTES and BRIDGE_AGENT_ENABLE_BATCH_POST are never written
# (client.py refuses to start with any of them, or with BRIDGE_AGENT_ENABLE_MASTER_WRITES).
# Example: python3 -I mkenv.py C:\bridge-lab\2162-sales-q\run-0 C:\bridge-lab\2162-sales-q\data-0
import hashlib, json, os, pathlib, sys
if len(sys.argv) != 3:
    raise SystemExit("STOP: usage mkenv.py RUNDIR DATADIR")
run, data = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
for p in (run, data):
    if p.exists():
        raise SystemExit(f"STOP: {p} exists; a launch takes new folders")
data.mkdir(parents=True)
(run / "inbox").mkdir(parents=True)
(run / "answers").mkdir()
env = {
    "BRIDGE_TALLY_HOST": "127.0.0.1",
    "BRIDGE_TALLY_PORT": "9001",
    "BRIDGE_AGENT_DATA_DIR": str(data.resolve()).rstrip(os.sep) + os.sep,
    "BRIDGE_TERMS_ACCEPTED": "true",
    "BRIDGE_AGENT_ENABLE_WRITES": "true",
}
target = run / "launch-env.json"
target.write_text(json.dumps(env, indent=1) + "\n", encoding="utf-8", newline="\n")
raw = target.read_bytes()
back = json.loads(raw.decode("utf-8"))
assert back == env, "the file does not read back as written"
print(raw.decode("utf-8"), end="")
print(f"{target}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
print(f"{data}: {len(list(data.iterdir()))} entries; inbox {len(list((run / 'inbox').iterdir()))}, "
      f"answers {len(list((run / 'answers').iterdir()))}")
