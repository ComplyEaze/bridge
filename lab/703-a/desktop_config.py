#!/usr/bin/env python3
"""Adds one of row 703-A's lab server entries to the desktop app's configuration, and puts the original bytes back.

The configuration is the file the app reads (its path is recorded in the notes on the lab PC and never pushed). No
editor opens it, so no save prompt can appear: this script reads and writes its bytes.

  python -I desktop_config.py add <config> <before-copy> <server.mjs> <tools.json> <log> [arguments <step-scale>]
      Refuses if <before-copy> or <log> exists. Copies the configuration's bytes to <before-copy>, prints its sha256
      (B0), adds the entry `bridge-lab-703a` to `mcpServers` (refused if that name is already there; every other key
      and entry is kept as parsed), writes the file as UTF-8 without a byte-order mark, and prints the new sha256
      (B1). With `arguments <step-scale>` (sheet version 3, phase S) the entry runs the server with
      `--continue-by arguments --step-scale <step-scale>` (0.01 to 1), and <tools.json> must be tools-args.json.
  python -I desktop_config.py add-sleep <config> <before-copy> <mcp-sleep-server.mjs> <log>
      The same, for phase H: the entry `bridge-lab-703h` runs the repository's sleep server with `--log <log>`.
  python -I desktop_config.py restore <config> <before-copy> <B0>
      Refuses unless <before-copy>'s sha256 is B0. Writes those bytes back over the configuration, reads the file again
      and prints its sha256, which must be B0 (exit 1 otherwise). Run it before the app is started again: the app
      rewrites its configuration when it starts (lab/2119-bills-60s-row-answers, RESTORE.md).
  python -I desktop_config.py compare <config> <before-copy>
      After a start: parses both files and prints the server names in each, the names added and removed, and the
      top-level keys added, removed and changed. Names only, never values.

Exit 0 = done, 1 = refused or failed (the line printed says why), 2 = the command line is wrong.
"""
import hashlib
import json
import os
import sys

ENTRY = "bridge-lab-703a"
SLEEP_ENTRY = "bridge-lab-703h"


class Failed(Exception):
    pass


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def parse(data, path):
    text = data[3:] if data.startswith(b"\xef\xbb\xbf") else data
    try:
        value = json.loads(text.decode("utf-8"))
    except (UnicodeDecodeError, ValueError) as error:
        raise Failed("{} does not parse as JSON: {}".format(path, error))
    if not isinstance(value, dict):
        raise Failed("{} is not a JSON object".format(path))
    return value


def servers(config):
    found = config.get("mcpServers", {})
    if not isinstance(found, dict):
        raise Failed("mcpServers is not an object")
    return found


def add(config_path, before_path, server_path, tools_path, log_path, mode=None, scale=None):
    if mode not in (None, "arguments"):
        raise Failed("the sixth word may only be arguments, not {}".format(mode))
    if mode == "arguments":
        if os.path.basename(tools_path) != "tools-args.json":
            raise Failed("continuation by arguments is served with tools-args.json, not {}".format(tools_path))
        try:
            ok = 0.01 <= float(scale) <= 1
        except ValueError:
            ok = False
        if not ok:
            raise Failed("the step scale must be a number from 0.01 to 1, not {}".format(scale))
    elif os.path.basename(tools_path) != "tools.json":
        raise Failed("continuation by id is served with tools.json, not {}".format(tools_path))
    args = [server_path, "--tools", tools_path, "--log", log_path] + (
        ["--continue-by", "arguments", "--step-scale", scale] if mode else [])
    return add_entry(config_path, before_path, ENTRY, args, (server_path, tools_path, log_path), log_path)


def add_sleep(config_path, before_path, server_path, log_path):
    if os.path.basename(server_path) != "mcp-sleep-server.mjs":
        raise Failed("{} is not the sleep server".format(server_path))
    return add_entry(config_path, before_path, SLEEP_ENTRY, [server_path, "--log", log_path], (server_path, log_path), log_path)


def add_entry(config_path, before_path, entry, args, paths, log_path):
    for path in paths:
        if not os.path.isabs(path):
            raise Failed("{} is not an absolute path (the app starts the server in a folder of its own)".format(path))
    if os.path.exists(before_path):
        raise Failed("{} exists: a before-copy is never overwritten".format(before_path))
    if os.path.exists(log_path):
        raise Failed("{} exists: the row's log starts empty".format(log_path))
    data = read(config_path)
    config = parse(data, config_path)
    # Checked before the before-copy is written, so a refused add leaves nothing behind (version 3).
    found = servers(config)
    for name in (ENTRY, SLEEP_ENTRY):
        if name in found:
            raise Failed("{} is already in mcpServers".format(name))
    with open(before_path, "xb") as handle:
        handle.write(data)
    if read(before_path) != data:
        raise Failed("the before-copy differs from the configuration")
    found[entry] = {"command": "node", "args": args}
    config["mcpServers"] = found
    out = json.dumps(config, indent=2, ensure_ascii=False).encode("utf-8")
    with open(config_path, "wb") as handle:
        handle.write(out)
    return "add ok B0 {} ({} bytes) B1 {} ({} bytes) servers {}".format(
        sha256(data), len(data), sha256(read(config_path)), len(out), ",".join(sorted(found)))


def restore(config_path, before_path, expected):
    data = read(before_path)
    if sha256(data) != expected:
        raise Failed("the before-copy's sha256 is {}, not {}".format(sha256(data), expected))
    with open(config_path, "wb") as handle:
        handle.write(data)
    now = sha256(read(config_path))
    if now != expected:
        raise Failed("the configuration reads {} after the write, not {}".format(now, expected))
    return "restore ok {} ({} bytes)".format(now, len(data))


def compare(config_path, before_path):
    now = parse(read(config_path), config_path)
    before = parse(read(before_path), before_path)
    names_now, names_before = set(servers(now)), set(servers(before))
    keys_now, keys_before = set(now), set(before)
    changed = sorted(k for k in keys_now & keys_before if k != "mcpServers" and now[k] != before[k])
    return "compare sha256 {} servers_now {} servers_before {} added {} removed {} keys_added {} keys_removed {} keys_changed {}".format(
        sha256(read(config_path)), ",".join(sorted(names_now)), ",".join(sorted(names_before)),
        ",".join(sorted(names_now - names_before)) or "-", ",".join(sorted(names_before - names_now)) or "-",
        ",".join(sorted(keys_now - keys_before)) or "-", ",".join(sorted(keys_before - keys_now)) or "-",
        ",".join(changed) or "-")


def main(argv):
    if len(argv) in (6, 8) and argv[0] == "add":
        return add(*argv[1:])
    if len(argv) == 5 and argv[0] == "add-sleep":
        return add_sleep(*argv[1:])
    if len(argv) == 4 and argv[0] == "restore":
        return restore(*argv[1:])
    if len(argv) == 3 and argv[0] == "compare":
        return compare(*argv[1:])
    return None


if __name__ == "__main__":
    try:
        result = main(sys.argv[1:])
    except (Failed, OSError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    if result is None:
        print(__doc__)
        sys.exit(2)
    print(result)
    sys.exit(0)
