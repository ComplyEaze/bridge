# Batch D2a fixture provenance: `partners_40b_194t`

Lane B, 2026-09-23. Every book here is invented: no fixture is a Tally read of any real assessee, and every
partner and ledger name is invented ("A Capital", "Interest to Partners", ...).

`partners_40b_194t` is ported only as `tds_interest_201`'s input (orchestrator's exception, 23-Sep-2026);
`tds_interest_201` itself follows in batch D2b, once the reference's own fix to its rows lands:
on the three real books it is real work on one (a firm); the other two are not firms or LLPs. Its parity
therefore rests on one real book, with its own edge books covering the not-applicable path and every
branch the real book does not reach. This bends the two-book bar; the owner may veto it.

## What these fixtures establish, and what they do not

- `golden/synthetic.partners_40b_194t.json` is the reference's dump on the synthetic read, an
  individual: one figure, `applicable` = "no".
- `edge-books/partners_firm_deed.json` (a firm with a deed below the statutory cap): a partner whose
  capital goes into debit for part of the year (those days count zero); a withdrawal on the first day; a
  transfer between two partners' capital; one voucher carrying both the interest and the remuneration
  ledger (read as interest); an interest journal with a TDS line under Duties & Taxes (seen) and one
  without (not seen), the latter with no voucher number; a partner with one capital ledger and no
  interest or remuneration ledger (it named its capital ledger twice until the reference began refusing
  that); an optional voucher on a capital ledger, outside the population; s.40(b) not computed for any
  partner (list-only), and an s.194T finding.
- `edge-books/partners_llp_no_deed.json` (an LLP, no deed, rules without `[s194t]`): the deed-missing
  finding, the statutory cap used as the rate, the module's own s.194T default (flagged in the finding's
  limits), s.40(b) not computed (list-only), a partner credited exactly the s.194T limit (not
  over it, so no finding), and a "TDS" ledger that is not under Duties & Taxes (so no TDS line is seen).
- `edge-books/partners_not_applicable.json` (a company, with partners and a deed configured): one
  figure, `applicable` = "no".
- Not reached here: a partner without `capital_ledgers`, a deed that is not a table or whose rate is not
  an integer, and rules without `[entity]`: each is refused (the reference raises), and unit tests cover
  them, as no golden can.

- `edge-books/partners_tds_mixed.json` (a firm, four partners, two sharing an interest ledger; added at the re-pin
  below): TDS on a partner's interest added back; an interest-and-remuneration voucher split exactly and one
  that also credits the bank, not split; an interest voucher that also debits the capital; vouchers on the
  shared interest ledger touching no capital, one carrying TDS attributed to no one; interest over the
  allowable; remuneration whose TDS is shared with another credited party; a reversal against the gross
  s.194T base. s.40(b) is computed for no partner (list-only); each partner's reasons are listed.
- `edge-books/partners_deed_no_interest.json` (a firm whose deed records `no_interest_authorised`; added at the
  list-only re-pin below): the rate read as nil with its own definition and no deed question; one partner with an
  interest ledger (its interest counted, and asked for as all disallowed) and one without (its own title, no
  s.40(b) question); the firm total withheld.

## Re-pin at list-only and the no-interest deed (Lane E3, 2026-09-28)

At `01b6f5d8dfd526474d32335778f6cada5dc23c85` (brain main) the reference computes the s.40(b) excess for no partner and asks for each
(`S40B_EXCESS_COMPUTED = False`), withholds the firm total, reads only plain forward interest vouchers, names a
credit to a ledger it does not read, refuses a partner configuration with an empty or repeated capital list, a
capital ledger shared by two partners, a ledger missing from the books or one ledger in two roles, and reads the
deed's `no_interest_authorised` flag. `partners_firm_deed.json` no longer names a capital ledger twice (the
reference now refuses it) and two book comments follow the list-only rule; `partners_llp_no_deed` and
`partners_not_applicable` are unchanged books. The goldens of `partners_firm_deed`, `partners_llp_no_deed`,
`partners_tds_mixed` and the new `partners_deed_no_interest` are regenerated at that commit, from an archive of its
engine source only; `synthetic.partners_40b_194t.json` and `edge.partners_not_applicable.partners_40b_194t.json`
regenerate byte-identical there.

## Re-pin at the reference's current head (Lane E2, 2026-09-27)

The two changed goldens and the new book's golden were produced at the reference engine (a private
repository), commit `e2456bcf4f163cf770945e8620e715788db0ca46`, from an archive of its engine source only, with
the invocations below. The reference's `partners_40b_194t.py` last changed there at `9fec3492`: TDS added back per
partner, the exact-or-unsplit mixed voucher, capital set-offs and off-capital vouchers, the s.40(b) not-computed
path, the gross s.194T base, TDS read through the ledgers classified as TDS payable, and the s.40(b)(v) slab
quoted from `[s40b_v]` (vendored at the same head). `synthetic.partners_40b_194t.json` and
`edge.partners_not_applicable.partners_40b_194t.json` regenerate byte-identical.

After the review round changed `edge-books/partners_tds_mixed.json` (a balanced Trial Balance, a capital
debit on the unsplit mixed voucher, and a partner with no TDS seen), its golden was regenerated at
`87e2f03e0f5687692c638c17b66b92341c99534f`. Between `e2456bcf` and that commit the reference's engine source
changes only `loans_interest.py` and `party_monthly.py`, neither of which `partners_40b_194t` imports.

## Real books (local only; nothing from them is in this repository)

`examples/local_parity` compared the port with the reference at `1038dc05` on three real client reads,
each with that client's own reference-engine config: byte-identical dumps on all three; on the firm, 26
figures and 6 findings (two partners, with their capital, interest and remuneration ledgers bound by
identity); on the other two, `applicable` = "no" alone. Binding every `[partners.*]` location changed no
other test on the firm: all fifteen other ported tests were byte-identical there (the 26AS pair and
`applicability_44ab` with that client's own documents and turnover, emitted by the reference and fed to
both sides).

## Reference commit and invocations

Produced at the reference engine commit `1038dc05527c3f3818060288feece43324f8c682`, from an archive of
that commit with no client data, under Python 3.13, with ENGINE the archive:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.TEST.json --test TEST
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

## Bytes

| Fixture | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `partners_firm_deed.json` | 5,704 | `c660779bcef45cec82a2c495068262cd0f58ff9f8efa1f75e62f76ce26fba8ae` | `edge-books/partners_firm_deed.json` |
| `partners_llp_no_deed.json` | 3,670 | `890f8981cc2676050db816d31857f3549f192dceef2fe9fab95cad2dc169cee8` | `edge-books/partners_llp_no_deed.json` |
| `partners_not_applicable.json` | 2,796 | `c5141157e39743f91fa0ba1817af82da293537d2bc510de9e932d5a395f9becb` | `edge-books/partners_not_applicable.json` |
| `edge.partners_firm_deed.partners_40b_194t.json` | 32,857 | `fb65b74e2983d5f7210f37b86d5d6a91cf931ecc4a651e18f2ec4b7ead36aff1` | `golden/edge.partners_firm_deed.partners_40b_194t.json` |
| `edge.partners_llp_no_deed.partners_40b_194t.json` | 25,253 | `812b4d4b6cefe93463b0a05b150d89ef5b8ada5e830a51340a8076995a7b673a` | `golden/edge.partners_llp_no_deed.partners_40b_194t.json` |
| `edge.partners_not_applicable.partners_40b_194t.json` | 1,049 | `d03a931902fd87eb43fcf832cc2bb5297e03da9e34d7dfb0b3a4c51c57677e73` | `golden/edge.partners_not_applicable.partners_40b_194t.json` |
| `synthetic.partners_40b_194t.json` | 1,605 | `671f8ecaa00a79a50499fa1853abfe156c08c48ded2496210c330fb956fae1c5` | `golden/synthetic.partners_40b_194t.json` |
| `partners_tds_mixed.json` | 7,336 | `6a5722722fea285506982aaaebd8afc9fa70defb77e26faf3ed6ab0de9baf9e8` | `edge-books/partners_tds_mixed.json` |
| `edge.partners_tds_mixed.partners_40b_194t.json` | 50,483 | `9d2d297210b9ce2e37ab6f1a9117754453d627f1559837679bbdb4940f012533` | `golden/edge.partners_tds_mixed.partners_40b_194t.json` |
| `partners_deed_no_interest.json` | 3,935 | `38e10d442f3fb66572909c4dab815b39d648b878a263cde8edcb95ddb4128a92` | `edge-books/partners_deed_no_interest.json` |
| `edge.partners_deed_no_interest.partners_40b_194t.json` | 21,935 | `670f514a8c0ed63b0fe5c1e9ecbbb66bc3a5e062345885fd5561027042552539` | `golden/edge.partners_deed_no_interest.partners_40b_194t.json` |
