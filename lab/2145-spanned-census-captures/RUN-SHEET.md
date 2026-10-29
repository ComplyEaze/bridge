# #2145: the census spanned to (0, mark] on the four synthetic books whose replays still hold only the unspanned census

Not for merge. READS ONLY, on four synthetic companies the lab already holds, one at a time. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6) applies in full, with THE COMPANY of each part below; where this sheet and it differ, the stricter holds. About 30 to 45 minutes.

**WHY.** #2145 sends a census whose voucher mark fits one request as the AlterID span `(0, mark]`, never unspanned. The maintainers' rule (4 Oct, on #899) is that the change is built only on captures of the spanned request: an answer captured for the unspanned census may not stand in for it. The replays below each recorded a live call whose census was unspanned. After the change the code sends that census spanned to the mark the call read. This item sends, for each recorded window, the recorded census request as the control and the same request spanned to that mark, so the replays can carry real answers. BRIDGE TRANSFER LAB, BRIDGE OUTSTANDINGS LAB and BRIDGE SHAPE LAB were captured spanned earlier, and are not read here.

**THE REQUESTS** (`requests/`, UTF-16LE with a byte-order mark):
- `X-marks`: the company marks read, the product's own request (`Bridge Agent Company High Water`) for that company.
- `KEY_c1_unspanned`: the census the replay recorded. Each is byte-equal to the request its replay pins: the sha256 pinned in the test, or for the pilot audit the committed request file. Two windows of one day share one control (`a-d3-0401`).
- `KEY_c2_spanned_0_M`: the same request with ` AND $AlterID &gt; 0 AND $AlterID &lt;= M` added at the end of its census formula, and nothing else changed. M is the voucher mark the replay's call read. The shape is the one sent spanned on 26 Oct and 28 Oct to three other synthetic books, with no dialog.

**REQUESTS BY CURL** are allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes. Each request is sent exactly as LAB-COMMON C3 says, one at a time. Before the first send, run `python -I check_requests.py requests` in this folder: EXPECT `census: ok 20, FAIL 0`; anything else = STOP, nothing sent. **Size bounds:** 16,384 bytes for a marks answer and 1,000,000 bytes for a census answer (the recorded ones are 5,444 to 126,960 bytes). **Time:** each request has 600 s to answer (C3). If no answer comes, or `GET /status` sent after it has no answer 60 s after it was sent, the gateway is held: STOP, leave the curl running, press no key, send nothing more, report, and wait (no state read and no shut until the run coordinator says).

## The books and windows

| Part | THE COMPANY (GUID) | Window (from to) | M | Control | Spanned | Replay's recorded census answer (sha256) |
|---|---|---|---|---|---|---|
| A | BRIDGE AMEND LAB (`17a10910-773c-42c6-bd66-7bba9a392536`) | 20260403 | 1676 | `a-wa1_c1` | `a-wa1_c2_spanned_0_1676` | `d92b4aff960009b5f42d4ba77512894989efee1a77ff50afe50b8362b3dd7be7` (`agent/wa1-payment-voucher-census`) |
| A | | 20260401 | 1469 | `a-d3-0401_c1` | `a-d3batch_c2_spanned_0_1469` | `eb90841da8d2237f43fee668b5219a3bb287e064d85ff27b3536c4ac003166ae` (`agent/d3-batch-voucher-census`) |
| A | | 20260401 | 1685 | (the same control) | `a-d3cancel_c2_spanned_0_1685` | `743dcd652b3d8470975056c07b3a4643a025866404bf902bc0a13ea6d1d541e5` (`agent/d3-cancelled-voucher-census`) |
| A | | 20260617 | 1790 | `a-l1_c1` | `a-l1_c2_spanned_0_1790` | `a46f499f37692b910c070c80d95b01d136082f15bb2ec35bec3814561af966d6` (`agent/l1-reentry-voucher-census`) |
| P | BRIDGE PILOT LAB (`6b43e498-430c-4d5c-bfef-d32e2ab93c85`) | 20260401 20270331 | 68 | `p-audit_c1` | `p-audit_c2_spanned_0_68` | `11c0077ba4a02cf6b076e56364b00928f20390db68257a65ffd25efc35e03f55` (`agent/audit-read-pilot-lab/pilot-lab-audit-census`) |
| G | BRIDGE GST LEDGER LAB (`aae372c5-a894-491c-8843-5532e43d2d2f`) | 20251103 | 64 | `g-03nov_c1` | `g-03nov_c2_spanned_0_64` | `f0d39e7fa67588da76d66a40363a6ec72e216d92948858e09570f19e53f58113` |
| G | | 20251104 | 64 | `g-04nov_c1` | `g-04nov_c2_spanned_0_64` | `d0f5d50a525a1735b342427ac4cfe98dd6f88509350bc24d0b800a0167d5e214` |
| G | | 20251105 | 64 | `g-05nov_c1` | `g-05nov_c2_spanned_0_64` | `ff6393088fdbd1c670251c2b1da6d15dce93bff0ecbfd3483270c8ae86f2013c` |
| G | | 20251106 | 64 | `g-06nov_c1` | `g-06nov_c2_spanned_0_64` | `4f669c66dda0031f567447088619496d6f5d9be02e234d450b08bf7cc54f018b` |
| G | | 20251107 | 64 | `g-07nov_c1` | `g-07nov_c2_spanned_0_64` | `4587e4d498c77fa5a563fe899c6583b1d3037f8ddf77582eee50f68775eea7a0` |
| G | | 20251108 | 64 | `g-08nov_c1` | `g-08nov_c2_spanned_0_64` | `4395c211e7ebe55f3e754846634be1ace1299a8fd499948df0a6734aa1e0c01a` |
| G | | 20251110 | 64 | `g-10nov_c1` | `g-10nov_c2_spanned_0_64` | `700d208be69301e70a8d5a71b68d82fd8edcb00592c7343b677e5716d02dd67b` |
| G | | 20251111 | 64 | `g-11nov_c1` | `g-11nov_c2_spanned_0_64` | `946bee1424766c64aa5df4a7fe4259ac04f207c90d803139785b4c6972905d27` |
| G | | 20251112 | 64 | `g-12nov_c1` | `g-12nov_c2_spanned_0_64` | `f96c8f32d34379e19d6fb4ef587a8aa8ce703a09eb148207360598e5679a9497` |
| G | | 20251113 | 64 | `g-13nov_c1` | `g-13nov_c2_spanned_0_64` | `81db84c63f6a675b562e5407d9618c3012a0d6b3564a3752049b85da0e805f4a` |
| G | | 20251114 | 64 | `g-14nov_c1` | `g-14nov_c2_spanned_0_64` | `fb6df7804b4bbee2680309ce36e2bb691fe348baa0a5cd19e21617dddfbd4cac` |
| G | | 20251115 | 64 | `g-15nov_c1` | `g-15nov_c2_spanned_0_64` | `a75c6fffb5390c3788cf5bf4f07c5cfe79434141ae9c663588222258cfd44c81` |
| G | | 20251116 | 64 | `g-16nov_c1` | `g-16nov_c2_spanned_0_64` | `8c1a770145779c10d5c103d22c366c661fbc99621abe5702e3669f248b5e6e1f` |
| G | | 20251117 | 64 | `g-17nov_c1` | `g-17nov_c2_spanned_0_64` | `599435920156c7bba90ef2c4fe075932c597278188b1811e8d1340bcddbcf074` |
| S | BRIDGE STOCK LAB (`1edb9b05-8d35-4c0c-9959-dce655730463`) | 20250421 | 3 | `s-0421_c1` | `s-0421_c2_spanned_0_3` | none: the replay's call sent no census, so it holds no census answer |

The GST LEDGER LAB answers are the `agent/gst-ledger-l1/s6-DDnov-n-1` records. All paths are under `src-tauri/crates/bridge-tally-protocol/tests/fixtures/`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I check_requests.py requests`: EXPECT `census: ok 20, FAIL 0`.
3. Tally licensed; Educational mode = STOP (C1). Record the release and licence mode only.
4. The working folder is `C:/bridge-lab/2145/` (absent or empty before, else STOP), with one folder per part (`A`, `P`, `G`, `S`).

## The reads: parts A, P, G and S, in that order, each in full before the next
1. Load THE COMPANY alone (C1). C2 step 0. Then the part's `X-marks`.
   - **The GUID check:** EXPECT one COMPANY row with THE COMPANY's GUID. Another GUID, or more than one row = STOP the part.
   - Record ALTVCHID and ALTMSTID. **ALTVCHID below the part's highest M** (A: 1790, P: 68, G: 64, S: 3) = STOP the part: the book is not the one recorded (it may have been restored to an older state). At or above it, go on, recording the value.
2. For each window of the part, in the table's order: send its control, then its spanned request. (Part A sends `a-d3-0401_c1` once, before both 1 April spanned requests.)
3. A dialog, a hang, a non-200 or an answer that does not parse = STOP (C3, C7, C9). The gateway is not probed again.
4. The part's `X-marks` again. EXPECT the marks of step 1, since the reads write nothing. A difference = record it and STOP.
5. C10 (Shut Company). Then the next part.

## EXPECT and the comparison (recorded; the lab judges only the STOP rules)
For each window run `python -I compare2145.py RECORDED M CONTROL_ANSWER SPANNED_ANSWER`, with RECORDED and M from the table (`none` for part S), and keep its whole output. It prints both answers' rows as (GUID, ALTERID, DATE), and the number of rows of the control above M, then:
- **A, the control equals the recorded answer:** the window is as it was when recorded. EXPECT it to hold for parts P and G, and for A's 20260403 and 20260617 windows. EXPECT it **not** to hold for A's 1 April window recorded at M 1469: on that day the book was altered after that record (the record at M 1685 is of the same day, three vouchers later altered past 1469). That is the finding, not a STOP.
- **B, the spanned answer equals the recorded answer:** the replay can carry the recorded answer for the spanned request, by this measurement. EXPECT it to hold wherever A holds. Where it does not hold, the replay needs a decision (below), not a guess.
- **C, every row of the spanned answer is inside the span.** EXPECT it to hold everywhere.

Exit 0: every verdict that ran holds. Exit 1: one does not. Exit 2: an error, never a verdict.

For part S there is no recorded census answer. EXPECT one row dated 21 Apr 2025 with ALTERID 3 or below in both answers, the vouchers of the replay's recorded window read.

## Where a verdict does not hold
Nothing is re-sent and nothing is edited in the book. The row is reported with both row lists. Each such replay is then decided on the issue: re-recorded from the book as it is now, or kept unchanged and named as a replay of the unspanned census. No restore is made for this sheet.

## The books
None changes; every request is a read. The marks are read at the start and the end of each part, and must be equal.

## Close and evidence
- **C10** for each part.
- **C11:** evidence to a new branch `lab/2145-spanned-census-captures-answers`, after the pre-push scan and the run coordinator's yes. It holds every answer, whole, with headers, size, sha256 and its send and answer times; each `compare2145.py` output; the `check_requests.py` output; and the notes, with UTC times and one line per step. Screenshots stay on the lab PC, listed by name, size and sha256.

## Not covered
- Any census whose mark is above 8,192: those are spanned already.
- The replays' other requests: only their census changes with #2145.
- The code itself: the replays are rebuilt on these answers in the #2145 change, and each spanned request here is checked there against the code's own rendering, byte for byte.
