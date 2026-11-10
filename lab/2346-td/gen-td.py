# gen-td.py: row 2346-TD's set-up and notes, made from row 2346-PB's own files with one change, the customer's name.
#
# Master admits a party to the one-party bills read only through LedgerScopeName (ledger_scope.rs:65), which takes
# ASCII letters, digits and single spaces, so PB's customer "BRIDGE Walk-in BC" is refused before any send
# (party_name_not_admitted, agent_party_bills.rs:28). This row's customer is "BRIDGE Walkin BC", which it admits.
#
# For each file of PB's in pb/ (each checked against PB's SHA256.txt first), it:
#   - decodes it as sent (a BOM, then UTF-16LE), and asserts the old name occurs exactly as often as expected and
#     that every "Walk-in" is part of it, and that the new name does not occur;
#   - replaces the old name with the new one, and nothing else;
#   - asserts that turning the new name back gives PB's file byte for byte;
#   - writes the result beside this script under PB's file name.
# Run: python3 -I gen-td.py   (from this folder; it writes 7 files and prints each one's size and sha256)
import hashlib
import os
import sys

OLD = "BRIDGE Walk-in BC"
NEW = "BRIDGE Walkin BC"
BOM = b"\xff\xfe"

# PB's files (lab/2346-pb-sheet at 53540665, its SHA256.txt), and how often each holds the old name.
PB = {
    "L-ledger.xml": ("00ae11c328069b7de58d8464e40a35883ee5686e49c71364cbb4a7a27bd36b42", 2),
    "I-0099.xml": ("b0b6db9d0ac62670246a3122447e645fe43deb49bc54e00caef8b4323266c062", 7),
    "I-0101.xml": ("b1d8ca1b8b35f056a94e9937a604da4f2b1d741f43ff5cddfc53c214b94eb9cb", 7),
    "I-CN-1.xml": ("4e6367469525fa96c5d801f7ed409b32bb60c0abd5b01ddc592616751a64bc00", 7),
    "I-CN-P.xml": ("48e61c4cb0d31e33f8de8e0c52ff03ad4f6ac640eecddd6a87256d1361d21a9c", 7),
    "I-CN-2.xml": ("d32161f35fd0569e8e76e7694e23e4c12e110a57d5889a81396b85cb93fbe09f", 7),
    "I-CN-3.xml": ("ab4846974e37e540f57a8da246820cb0c97d57eda464b8b20d56d6e0ba4788ef", 7),
}


def stop(message):
    print(f"STOP: {message}")
    sys.exit(1)


def encode(text):
    return BOM + text.encode("utf-16-le")


here = os.path.dirname(os.path.abspath(__file__))
for name, (sha, times) in PB.items():
    with open(os.path.join(here, "pb", name), "rb") as handle:
        old_bytes = handle.read()
    if hashlib.sha256(old_bytes).hexdigest() != sha:
        stop(f"pb/{name} is not PB's file")
    if not old_bytes.startswith(BOM):
        stop(f"pb/{name} has no BOM")
    text = old_bytes[2:].decode("utf-16-le")
    if text.count(OLD) != times:
        stop(f"pb/{name} holds the old name {text.count(OLD)} times, not {times}")
    if text.count("Walk-in") != times:
        stop(f"pb/{name} holds a Walk-in that is not the old name")
    if NEW in text:
        stop(f"pb/{name} already holds the new name")
    new_text = text.replace(OLD, NEW)
    if new_text.count(NEW) != times or OLD in new_text:
        stop(f"{name}: the replacement did not take")
    new_bytes = encode(new_text)
    if encode(new_text.replace(NEW, OLD)) != old_bytes:
        stop(f"{name}: turned back, it is not PB's file")
    with open(os.path.join(here, name), "wb") as handle:
        handle.write(new_bytes)
    print(f"{name} {len(new_bytes)} {hashlib.sha256(new_bytes).hexdigest()} (the name {times} times)")
