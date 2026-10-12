# W21: the GSTR-1 status read with the post-dated flag, then with the deleted flags as well (for #1655's follow-up)

Synthetic company `BRIDGE PILOT LAB` only. READS ONLY: nothing is keyed, altered, accepted, cancelled or deleted, no voucher or master is opened for alteration, and no screen is changed. No backup is needed (say so if you take one). Never use Accept As Is. It uses the book as W20 left it.

Each request is the status request the code sends once #1661 merges (`w20-status-window-aug-cancel-optional.xml`, SHA-256 `b5844282…`, a test pins the code's render to it), for this company, the window 1 to 31 August 2026 and the type `BRIDGE Sales`, with ONE change each to the end of its field list:

- `w21-status-window-aug-postdated.xml`: `,ISPOSTDATED` added.
- `w21-status-window-aug-postdated-deleted.xml`: `,ISPOSTDATED,ISDELETED,ISDELETEDVCHRETAINED` added.

Removing the added names from either file gives the W20 request back byte for byte. Check each file against `SHA256.txt` first.

**General STOP rule.** Stop and tell us before going on if: Tally shows any dialog, message or warning; an answer is not a complete envelope with a `COLLECTION` or carries a `LINEERROR`; an answer is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers `../r02-marks.xml`).

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request.

## The sitting, in this order
1. `../r02-marks.xml` -> `aw21-marks-0.xml`
2. `w20-status-window-aug-cancel-optional.xml` -> `aw21-a.xml` (the control, answered in W20 as `aw20-b.xml`).
3. `w21-status-window-aug-postdated.xml` -> `aw21-b.xml`
4. `w21-status-window-aug-postdated-deleted.xml` -> `aw21-c.xml`
5. `../r02-marks.xml` -> `aw21-marks-1.xml`

## What to post back
The five answers, their hashes added to `SHA256.txt`, and for each request curl's exit code and printed line. Say whether you did anything that differs from this list, and whether the book holds any voucher you know to be post-dated or deleted (W21 does not create one).

---

*For the reader (not for the lab):* `gst_status` (#1661) decides cancelled and optional vouchers before the three GSTR-1 flags. Post-dated vouchers are not marked, and a voucher Tally keeps after deletion (the edit log) is not guarded. Every voucher of W20's answer already carried `ISDELETED` and `ISDELETEDVCHRETAINED` (No) without the request asking for them. This set measures the exact requests a follow-up would send. One variable at a time: `b` adds one name, and `c` adds the two deleted flags on top of `b`. A Yes case for any of the three needs a voucher this read-only set does not make.

Pre-registered expectations (written before any answer):
- (a) the two marks answers are equal.
- (b) `aw21-a.xml` carries the same 32 vouchers as `aw20-b.xml`, with the same `ALTERID`, `MASTERID`, three status flags, `ISGSTOVERRIDDEN`, `ISCANCELLED` and `ISOPTIONAL` on each. Byte equality with `aw20-b.xml` is likely but not required.
- (c) `aw21-b.xml` carries the same vouchers, each with `ISPOSTDATED` exactly once and No, and every field of (b) unchanged.
- (d) `aw21-c.xml` carries the same vouchers, each with `ISPOSTDATED`, `ISDELETED` and `ISDELETEDVCHRETAINED` exactly once (not twice, although the last two appear unasked in `aw20-b.xml`), all No, and every field of (b) unchanged.

The opposite of any of these is a finding, not an error.
