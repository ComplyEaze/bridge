# Lab helper for item 1738-COUNTS (not product code). Reads one copy of ComplyEaze Bridge's import journal
# (agent-import-ledger.jsonl) and, if given, its skip-only file (agent-import-statement-skips.jsonl), and prints
# counts as JSON: candidate transfer double bookings, sends Tally answered as creating none of their vouchers, and
# skipped rows posted by a batch found in the book. It reads only the files named; it never writes them, opens no
# network connection and sends nothing to Tally. It prints no reference, amount, date, ledger or company name, and no
# batch id unless --ids is given (batch ids stay on the lab PC). Every count is meant as an over-count of what
# ComplyEaze Bridge itself reports: each condition the product applies and this script does not is one that can only
# add candidates, never remove one. So a zero is decisive, and a non-zero is a lead to check with verify_import.
# Usage: python -I count_journals.py <journal copy> [<skips copy>] [--ids]
import json
import sys
from datetime import datetime

COUNTERS = ('created', 'altered', 'deleted', 'ignored', 'errors', 'cancelled', 'exceptions')


def read_lines(path):
    """Each line parsed as JSON (a BOM is allowed); a line that does not parse is counted, never skipped silently."""
    rows, bad = [], 0
    with open(path, 'rb') as fh:
        for number, raw in enumerate(fh):
            text = raw.decode('utf-8-sig' if number == 0 else 'utf-8').strip()
            if not text:
                continue
            try:
                rows.append(json.loads(text))
            except ValueError:
                bad += 1
    return rows, bad


def when(text):
    """built_at as a time; an unreadable one sorts first, so it never makes a later record look newer."""
    try:
        return datetime.fromisoformat(str(text).replace('Z', '+00:00')).timestamp()
    except ValueError:
        return float('-inf')


def canonical_amount(text):
    text = str(text).lstrip('-').lstrip('0') or '0'
    if '.' in text:
        text = text.rstrip('0').rstrip('.')
    return text or '0'


def lineage(line):
    return line.get('amends_batch_id') or line.get('batch_id')


def company(line):
    return str(line.get('company_guid', '')).lower()


def main():
    args = [a for a in sys.argv[1:] if a != '--ids']
    show_ids = '--ids' in sys.argv[1:]
    rows, bad = read_lines(args[0])
    lines = [r for r in rows if 'record_type' not in r]           # every build line, in file order
    statuses = {}
    for row in rows:
        if 'record_type' in row:
            statuses.setdefault(row.get('batch_id'), []).append(row)
    batches = {}
    for line in lines:
        batches[line.get('batch_id')] = line                        # the latest line of a batch
    sent = {b for b, s in statuses.items() if any(r.get('record_type') == 'dispatch_intent' for r in s)}
    # Found: a verification status, or (journals written before status records) a build line whose own status says so.
    found = {b for b, s in statuses.items()
             if any(r.get('record_type') == 'verification_status' and r.get('status') == 'posted_verified' for r in s)}
    found |= {l.get('batch_id') for l in lines if l.get('status') == 'posted_verified'}
    sent_or_found = sent | found
    # Writers of a rollback record only; the product covers other batches chosen by mark, so this over-counts except
    # for a writer later found posted, which is kept here (not dropped).
    rolled = {b for b, s in statuses.items() if any(r.get('record_type') == 'rollback_seen' for r in s)} - found

    # A. Per lineage and statement row: the newest build line holding the row as a voucher decides its voucher, and
    # the newest statement entry naming it decides its reference and bank (as the product's deciding build does).
    ordered = sorted(enumerate(lines), key=lambda item: (when(item[1].get('built_at')), item[0]))
    voucher_of, facts_of = {}, {}
    for _, line in ordered:
        key_base = (company(line), line.get('endpoint_origin'), lineage(line))
        statement = line.get('statement') or {}
        for voucher in line.get('vouchers') or []:
            voucher_of[key_base + (voucher.get('bridge_txn_id'),)] = (line.get('batch_id'), voucher)
        for row in statement.get('contra_rows') or []:
            facts_of[key_base + (row.get('bridge_txn_id'),)] = (row.get('reference'), statement.get('bank_ledger_guid'))
    contras = []
    for key, (batch_id, voucher) in voucher_of.items():
        if voucher.get('voucher_type') != 'Contra' or key not in facts_of:
            continue
        reference, bank = facts_of[key]
        amounts = {canonical_amount(e.get('amount')) for e in voucher.get('entries') or []}
        contras.append({'batch': batch_id, 'company': key[0], 'endpoint': key[1], 'lineage': key[2],
                        'reference': reference or None, 'bank': bank, 'date': voucher.get('date'),
                        'amount': min(amounts) if amounts else None})
    equal_ref, missing_ref = [], []
    for i, a in enumerate(contras):
        for b in contras[i + 1:]:
            if (a['company'], a['endpoint'], a['amount']) != (b['company'], b['endpoint'], b['amount']) \
                    or a['bank'] == b['bank'] or a['lineage'] == b['lineage']:
                continue
            if a['reference'] and a['reference'] == b['reference']:
                equal_ref.append((a['batch'], b['batch']))         # the product's RefuseAtBuild class
            elif (not a['reference'] or not b['reference']) and a['date'] == b['date']:
                missing_ref.append((a['batch'], b['batch']))       # the product's List class, k = 0 days
    in_book = [p for p in equal_ref if all(x in sent_or_found for x in p)]

    # B. The latest dispatch answer with an outcome reads every counter present, created to cancelled 0, and the
    # exceptions equal to the vouchers of the batch's latest line (the product's not-created shape).
    not_created = []
    for batch_id, records in statuses.items():
        answers = [r for r in records if r.get('record_type') == 'dispatch_response'
                   and (r.get('response') or {}).get('outcome') is not None]
        if not answers or batch_id not in batches:
            continue
        counters = answers[-1]['response']['outcome'].get('counters') or {}
        presence = counters.get('counter_presence') or {}
        count = len(batches[batch_id].get('vouchers') or [])
        if count >= 1 and all(presence.get(k) is True for k in COUNTERS) \
                and all(counters.get(k) == 0 for k in COUNTERS[:6]) and counters.get('exceptions') == count:
            not_created.append(batch_id)

    # C. A skip row's newest mention (from build lines and the skip-only file); a batch of another lineage, found
    # posted and not rolled back, holding the row as a voucher; and no withdrawal of the row by a build of that
    # lineage that was sent or found posted, built after the newest mention.
    mentions = {}
    sources = [(lineage(l), when(l.get('built_at')), l.get('statement') or {}, company(l)) for l in lines]
    skip_bad = 0
    if len(args) > 1:
        skip_rows, skip_bad = read_lines(args[1])
        sources += [(None, when(r.get('built_at')), r.get('statement') or {}, company(r)) for r in skip_rows]
    for own, built, statement, owner in sources:
        for row in statement.get('skip_rows') or []:
            key = (owner, row.get('bridge_txn_id'))
            if key not in mentions or built > mentions[key][1]:
                mentions[key] = (own, built)
    posted_against = []
    for (owner, txn), (own, built) in mentions.items():
        for batch_id, line in batches.items():
            if company(line) != owner or lineage(line) == own or batch_id not in found or batch_id in rolled:
                continue
            if txn not in {v.get('bridge_txn_id') for v in line.get('vouchers') or []}:
                continue
            withdrawn = any(lineage(l) == lineage(line) and l.get('batch_id') in sent_or_found
                            and txn in (l.get('withdrawn_skips') or []) and when(l.get('built_at')) > built
                            for l in lines)
            if not withdrawn:
                posted_against.append(batch_id)
                break

    out = {
        'journal_lines': len(rows) + bad, 'lines_not_parsed': bad, 'skip_lines_not_parsed': skip_bad,
        'batches': len(batches), 'status_records': sum(len(s) for s in statuses.values()),
        'batches_sent': len(sent), 'batches_found_posted': len(found),
        'contras_with_statement_facts': len(contras),
        'A_equal_reference_pairs': len(equal_ref), 'A_equal_reference_pairs_both_sent_or_found': len(in_book),
        'A_missing_reference_same_day_pairs': len(missing_ref),
        'B_not_created_sends': len(not_created),
        'C_skips_posted_by_a_found_batch_unwithdrawn': len(posted_against),
    }
    if show_ids:
        out.update({'A_pair_batches': sorted(set(equal_ref)), 'A_missing_reference_batches': sorted(set(missing_ref)),
                    'B_batches': sorted(not_created), 'C_holder_batches': sorted(set(posted_against))})
    print(json.dumps(out, indent=1))


main()
