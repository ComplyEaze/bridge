#!/usr/bin/env python3
# count.py RUN_DIR RELAY_DIR: prints shapes and counts for this row, from the client's answers (RUN_DIR/answers/*.json)
# and the relay's log (RELAY_DIR/relay.jsonl). It prints tool names, codes, states, counts and the request sequence
# of each call as letters; no amount, party or ledger name. Copy its whole output into RESULTS.md.
# Example: python3 -I count.py run relay
import json, pathlib, sys
from datetime import datetime, timedelta
run, relay = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
# The request each letter stands for, by the sha256 of its body: the legs of
# a_window_of_foreign_currency_composites_is_read_at_its_rupee_figures_by_all_three_tools
# (src-tauri/src/agent_register_server_tests.rs at a4b4adff), which replays one register call on this book.
LEGS = {
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855": "s",  # GET /status
    "9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8": "e",  # company extent
    "017dd78479ff02e4aae52d7c372da34f061ceb676b7272c0f34a44f72248cd62": "b",  # book extent
    "df26219283f66347545cb3b93261014a2448719cc04778b40b734ebb799769cb": "c",  # currencies
    "0f10c96e7a00445ab6655dd915cb7ff50f9e1adbacef99bc42167a1d43a4c1bd": "C",  # currencies, original names
    "8f6205332f84a0b8bf3df5f55d9869dd79a0bd551d2e51b37a4fa6552a4b1fe0": "k",  # base currency
    "b20e80b785c040a31c70051dab8ded0af0a7e5bdcff479a0ed78e8e5c3d2abb4": "L",  # ledger masters
    "71e429b8c54e890e08152b37b5844932f653b89743462719b16eac4c4100de7b": "B",  # ledger balances
    "205458f1ef405f132ff4e87703f258f7cae16f6aadbafda2ea12943d434ec0e8": "g",  # groups
    "12e97cc3a11334e443568e1d5a956718d429f1afce590246dc0cb0b1f3bf853c": "n",  # census
    "ab78fbe5afeb6734a3ec36a940dc098371560faf0a1210ca7e511927ee3dd2f5": "w",  # class-entry window
    "5b447d03d63414bb46753503faee9b105347e13c28cac548bfd51d6fdb9fb4b7": "m",  # marks
}
EXPECTED = ("esesebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbseseensnseewswse"
            "ebsbscscsCsCsksksbsbse"
            "emsmseebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbsese")
REGISTERS = ("sales_register", "purchase_register", "gst_by_counter_ledger")
LISTS = ("sales_vouchers_without_duties_taxes_entry", "purchase_vouchers_without_duties_taxes_entry")
def utc(text):
    return datetime.strptime(text.rstrip("Z")[:19], "%Y-%m-%dT%H:%M:%S")
calls = []
for path in sorted((run / "answers").glob("*.json")):
    answer = json.loads(path.read_text(encoding="utf-8"))
    calls.append((path.name, answer))
events = [json.loads(line) for line in (relay / "relay.jsonl").read_text(encoding="utf-8").splitlines() if line.strip()]
exchanges = [e for e in events if "sent_sha256" in e]
print(f"relay: {len(exchanges)} exchanges, import_refused {sum(1 for e in events if e.get('event') == 'import_refused')},"
      f" reply_lost {sum(1 for e in events if e.get('event') == 'reply_lost')},"
      f" connections {sum(1 for e in events if e.get('event') == 'open')}")
# Each exchange belongs to the last call sent at or before it (call times are whole seconds).
starts = [(utc(a["call_sent_utc"]), name) for name, a in calls if "call_sent_utc" in a]
per_call = {name: [] for _, name in starts}
for e in exchanges:
    at = utc(e["utc"])
    owner = [name for start, name in starts if start <= at + timedelta(seconds=0)]
    if owner:
        per_call[owner[-1]].append(e)
for name, answer in calls:
    tool = answer.get("tool")
    result = answer.get("response", {}).get("result", {})
    content = result.get("structuredContent") or {}
    body = content.get("result") or {}
    error = body.get("error") if isinstance(body, dict) else None
    line = f"{name}: tool {tool}, isError {result.get('isError')}, {answer.get('round_trip_seconds')} s"
    if error:
        line += f", code {error.get('code')}, cause {error.get('cause')}"
    elif isinstance(body, dict) and body:
        line += (f", state {body.get('state')}, reason {body.get('reason')}, total {body.get('total')},"
                 f" withheld_total {body.get('withheld_total', 'absent')}, vouchers_observed {body.get('vouchers_observed', 'absent')},"
                 f" ledger_masters_observed {body.get('ledger_masters_observed', 'absent')}")
        for key in LISTS:
            if isinstance(body.get(key), dict):
                line += f", {key}.total {body[key].get('total')}"
        if "vouchers_without_gst_entry" in body:
            line += f", vouchers_without_gst_entry {body['vouchers_without_gst_entry']}"
        if tool == "vouchers":
            items = body.get("items") or []
            line += f", items {len(items)}, with a foreign entry {sum(1 for v in items if any(e.get('foreign') for e in v.get('amounts', [])))}"
    print(line)
    if name not in per_call:
        continue
    sent = per_call[name]
    letters = "".join(LEGS.get(e["sent_sha256"], "?") for e in sent)
    posts = sum(1 for e in sent if e.get("line", "").startswith("POST"))
    print(f"  exchanges {len(sent)} (POST {posts}, other {len(sent) - posts}), unknown requests {letters.count('?')}")
    if tool in REGISTERS:
        same = letters == EXPECTED
        print(f"  sequence equals the replayed register call ({len(EXPECTED)} legs): {'yes' if same else 'no'}")
        if not same:
            first = next((i for i, (a, b) in enumerate(zip(letters, EXPECTED)) if a != b), min(len(letters), len(EXPECTED)))
            print(f"  first difference at leg {first + 1}: sent {letters[first:first + 12]!r}, expected {EXPECTED[first:first + 12]!r}")
        print(f"  letters: {letters}")
