# Lab helper for item 1738-COUNTS (not product code). Reads one copy of ComplyEaze Bridge's import journal
# (agent-import-ledger.jsonl) and, if given, its skip-only file (agent-import-statement-skips.jsonl), and prints
# counts as JSON: candidate transfer double bookings, sends Tally answered as creating none of their vouchers, and
# skipped rows posted by a batch found in the book. It reads only the files named; it never writes them, opens no
# network connection and sends nothing to Tally. It prints no reference, amount, date, ledger or company name: only
# counts and batch ids. Every count is an over-count of what ComplyEaze Bridge itself reports (conditions it drops are
# listed beside each count), so a zero is decisive and a non-zero is a lead to check with verify_import.
# Usage: python -I count_journals.py <journal copy> [<skips copy>]
import json
import sys

COUNTERS = ('created', 'altered', 'deleted', 'ignored', 'errors', 'cancelled', 'exceptions')


def read_lines(path):
    """Each line parsed as JSON; a line that does not parse is counted, never skipped silently."""
    rows, bad = [], 0
    with open(path, 'rb') as fh:
        for raw in fh:
            text = raw.decode('utf-8').strip()
            if not text:
                continue
            try:
                rows.append(json.loads(text))
            except ValueError:
                bad += 1
    return rows, bad


def canonical_amount(text):
    text = str(text).lstrip('-').lstrip('0') or '0'
    if '.' in text:
        text = text.rstrip('0').rstrip('.')
    return text or '0'


def main():
    journal = sys.argv[1]
    skips = sys.argv[2] if len(sys.argv) > 2 else None
    rows, bad = read_lines(journal)
    batches, statuses = {}, {}
    for row in rows:
        if 'record_type' in row:
            statuses.setdefault(row.get('batch_id'), []).append(row)
        else:
            batches[row.get('batch_id')] = row  # the latest line of a batch decides it
    sent = {b for b, s in statuses.items() if any(r.get('record_type') == 'dispatch_intent' for r in s)}
    found = {b for b, s in statuses.items()
             if any(r.get('record_type') == 'verification_status' and r.get('status') == 'posted_verified' for r in s)}
    rolled = {b for b, s in statuses.items() if any(r.get('record_type') == 'rollback_seen' for r in s)}

    def lineage(line):
        return line.get('amends_batch_id') or line.get('batch_id')

    # A. Contras that carry a statement reference, then every pair with the same company, endpoint, reference and
    # canonical amount, from two different statement banks and two different lineages. Dropped versus the product:
    # the ledger-pair grouping, the two-leg shape checks, the newest-build-in-lineage rule and the sent-elsewhere drop.
    contras = []
    for batch_id, line in batches.items():
        statement = line.get('statement') or {}
        refs = {r.get('bridge_txn_id'): r.get('reference') for r in statement.get('contra_rows') or [] if r.get('reference')}
        for voucher in line.get('vouchers') or []:
            if voucher.get('voucher_type') != 'Contra' or voucher.get('bridge_txn_id') not in refs:
                continue
            amounts = {canonical_amount(e.get('amount')) for e in voucher.get('entries') or []}
            contras.append({
                'batch': batch_id, 'lineage': lineage(line), 'company': str(line.get('company_guid', '')).lower(),
                'endpoint': line.get('endpoint_origin'), 'bank': statement.get('bank_ledger_guid'),
                'reference': refs[voucher.get('bridge_txn_id')], 'amount': min(amounts) if amounts else None,
            })
    pairs = []
    for i, a in enumerate(contras):
        for b in contras[i + 1:]:
            if (a['company'], a['endpoint'], a['reference'], a['amount']) == (b['company'], b['endpoint'], b['reference'], b['amount']) \
                    and a['bank'] != b['bank'] and a['lineage'] != b['lineage']:
                pairs.append((a['batch'], b['batch']))
    in_book = [p for p in pairs if all(x in sent or x in found for x in p)]

    # B. The latest dispatch answer with an outcome reads every counter present, created to cancelled 0, and the
    # exceptions equal to the vouchers sent (the product's not-created shape).
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

    # C. Skip rows (from statements and the skip-only file) that a batch of another lineage, found in the book and
    # not rolled back, holds as a voucher, with no withdrawal of the row recorded in that lineage after the skip's
    # newest mention. Dropped versus the product: a withdrawal counts here whether or not its build was sent.
    mentions = {}
    skip_sources = [(lineage(l), l.get('built_at') or '', l.get('statement') or {}, str(l.get('company_guid', '')).lower())
                    for l in batches.values()]
    skip_bad = 0
    if skips:
        skip_rows, skip_bad = read_lines(skips)
        skip_sources += [(None, r.get('built_at') or '', r.get('statement') or {}, str(r.get('company_guid', '')).lower())
                         for r in skip_rows]
    for own, built, statement, company in skip_sources:
        for row in statement.get('skip_rows') or []:
            key = (company, row.get('bridge_txn_id'))
            if key not in mentions or built > mentions[key][1]:
                mentions[key] = (own, built)
    posted_against = []
    for (company, txn), (own, built) in mentions.items():
        for batch_id, line in batches.items():
            if str(line.get('company_guid', '')).lower() != company or lineage(line) == own:
                continue
            if batch_id not in found or batch_id in rolled:
                continue
            if txn not in {v.get('bridge_txn_id') for v in line.get('vouchers') or []}:
                continue
            withdrawn = any(lineage(l) == lineage(line) and txn in (l.get('withdrawn_skips') or [])
                            and (l.get('built_at') or '') > built for l in batches.values())
            if not withdrawn:
                posted_against.append(batch_id)
                break

    print(json.dumps({
        'journal_lines': len(rows) + bad, 'lines_not_parsed': bad, 'skip_lines_not_parsed': skip_bad,
        'batches': len(batches), 'status_records': sum(len(s) for s in statuses.values()),
        'batches_sent': len(sent), 'batches_found_posted': len(found),
        'contras_with_reference': len(contras),
        'A_equal_reference_pairs': len(pairs), 'A_pairs_both_sent_or_found': len(in_book),
        'A_pair_batches': sorted(set(pairs)),
        'B_not_created_sends': len(not_created), 'B_batches': sorted(not_created),
        'C_skips_posted_by_a_found_batch_unwithdrawn': len(posted_against), 'C_holder_batches': sorted(set(posted_against)),
    }, indent=1))


main()
