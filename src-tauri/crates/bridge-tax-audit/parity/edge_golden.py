# SPDX-License-Identifier: Apache-2.0
"""Produce the reference Python implementation's canonical dumps for one edge book.

An edge book (tests/fixtures/edge-books/NAME.json) is a small invented book, written by hand to
reach a branch or boundary the synthetic read does not: a period, group masters, ledgers with
their chains, Trial Balance rows, vouchers, the engagement's cash and bank ledgers and
own-account narration terms, and the tests to run on it. It is not a Tally read -- the book is
built directly -- so it proves the Rust port and the reference agree on the same book, and
nothing about reading Tally.

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

writes golden/edge.NAME.TEST.json for each test the book names -- the reference's own canonical
dump (`tae.parity.canonical.canonical_test_result`), module invariant included -- and, for
`trial_balance`, golden/edge.NAME.trial_balance.order.json: the order in which the reference emits
its per-ledger rows, which the canonical dump (sorted by id) does not show. `tests/edge_books.rs`
builds the same book in Rust and compares whole dumps. Python 3.13 is pinned because its Unicode
tables (15.1.0) are the ones the crate's case mapping reproduces (`src/support.rs`).

Spec keys: `period` ([start, end], ISO; default the AY 2026-27 previous year), `groups` ({name:
parent or null}), `ledgers` ([{name, chain, guid, chain_complete?}], `chain_complete` a boolean, absent
meaning true), `tb` ([{ledger, opening, debit, credit,
closing}]), `vouchers` ([{guid, date, base_type, vtype?, number?, reference?, status?, narration?,
masterid?, inventory?, lines: [[ledger, paise], ...]}]; `number` defaults to the GUID, so pass `""`
to test a voucher with no number; `reference` is text, absent meaning ""; `masterid` is text, absent
meaning none; `inventory` is [{item, qty?, rate?,
amount?, direction?, qty_field_present?}], `qty` a number read as a float, `rate`/`amount` integer
paise (debit positive), `direction` 1 or -1, each absent or null meaning None, `qty_field_present` a
boolean defaulting to true when absent; any other type is refused, here and in
`tests/edge_books.rs`), `cash`, `bank`, `own_account_terms`, `counter_cheque_terms` (a list of text, default []; the
`[roles].counter_cheque_narration_terms` of `counter_cheques_40a3`), for `applicability_44ab`: `a44_turnover` (the reference's `turnover_inputs`
dict as written there), `a44_cash_share` ({receipts_bp, payments_bp, limits}), `presumptive_history` (a table of
text, integers and booleans, absent meaning none) and `deductor_activity` (absent meaning not recorded), with
`entity_type` as for `tds_payees`, `rules_without` (top-level rules tables
to drop, e.g. ["ledger_scrutiny"]; the Rust side must map each one, see `tests/edge_books.rs`),
`creditors` (the trade-creditor ledger names `creditor_ageing_43bh` ages), `creditor_ageing`
({acceptance_lag_days?, supplier_classification?, mse_interest_ledgers?, post_year_payments?: {ledger:
[[ISO date, paise], ...]}}; each key defaults as the reference's `run()` defaults it),
`statutory_dues` ({nature_by_ledger?, salary_expense_ledgers?}), `tests`; per voucher `party`
(PARTYLEDGERNAME, default ""); for `tds_payees`: `entity_type` (default "individual"),
`nature_by_ledger`, `payee_aliases`, `s194j_category_by_ledger` (each default {}) and
`previous_year_turnover_paise` (default absent), and the inputs the reference's pack reads from the client
config, each through the reference's own `tae.config` reader (so each is refused as a client config would be):
`reversals`, `gst_separate_by_agreement`, `foreseeability` (the `[tds_payees]` keys), `challans` (as
`[[tds.challans]]`, each `date` an ISO date string), `form_26a`, `previous_year_turnover_status` and
`goods_carriage_ledgers` (the `[tds]` keys), `deductor_activity` (`[deductor].activity`), `client_state`
(`[client].state`), `tds_payable_ledgers` (the ledgers `[statutory_dues]` classifies as TDS payable),
`gst_ledgers` (the `[roles].tax_ledgers` ledgers) and the keys of `partners`; and for `tds_tcs_26as`/`twentysixas_receipts`:
`form26as`, `ais`, `tis` (invented document rows in the shape `parity/python_golden.py
--emit-traces-documents` writes; default []) and `tds_ledgers`, `tcs_ledgers`,
`advance_tax_ledgers`, `deductor_aliases` (default empty); and for `loans_interest`: `entity_type` and
`previous_year_turnover_paise`, `previous_year_turnover_status`, `deductor_activity` and
`tds_payable_ledgers` as for `tds_payees`, `loans` ({loan ledger: {lender, lender_type,
interest_ledger?}}, the interest ledger one name or a list; default {}), `shared_interest_ledgers` (default []) and `net_reversals` (a boolean,
default false: true sets the module's NET_REVERSALS switch, reaching the dormant reversal rule in `run` and
in the module invariant alike); and for `partners_40b_194t`: `entity_type` as for `tds_payees`, `partners`
({key: {capital_ledgers, interest_ledger?, remuneration_ledger?}}, default {}), `deed` (a table such as
{interest_rate_bp}, absent meaning none) and `tds_payable_ledgers` (as for `tds_payees`); and for `bank_reconciliation`: `bank_statement` (an invented
statement in the shape `parity/python_golden.py --emit-bank-statement` writes, either balance null where the
statement declares none), `bank_reconciliation_ledger`
and `bank_charge_terms` (default []); the statement's rows also feed the module invariant, as the
reference's pack sets `eng.bank`; a statement the reference's own reader refuses gives the module's
`refused` result, with the reader's reason, as the pack does; and for `high_value_register`: `bank_statement` (optional here, absent
meaning none supplied, and none supplied with the reader's reason when that reader refuses it), `ais` as above, `s194n_terms` and `round_off_ledgers` (default []),
`bank_statement_refused` (the reader's plain-words reason a supplied statement was refused, with no `bank_statement`; default none) and `counterparty_types` ({ledger: type}, the map pack.py builds from the loan ledgers and
`[roles].counterparty_type_by_ledger`; default {}) and `s194n_recipient_type` (one of the module's two
recipient constants or "unknown"; absent meaning derived from `entity_type` as pack.py derives it); and
for `cash_payments_40a3`: `loan_ledgers` and `round_off_ledgers` (default []); for `entity_269st_gap`: `party_identity` (the engagement's own
[party_identity] table, default {}), `round_off_ledgers`, and per ledger `pan` and `gstin` (default ""); for `read_scope`: `currency_read` (default false); for `books_examined`: `documents_read` (the
names of the documents the pack loaded, in its order: a list of text, default []); for `clause21a_candidates`:
`clause21a_extra_terms` (the client's `[clause21a].extra_terms`, read by the reference's own reader; absent meaning
none) and `partners` (as for `partners_40b_194t`; its interest and remuneration ledgers, as pack.py takes them); and
for `stock`: `stock_items` ({name: {base_unit?, guid?, opening_qty?, opening_value?, closing_qty?,
closing_value?}}, default {}), `stock_opening` and `stock_closing` ({as_of, rows: {name: {qty?, value?,
rate?}}}), each quantity a number, each value or rate integer paise, absent or null meaning None, and
`is_integrated` (true, false, or absent/null for unknown); and for `party_monthly`: `cash`, `bank` (both also
passed to the module invariant, as the reference's pack passes them) and `period` as above, and `top_n` (a non-negative integer, default the module's PARTY_TOP_N; Python would slice
a negative one from the end, which the Rust `usize` cannot express, so both sides refuse it); and for
`narration_payees`: `narration_payee_ledgers` (a list of text, default []), `bank` (also passed to the
module invariant) and the keys `tds_payees` reads, whose result gives the 194C ledgers it adds.
"""
from __future__ import annotations

import copy
import json
import sys
import tempfile
from types import SimpleNamespace
from datetime import date
from pathlib import Path

STATUS = ("regular", "optional", "cancelled", "postdated")


def main() -> int:
    engine, spec_path, out_dir = sys.argv[1], Path(sys.argv[2]), Path(sys.argv[3])
    sys.path.insert(0, str(Path(engine).resolve()))
    from tae.adapters.bank_documents import (BankStatementDoc, StatementBalanceMissing, StatementOrderRefused,
                                             StatementPeriodRefused, load_bank_statement_json)
    from tae.adapters.tally_stock import StockItemMaster, StockSnapshot, StockSnapshotRow
    from tae.adapters.traces_documents import AisRow, TisRow
    from tae.audit_tests import (applicability_44ab, bank_reconciliation, book_keeping_quality, books_examined, cash_book_integrity, cash_payments_40a3, clause21a_candidates, counter_cheques_40a3,
                                 creditor_ageing_43bh, entity_269st_gap, high_value_register, ledger_scrutiny, loans_interest, partners_40b_194t, party_monthly, read_scope, related_parties_cl23, specified_persons_40a2b, stale_balances_41_1,
                                 statutory_dues_43b, stock, tds_payees, tds_tcs_26as, trial_balance, twentysixas_receipts)
    from tae.model import Form26ASRow
    from tae.config import load_rules, related_parties_config
    from tae.model import (BankStatementRow, Book, Engagement, Group, InventoryLine, Ledger, LedgerLine, Period,
                           TBRow, Voucher, VoucherStatus)
    from tae.parity import canonical

    spec = json.loads(spec_path.read_text(encoding="utf-8"))
    name = spec_path.stem
    status = {s: getattr(VoucherStatus, s.upper()) for s in STATUS}
    start, end = spec.get("period", ["2025-04-01", "2026-03-31"])
    groups = {n: Group(name=n, parent=p) for n, p in spec["groups"].items()}
    # Typed strictly, the same way tests/edge_books.rs reads them, so that a mistyped key fails on
    # both sides instead of building two different books.
    def typed(d, key, ok, what, absent=None, nullable=True):
        if key not in d or (nullable and d[key] is None):
            return absent
        if not ok(d[key]):
            raise SystemExit(f"{spec_path.name}: {key} must be {what}, got {d[key]!r}")
        return d[key]

    ledgers = {l["name"]: Ledger(name=l["name"], parent=l["chain"][0] if l["chain"] else "",
                                 chain=tuple(l["chain"]),
                                 chain_complete=typed(l, "chain_complete", lambda x: isinstance(x, bool),
                                                      "true or false", absent=True, nullable=False),
                                 guid=l.get("guid", ""),
                                 pan=typed(l, "pan", lambda x: isinstance(x, str), "text", absent="", nullable=False),
                                 gstin=typed(l, "gstin", lambda x: isinstance(x, str), "text", absent="", nullable=False))
               for l in spec["ledgers"]}

    def integer(x):
        return isinstance(x, int) and not isinstance(x, bool)

    def inventory_line(i):
        qty = typed(i, "qty", lambda x: integer(x) or isinstance(x, float), "a number or null")
        if not isinstance(i.get("item"), str):
            raise SystemExit(f"{spec_path.name}: item must be text, got {i.get('item')!r}")
        return InventoryLine(item=i["item"],
                             qty=None if qty is None else float(qty),
                             rate_paise=typed(i, "rate", integer, "an integer or null"),
                             amount_paise=typed(i, "amount", integer, "an integer or null"),
                             direction=typed(i, "direction", lambda x: x in (1, -1) and integer(x), "1, -1 or null"),
                             qty_field_present=typed(i, "qty_field_present", lambda x: isinstance(x, bool),
                                                     "true or false", absent=True, nullable=False))

    vouchers = [Voucher(guid=v["guid"], masterid=typed(v, "masterid", lambda x: isinstance(x, str), "text", nullable=False), alterid=None, date=date.fromisoformat(v["date"]),
                        vtype=v.get("vtype", v["base_type"]), base_type=v["base_type"],
                        number=v.get("number", v["guid"]), reference=typed(v, "reference", lambda x: isinstance(x, str), "text", absent="", nullable=False),
                        party_field=v.get("party", ""), party_gstin="",
                        narration=v.get("narration", ""), status=status[v.get("status", "regular")],
                        status_source="edge-book",
                        lines=tuple(LedgerLine(ledger=l, amount_paise=a) for l, a in v["lines"]),
                        inventory=tuple(inventory_line(i) for i in v.get("inventory", ())))
                for v in spec["vouchers"]]
    tb = {t["ledger"]: TBRow(ledger=t["ledger"], opening_paise=t["opening"], debit_paise=t["debit"],
                             credit_paise=t["credit"], closing_paise=t["closing"]) for t in spec["tb"]}
    book = Book(company_name="Invented edge book",
                period=Period(date.fromisoformat(start), date.fromisoformat(end)), groups=groups,
                ledgers=ledgers, vouchers=vouchers, tb=tb, company_guid="invented-edge-company",
                currency_read=typed(spec, "currency_read", lambda x: isinstance(x, bool), "true or false",
                                    absent=False, nullable=False))
    entity_type = spec.get("entity_type", "individual")
    eng = Engagement(entity_type, "2026-27", book)
    rules = load_rules("2026-27", entity_type)
    for table in spec.get("rules_without", []):
        rules = copy.copy(rules)
        rules.pop(table)
    cash, bank = set(spec.get("cash", [])), set(spec.get("bank", []))
    day = lambda s: None if s is None else date.fromisoformat(s)
    form26as = [Form26ASRow(**{**r, "txn_date": day(r["txn_date"])}) for r in spec.get("form26as", [])]
    ais = [AisRow(**{**r, "txn_date": day(r["txn_date"])}) for r in spec.get("ais", [])]
    tis = [TisRow(**r) for r in spec.get("tis", [])]
    eng.form26as = form26as
    aliases = dict(spec.get("deductor_aliases", {}))
    terms = frozenset(spec.get("own_account_terms", []))
    counter_cheque_terms = typed(spec, "counter_cheque_terms",
                                 lambda x: isinstance(x, list) and all(isinstance(t, str) for t in x),
                                 "a list of text", absent=[], nullable=False)
    documents_read = typed(spec, "documents_read",
                           lambda x: isinstance(x, list) and all(isinstance(t, str) for t in x),
                           "a list of text", absent=[], nullable=False)
    ca = spec.get("creditor_ageing", {})
    sd = spec.get("statutory_dues", {})
    post_year = {k: [(date.fromisoformat(d), a) for d, a in v] for k, v in ca.get("post_year_payments", {}).items()}

    def loans_interest_run():
        # The switch is a module global the reference's run() and check_invariants() both read; the
        # canonical dump below calls check_invariants in this same process, before anything resets it.
        loans_interest.NET_REVERSALS = typed(spec, "net_reversals", lambda x: isinstance(x, bool), "true or false",
                                             absent=False, nullable=False)
        # As tae/pack.py passes them: the deductor activity and the turnover's status through the
        # reference's own readers, from a config built out of the spec's keys.
        from tae import config as tc
        cfg = {"tds": {k: spec[k] for k in ("previous_year_turnover_status",) if k in spec},
               "deductor": {"activity": spec["deductor_activity"]} if "deductor_activity" in spec else {}}
        return loans_interest, loans_interest.run(
            eng, rules, {k: dict(v) for k, v in spec.get("loans", {}).items()},
            spec.get("previous_year_turnover_paise"), cash, bank,
            frozenset(spec.get("shared_interest_ledgers", [])),
            tds_payable_ledgers=frozenset(spec.get("tds_payable_ledgers", [])),
            deductor_activity=tc.deductor_activity(cfg), turnover_is_placeholder=tc.turnover_is_placeholder(cfg))

    # One runner per test an edge book may name: the module and its result, run as the reference's
    # pack runs it.
    # book_keeping_quality's inputs, as tae/pack.py passes them: tax_ledgers flattened to
    # {ledger: head} in document order, as tae.config.tax_ledgers_by_head flattens it.
    bkq = spec.get("book_keeping_quality", {})
    bkq_tax = {ledger: head for head, ledgers in bkq.get("tax_ledgers", {}).items() for ledger in ledgers}

    def bank_statement(bs):
        rows = tuple(BankStatementRow(**{**r, "txn_date": date.fromisoformat(r["txn_date"])}) for r in bs["rows"])
        return BankStatementDoc(
            doc_id=bs["doc_id"], source_sha256=bs["source_sha256"], account_ref=bs["account_ref"],
            bank=bs["bank"], period=Period(date.fromisoformat(bs["period"]["start"]),
                                           date.fromisoformat(bs["period"]["end"])),
            opening_balance_paise=bs["opening_balance_paise"], closing_balance_paise=bs["closing_balance_paise"],
            rows=rows)

    def statement_refused(bs):
        """As tae/pack.py reads a statement: the reference's own reader, on the same statement in the extraction's
        shape (amounts as rupee text, an absent balance as null), and its plain-words reason when it refuses one (no
        declared balance, a row outside the period, a date that steps back with no running balance to confirm it).
        None when it reads it, after checking it read the rows the spec gives."""
        def rupees(p):
            return None if p is None else f"{'-' if p < 0 else ''}{abs(p) // 100}.{abs(p) % 100:02d}"
        raw = {"account_no_masked": bs["account_ref"], "bank": bs["bank"],
               "period_from": bs["period"]["start"], "period_to": bs["period"]["end"],
               "opening_balance": rupees(bs.get("opening_balance_paise")),
               "closing_balance": rupees(bs.get("closing_balance_paise")),
               "transactions": [{"date": r["txn_date"], "description": r["narration"], "debit": rupees(r["debit_paise"]),
                                 "credit": rupees(r["credit_paise"]), "balance": rupees(r["balance_paise"])}
                                for r in bs["rows"]]}
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "statement.json"
            path.write_text(json.dumps(raw), encoding="ascii")
            try:
                doc = load_bank_statement_json(path, "edge")
            except (StatementBalanceMissing, StatementOrderRefused, StatementPeriodRefused) as e:
                return e.reason
        read = [(r.txn_date.isoformat(), r.narration, r.debit_paise, r.credit_paise, r.balance_paise) for r in doc.rows]
        given = [(r["txn_date"], r["narration"], r["debit_paise"], r["credit_paise"], r["balance_paise"])
                 for r in bs["rows"]]
        if (read, doc.opening_balance_paise, doc.closing_balance_paise) != (
                given, bs["opening_balance_paise"], bs["closing_balance_paise"]):
            raise SystemExit(f"{spec_path.name}: the reference's reader read another statement than the spec gives")
        return None

    def bank_reconciliation_run():
        # As tae/pack.py: a statement its reader refuses is reconciled as refused, with the reason; otherwise the
        # statement is caller data, and eng.bank carries its rows for BANK-1.
        reason = statement_refused(spec["bank_statement"])
        if reason is not None:
            return bank_reconciliation, bank_reconciliation.refused(eng, rules, reason)
        statement = bank_statement(spec["bank_statement"])
        eng.bank = list(statement.rows)
        return bank_reconciliation, bank_reconciliation.run(
            eng, rules, statement, spec["bank_reconciliation_ledger"],
            bank_charge_narration_terms=set(spec.get("bank_charge_terms", [])))

    def high_value_register_run():
        # As tae/pack.py: the statement and the AIS rows are optional; the counterparty types are
        # given already merged; the recipient type follows entity_type as pack.py maps it, unless
        # the spec names one ("unknown" meaning none).
        bs = spec.get("bank_statement")
        refused = spec.get("bank_statement_refused")
        # As tae/pack.py: a statement its reader refuses is not supplied, and the reader's reason is passed
        if bs is not None and (reason := statement_refused(bs)) is not None:
            if refused is not None:
                raise SystemExit(f"{spec_path.name}: bank_statement_refused is given and the reader refuses the statement")
            bs, refused = None, reason
        recipient = spec.get("s194n_recipient_type")
        if recipient is None:
            recipient = (high_value_register.RECIPIENT_NOT_CO_OPERATIVE
                         if entity_type in ("individual", "huf", "firm", "llp", "company")
                         else high_value_register.RECIPIENT_CO_OPERATIVE
                         if entity_type == "cooperative_society" else None)
        elif recipient == "unknown":
            recipient = None
        elif recipient not in (high_value_register.RECIPIENT_CO_OPERATIVE,
                               high_value_register.RECIPIENT_NOT_CO_OPERATIVE):
            raise SystemExit(f"{spec_path.name}: s194n_recipient_type {recipient!r} is not a recipient type")
        return high_value_register, high_value_register.run(
            eng, rules, cash, bank, bank_statement=None if bs is None else bank_statement(bs),
            s194n_narration_terms=frozenset(spec.get("s194n_terms", [])), ais_rows=ais,
            s194n_recipient_type=recipient, round_off_ledgers=frozenset(spec.get("round_off_ledgers", [])),
            counterparty_type_by_ledger=dict(spec.get("counterparty_types", {})),
            bank_statement_refused=refused)

    def stock_run():
        # As tae/pack.py: invented masters and both Stock Summaries, typed strictly as
        # tests/edge_books.rs reads them; STK-1 gets them bound, as pack.py passes them.
        number = lambda x: integer(x) or isinstance(x, float)
        text = lambda x: isinstance(x, str)

        def qty(d, key):
            q = typed(d, key, number, "a number or null")
            return None if q is None else float(q)

        items = {n: StockItemMaster(name=n, guid=typed(m, "guid", text, "text", absent="", nullable=False), parent="",
                                     base_unit=typed(m, "base_unit", text, "text", absent="", nullable=False),
                                     opening_qty=qty(m, "opening_qty"),
                                     opening_value_paise=typed(m, "opening_value", integer, "an integer or null"),
                                     closing_qty=qty(m, "closing_qty"),
                                     closing_value_paise=typed(m, "closing_value", integer, "an integer or null"))
                 for n, m in spec.get("stock_items", {}).items()}

        def snapshot(key):
            s = spec[key]
            return StockSnapshot(date.fromisoformat(s["as_of"]), {
                n: StockSnapshotRow(name=n, guid="", qty=qty(r, "qty"),
                                    value_paise=typed(r, "value", integer, "an integer or null"),
                                    rate_paise=typed(r, "rate", integer, "an integer or null"))
                for n, r in s["rows"].items()})

        opening, closing = snapshot("stock_opening"), snapshot("stock_closing")
        module = SimpleNamespace(TEST_ID=stock.TEST_ID, check_invariants=lambda e, res: stock.check_invariants(
            e, res, items, closing, opening_snapshot=opening))
        integrated = typed(spec, "is_integrated", lambda x: isinstance(x, bool), "true, false or null")
        return module, stock.run(eng, {"version": rules.version}, items, opening, closing, integrated)

    def clause21a_candidates_run():
        # As tae/pack.py: the client's [clause21a].extra_terms through the reference's own reader, from a config
        # built out of the spec's key, and the partners' interest and remuneration ledgers, each kept when truthy;
        # on the crate's vendored keyword table, checked against the reference's.
        from python_golden import require_vendored_clause21a_keywords
        from tae.config import clause21a_extra_terms
        require_vendored_clause21a_keywords(clause21a_candidates)
        cfg = {"clause21a": {"extra_terms": spec["clause21a_extra_terms"]}} if "clause21a_extra_terms" in spec else {}
        partners = spec.get("partners", {})
        return clause21a_candidates, clause21a_candidates.run(
            eng, rules, clause21a_extra_terms(cfg),
            frozenset(led for q in partners.values() for led in (q.get("interest_ledger"), q.get("remuneration_ledger"))
                      if led))

    def tds_payees_run():
        # As tae/pack.py's _tds_payees: every client-config input through the reference's own reader, from a
        # config built out of the spec's keys.
        from tae import config as tc
        tds_tbl = {k: spec[k] for k in ("nature_by_ledger", "previous_year_turnover_status", "goods_carriage_ledgers",
                                        "form_26a") if k in spec}
        if "challans" in spec:
            tds_tbl["challans"] = [{**c, "date": date.fromisoformat(c["date"])} if isinstance(c.get("date"), str) else c
                                   for c in spec["challans"]]
        cfg = {"tds": tds_tbl,
               "tds_payees": {k: spec[k] for k in ("reversals", "gst_separate_by_agreement", "foreseeability")
                              if k in spec},
               "client": {"state": spec["client_state"]} if "client_state" in spec else {},
               "deductor": {"activity": spec["deductor_activity"]} if "deductor_activity" in spec else {}}
        return tds_payees, tds_payees.run(
            eng, rules, dict(spec.get("nature_by_ledger", {})), dict(spec.get("payee_aliases", {})),
            spec.get("previous_year_turnover_paise"), dict(spec.get("s194j_category_by_ledger", {})),
            reversals=tc.tds_payees_reversals(cfg), gst_separate=tc.tds_payees_gst_separate(cfg),
            gst_ledgers=frozenset(spec.get("gst_ledgers", [])), tds_ledgers=frozenset(spec.get("tds_payable_ledgers", [])),
            foreseeability_names=frozenset(tc.tds_payees_foreseeability(cfg)),
            other_names=frozenset(spec.get("partners", {})), challans=tc.tds_challans(cfg), form_26a=tc.tds_form_26a(cfg),
            client_state=tc.client_state(cfg), turnover_is_placeholder=tc.turnover_is_placeholder(cfg),
            deductor_activity=tc.deductor_activity(cfg), goods_carriage_ledgers=tc.tds_goods_carriage_ledgers(cfg))

    def narration_payees_run():
        from tae.audit_tests import narration_payees
        configured = frozenset(typed(spec, "narration_payee_ledgers",
                                     lambda x: isinstance(x, list) and all(isinstance(t, str) for t in x),
                                     "a list of text", absent=[], nullable=False))
        added = narration_payees.unnamed_194c_ledgers(book, tds_payees_run()[1], dict(spec.get("nature_by_ledger", {})))
        module = SimpleNamespace(TEST_ID=narration_payees.TEST_ID, check_invariants=lambda e, res:
                                 narration_payees.check_invariants(e, res, bank, configured | added))
        return module, narration_payees.run(eng, rules, bank, configured, added_ledgers=added)

    def specified_persons_run():
        # As the pack's runner: related_parties_cl23 on the same table first, its result this test's input.
        rp = related_parties_config({"related_parties": spec.get("related_parties", {})})
        cl23 = related_parties_cl23.run(eng, rules, rp)
        module = SimpleNamespace(TEST_ID=specified_persons_40a2b.TEST_ID, check_invariants=lambda e, res:
                                 specified_persons_40a2b.check_invariants(e, rules, rp, cl23, res))
        return module, specified_persons_40a2b.run(eng, rules, rp, cl23)

    from tae.party_identity import build_party_index
    runners = {
        "applicability_44ab": lambda: (applicability_44ab, applicability_44ab.run(
            eng, rules, dict(spec.get("a44_turnover", {})), dict(spec.get("a44_cash_share", {})),
            spec.get("presumptive_history"), deductor_activity=spec.get("deductor_activity"))),
        "bank_reconciliation": bank_reconciliation_run,
        "book_keeping_quality": lambda: (book_keeping_quality, book_keeping_quality.run(
            eng, rules, cash, set(bkq.get("payment_channel_debtors", [])), bkq_tax,
            set(bkq.get("gst_payment_ledgers", [])), list(bkq.get("reissue_narration_terms", [])),
            set(bkq.get("writeoff_discount_ledgers", [])))),
        # As tae/pack.py: the documents loaded, in the pack's order, as a tuple.
        "books_examined": lambda: (books_examined, books_examined.run(eng, rules, tuple(documents_read))),
        "cash_book_integrity": lambda: (cash_book_integrity,
                                        cash_book_integrity.run(eng, rules, cash, bank, terms)),
        # As tae/pack.py: the loan ledgers the client configured and the round-off ledgers, as given.
        "cash_payments_40a3": lambda: (cash_payments_40a3, cash_payments_40a3.run(
            eng, rules, cash=cash, bank=bank, loan_ledgers_configured=set(spec.get("loan_ledgers", [])),
            round_off_ledgers=frozenset(spec.get("round_off_ledgers", [])))),
        "clause21a_candidates": clause21a_candidates_run,
        "counter_cheques_40a3": lambda: (counter_cheques_40a3, counter_cheques_40a3.run(
            eng, rules, cash, bank, frozenset(counter_cheque_terms))),
        "creditor_ageing_43bh": lambda: (creditor_ageing_43bh, creditor_ageing_43bh.run(
            eng, rules, set(spec.get("creditors", [])), acceptance_lag_days=ca.get("acceptance_lag_days", 0),
            supplier_classification=ca.get("supplier_classification", {}), post_year_payments=post_year,
            mse_interest_ledgers=frozenset(ca.get("mse_interest_ledgers", [])))),
        # As tae/pack.py: the party index is built from the engagement's [party_identity] table, and
        # the round-off ledgers are passed as given.
        "entity_269st_gap": lambda: (entity_269st_gap, entity_269st_gap.run(
            eng, rules, cash, bank, build_party_index(book, {"party_identity": spec.get("party_identity", {})}),
            round_off_ledgers=frozenset(spec.get("round_off_ledgers", [])))),
        "high_value_register": high_value_register_run,
        "ledger_scrutiny": lambda: (ledger_scrutiny, ledger_scrutiny.run(eng, rules, cash)),
        "loans_interest": loans_interest_run,
        "narration_payees": narration_payees_run,
        "partners_40b_194t": lambda: (partners_40b_194t, partners_40b_194t.run(
            eng, rules, {k: dict(v) for k, v in spec.get("partners", {}).items()}, spec.get("deed"),
            tds_ledgers=frozenset(spec.get("tds_payable_ledgers", [])))),
        # As tae/pack.py: PWM-2 takes the cash and bank ledgers run() takes, so they are bound here.
        "party_monthly": lambda: (SimpleNamespace(TEST_ID=party_monthly.TEST_ID, check_invariants=lambda e, res:
                                                  party_monthly.check_invariants(e, res, cash, bank)),
                                  party_monthly.run(
            eng, rules, cash, bank,
            top_n=typed(spec, "top_n", lambda x: integer(x) and x >= 0, "a non-negative integer",
                        absent=party_monthly.PARTY_TOP_N, nullable=False))),
        "read_scope": lambda: (read_scope, read_scope.run(eng, rules)),
        "related_parties_cl23": lambda: (related_parties_cl23, related_parties_cl23.run(
            eng, rules, related_parties_config({"related_parties": spec.get("related_parties", {})}))),
        "specified_persons_40a2b": specified_persons_run,
        "stale_balances_41_1": lambda: (stale_balances_41_1, stale_balances_41_1.run(eng, rules)),
        "statutory_dues_43b": lambda: (statutory_dues_43b, statutory_dues_43b.run(
            eng, rules, dict(sd.get("nature_by_ledger", {})), frozenset(sd.get("salary_expense_ledgers", [])))),
        "stock": stock_run,
        "tds_payees": tds_payees_run,
        "tds_tcs_26as": lambda: (tds_tcs_26as, tds_tcs_26as.run(
            eng, rules, form26as=form26as, ais_rows=ais, tis_rows=tis,
            tds_ledgers=set(spec.get("tds_ledgers", [])), tcs_ledgers=set(spec.get("tcs_ledgers", [])),
            deductor_aliases=aliases, advance_tax_ledgers=set(spec.get("advance_tax_ledgers", [])))),
        "trial_balance": lambda: (trial_balance, trial_balance.run(eng, rules)),
        "twentysixas_receipts": lambda: (twentysixas_receipts, twentysixas_receipts.run(eng, rules, aliases)),
    }
    for test in spec["tests"]:
        if test not in runners:
            raise SystemExit(f"{spec_path}: no edge runner for test {test!r}")
        module, result = runners[test]()
        if test == "trial_balance":
            order = [fid for fid in result.figures if fid.startswith("trial_balance.tb_group_")]
            (out_dir / f"edge.{name}.trial_balance.order.json").write_text(
                json.dumps(order, indent=1) + "\n", encoding="utf-8")
        doc = canonical.canonical_test_result(eng, result, module)
        out = out_dir / f"edge.{name}.{test}.json"
        out.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print(f"{out}: {len(doc['figures'])} figures, {len(doc['findings'])} findings, "
              f"{len(doc['module_invariant_violations'])} module invariant violations")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
