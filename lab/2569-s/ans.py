# -*- coding: ascii -*-
# Lab helper (row 2569-S; not product code). Python 3.10, standard library only, read-only.
# ans.py ANSWER.json: prints the lines the sheet grades from one answer that client.py saved, bounded:
#   every answer: tool, round trip, isError, a JSON-RPC error, the reason code;
#   list: the tool count and whether each of the five tools this row calls is there;
#   list_companies: each company's name, whether its GUID is the book's, identity state (at most 10);
#   ledger_masters: the snapshot's master and voucher alter ids and reused;
#   build_import_xml: batch_id, voucher_count, the HSN warning count, the error code;
#   post_import: approval state, dialog_remaining_s, retry_after_s, attempt_recorded, dispatch state, error code.
# Exit 0 when the answer was read (a refusal is an answer); 2 when the file is not a whole client answer
# (missing, not JSON, no response, or a result with no structured content): a failed read, never "nothing".
import json
import sys

BOOK = "eb2f5190-292e-4ed8-bfd3-eace839fe699"
TOOLS = ("list_companies", "ledger_masters", "build_import_xml", "post_import", "verify_import")


def fail(why):
    print("STOP: " + why)
    sys.exit(2)


def main(path):
    try:
        with open(path, encoding="utf-8") as f:
            body = json.load(f)
    except (OSError, ValueError) as e:
        fail("unreadable answer %s (%s)" % (path, type(e).__name__))
    if not isinstance(body, dict) or "response" not in body or "tool" not in body:
        fail("not a client answer: no tool or response")
    tool = body["tool"]
    res = body["response"]
    print("tool %s round_trip_s %s call_sent %s" % (tool, body.get("round_trip_seconds"), body.get("call_sent_utc")))
    if res.get("error") is not None:
        print("jsonrpc_error %s" % json.dumps(res["error"])[:300])
        return
    result = res.get("result")
    if not isinstance(result, dict):
        fail("response has neither result nor error")
    if tool == "list":
        names = [t.get("name") for t in result.get("tools", [])]
        print("tools %d" % len(names))
        for name in TOOLS:
            print("tool %s present %s" % (name, name in names))
        return
    sc = result.get("structuredContent")
    if not isinstance(sc, dict):
        fail("result holds no structuredContent")
    print("isError %s" % result.get("isError"))
    ev = sc.get("evidence") or {}
    if ev.get("reason_code") is not None:
        print("reason_code %s" % ev.get("reason_code"))
    inner = sc.get("result") or {}
    err = inner.get("error") or {}
    if err:
        print("error_code %s" % err.get("code"))
    if tool == "list_companies":
        companies = inner.get("companies") or []
        print("companies %d" % len(companies))
        for c in companies[:10]:
            print("company name [%s] guid_is_book %s identity %s" % (c.get("name"), c.get("guid") == BOOK, c.get("identity_state")))
        if len(companies) > 10:
            print("companies omitted %d" % (len(companies) - 10))
    elif tool == "ledger_masters":
        snap = inner.get("snapshot") or {}
        company = sc.get("company") or {}
        print("company guid_is_book %s" % (company.get("guid") == BOOK))
        print("marks master_alter_id %s voucher_alter_id %s reused %s" % (
            snap.get("master_alter_id"), snap.get("voucher_alter_id"), snap.get("reused")))
    elif tool == "build_import_xml":
        print("batch_id %s voucher_count %s hsn_warnings %s" % (
            inner.get("batch_id"), inner.get("voucher_count"), len(inner.get("hsn_warnings") or [])))
    elif tool == "post_import":
        approval = inner.get("approval") or {}
        dispatch = inner.get("dispatch") or {}
        print("batch_id %s" % inner.get("batch_id"))
        print("approval state %s dialog_remaining_s %s retry_after_s %s" % (
            approval.get("state"), approval.get("dialog_remaining_s"), approval.get("retry_after_s")))
        print("attempt_recorded %s dispatch state %s" % (inner.get("attempt_recorded"), dispatch.get("state")))
    else:
        print("keys %s" % sorted(inner)[:20])


if __name__ == "__main__":
    if len(sys.argv) != 2:
        fail("usage: python -I ans.py ANSWER.json")
    main(sys.argv[1])
