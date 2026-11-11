#!/usr/bin/env python3
"""Writes tools.json: the text the dev server serves, taken from Bridge's own source (lab row 703-A, #703).

  python -I extract_text.py <folder holding the four files below> <commit, 8 characters> > tools.json
  python -I extract_text.py <folder> <commit> arguments > tools-args.json

The second form (sheet version 3) writes the text for continuation by the same arguments: the planned texts are the
`*_ARGS` ones below, and the `vouchers` schema gets no read_id. Everything taken from the source is the same.

The four files are `agent_protocol.rs`, `agent_catalog.rs`, `agent_import.rs` and `agent_voucher_search.rs` from
`src-tauri/src/`, as `git show <commit>:<path>` prints them. The row measures whether a model carries a read_id on
the text Bridge ships, so nothing here is written by hand except the four texts marked `planned` below (two sentences,
the read_id parameter and the done-false result's next_step), the build's proposed additions (design section 6) and
the one thing the row varies. Taken from the source, each exactly once or the run fails (exit 1), and assembled as
`agent_catalog.rs` and `agent_protocol.rs` assemble them:
  - the server instructions: `SERVER_INSTRUCTIONS`, then the `Prepare` arm of `VoucherReach::instruction`, the
    default bundle's (import served, posting off);
  - the `list_companies` and `vouchers` descriptions, each followed by `READ_RECEIPT_SENTENCE` and given the
    `ToolEffect::Read` annotations, as every read tool is;
  - the `vouchers` input schema, the whole `json!` literal, with its two constants put in from `agent_import.rs` and
    `agent_voucher_search.rs`, and each property given the first `parameter_description` arm that matches it, in
    source order, as Rust's `match` does.
"""
import json
import re
import sys

LITERAL = r'"((?:[^"\\]|\\.)*)"'
ESCAPES = {"n": "\n", "t": "\t", "\\": "\\", '"': '"', "'": "'", "0": "\0"}

PLANNED_STEP_SENTENCE = (
    "A read too long for one call is served in steps: a result with done false carries read_id, step and steps and "
    "no rows. Call vouchers again with the same company_guid, from, to and other arguments plus that read_id, without "
    "asking the user, until done is true; only that last result carries the rows. Each step returns in under a minute. "
    "A read_id that is unknown, finished or idle for 15 minutes is refused (read_unknown, read_finished, "
    "read_expired), and so is one sent with other arguments (read_arguments_changed): start again without it."
)
PLANNED_INSTRUCTION = (
    " A result with done false is not an answer: call the same tool again with its read_id until done is true, "
    "without asking the user."
)
NEXT_STEP = "Call vouchers again with the same arguments and this read_id."
# Continuation by the same arguments (sheet version 3, question 2(a) of the #703 design note): no read_id is served.
PLANNED_STEP_SENTENCE_ARGS = (
    "A read too long for one call is served in steps: a result with done false carries step and steps and no rows. "
    "Call vouchers again with exactly the same arguments, without asking the user, until done is true; only that "
    "last result carries the rows. Each step returns in under a minute. A read left idle for 15 minutes, or followed "
    "by a vouchers call with other arguments, is dropped, and the next call starts it again from its first step."
)
PLANNED_INSTRUCTION_ARGS = (
    " A result with done false is not an answer: call the same tool again with exactly the same arguments until done "
    "is true, without asking the user."
)
NEXT_STEP_ARGS = "Call vouchers again with exactly the same arguments."
READ_ID_DESCRIPTION = (
    "Optional: the read_id of a read still in progress (done false), to run its next step. Give the same other "
    "arguments as the first call."
)
class Failed(Exception):
    pass


def unescape(text):
    out = []
    i = 0
    while i < len(text):
        if text[i] != "\\":
            out.append(text[i])
            i += 1
            continue
        nxt = text[i + 1]
        if nxt in ESCAPES:
            out.append(ESCAPES[nxt])
            i += 2
        elif nxt == "u":
            end = text.index("}", i)
            out.append(chr(int(text[i + 3:end], 16)))
            i = end + 1
        elif nxt == "\n":
            i += 2
            while i < len(text) and text[i] in " \t\n":
                i += 1
        else:
            raise Failed("unknown escape \\{}".format(nxt))
    return "".join(out)


def once(pattern, source, what):
    found = re.findall(pattern, source, re.S)
    if len(found) != 1:
        raise Failed("{}: found {} times, expected once".format(what, len(found)))
    return unescape(found[0])


def read(directory, name):
    with open(directory + "/" + name, encoding="utf-8") as handle:
        return handle.read()


def constant(source, name, what):
    return once(r"const " + name + r": usize = ([0-9]+);", source, what)


def main(directory, commit, continue_by="id"):
    protocol = read(directory, "agent_protocol.rs")
    catalog = read(directory, "agent_catalog.rs")
    names = constant(read(directory, "agent_import.rs"), "MAX_MASTER_NAME_CHARS", "MAX_MASTER_NAME_CHARS")
    terms = constant(read(directory, "agent_voucher_search.rs"), "MAX_SEARCH_TERM_CHARS", "MAX_SEARCH_TERM_CHARS")
    instructions = once(r"const SERVER_INSTRUCTIONS: &str = " + LITERAL + ";", protocol, "SERVER_INSTRUCTIONS")
    prepare = once(r"Self::Prepare => " + LITERAL + ",", protocol, "VoucherReach Prepare instruction")
    receipt = once(r"const READ_RECEIPT_SENTENCE: &str = " + LITERAL + ";", catalog, "READ_RECEIPT_SENTENCE")
    annotations = json.loads(once(r"Self::Read => \{\s*json!\((\{[^}]*\})\)\s*\}", catalog, "Read annotations"))
    companies = once(r'"list_companies" => \(\s*' + r'"(Start here\. (?:[^"\\]|\\.)*)"', catalog, "list_companies")
    vouchers = once(r'"vouchers" => \(\s*' + r'"(Return literal-window voucher evidence(?:[^"\\]|\\.)*)"', catalog, "vouchers")
    guid = once(r"const COMPANY_GUID_DESCRIPTION: &str =\s*" + LITERAL + ";", catalog, "COMPANY_GUID_DESCRIPTION")
    raw = re.findall(r'"vouchers" => \(\s*"Return literal-window voucher evidence(?:[^"\\]|\\.)*",\s*json!\((.*?)\),\n', catalog, re.S)
    if len(raw) != 1:
        raise Failed("vouchers schema: found {} times, expected once".format(len(raw)))
    text = raw[0].replace("agent_import::MAX_MASTER_NAME_CHARS", names)
    text = text.replace("voucher_search::MAX_SEARCH_TERM_CHARS", terms).replace('r"\\S"', '"\\\\S"')
    try:
        schema = json.loads(text)
    except ValueError as error:
        raise Failed("vouchers schema does not parse once its constants are put in: {}".format(error))
    body = catalog[catalog.index("fn parameter_description("):]
    body = body[:body.index("\n}\n")]
    for name, entry in schema["properties"].items():
        if "description" in entry:
            continue
        arms = re.findall(r'\((_|"vouchers"), "' + re.escape(name) + r'"\) => (' + LITERAL + r'|COMPANY_GUID_DESCRIPTION)',
                          body)
        if not arms:
            raise Failed("no parameter_description arm for vouchers parameter {}".format(name))
        first = arms[0][1]
        entry["description"] = guid if first == "COMPANY_GUID_DESCRIPTION" else unescape(first[1:-1])
    if continue_by == "arguments":
        step_sentence, instruction = PLANNED_STEP_SENTENCE_ARGS, PLANNED_INSTRUCTION_ARGS
        planned = {"continue_by": "arguments", "instruction": instruction.strip(),
                   "vouchers_description": step_sentence, "next_step": NEXT_STEP_ARGS}
    else:
        step_sentence, instruction = PLANNED_STEP_SENTENCE, PLANNED_INSTRUCTION
        schema["properties"]["read_id"] = {"type": "string", "minLength": 1, "maxLength": 64,
                                           "description": READ_ID_DESCRIPTION, "planned": True}
        planned = {"instruction": PLANNED_INSTRUCTION.strip(), "vouchers_description": PLANNED_STEP_SENTENCE,
                   "read_id": READ_ID_DESCRIPTION, "next_step": NEXT_STEP}
    return {
        "source_commit": commit,
        "reach": "Prepare",
        "instructions": instructions + " " + prepare + instruction,
        "planned": planned,
        "tools": [
            {"name": "list_companies", "description": companies + " " + receipt,
             "inputSchema": {"type": "object", "additionalProperties": False}, "annotations": annotations},
            {"name": "vouchers", "description": vouchers + " " + receipt + " " + step_sentence,
             "inputSchema": schema, "annotations": annotations},
        ],
    }


if __name__ == "__main__":
    if len(sys.argv) not in (3, 4) or not re.fullmatch(r"[0-9a-f]{8}", sys.argv[2]) or sys.argv[3:] not in ([], ["arguments"]):
        print(__doc__)
        sys.exit(2)
    try:
        document = main(sys.argv[1], sys.argv[2], "arguments" if sys.argv[3:] else "id")
    except (Failed, OSError, ValueError) as error:
        print("FAIL: {}".format(error), file=sys.stderr)
        sys.exit(1)
    # The "planned" key on read_id is the kit's own marker; the server strips it before serving.
    sys.stdout.buffer.write((json.dumps(document, indent=1, ensure_ascii=False) + "\n").encode("utf-8"))
