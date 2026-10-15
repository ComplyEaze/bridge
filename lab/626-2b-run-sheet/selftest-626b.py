#!/usr/bin/env python3 -I
"""Known-answer controls of check.py: real answers of the 13 Oct 2026 twin run (selftest-data/) and edits of them.
Run: python3 -I selftest-626b.py   (py -3 -I selftest-626b.py). Exit 0 only when every control gets its expected exit code
and its expected last-line prefix. Nothing here talks to Tally."""
import os, re, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
D = lambda f: os.path.join(HERE, "selftest-data", f)
tmp = tempfile.mkdtemp(prefix="selftest626b")

def run(args):
    p = subprocess.run([sys.executable, "-I", os.path.join(HERE, "check.py")] + args, capture_output=True, text=True)
    lines = [l for l in p.stdout.splitlines() if l.strip()]
    return p.returncode, (lines[-1] if lines else "")

def u16(path): 
    data = open(path, "rb").read()
    return data.decode("utf-16-le") if data[:2] != b"\xff\xfe" else data[2:].decode("utf-16-le")

def write(name, text):
    p = os.path.join(tmp, name); open(p, "wb").write(text.encode("utf-16-le")); return p

failures = 0
def control(label, args, code, prefix):
    global failures
    got, last = run(args)
    ok = got == code and last.startswith(prefix)
    if not ok: failures += 1
    print("%s %s: expected exit %d %s..., got exit %d %r" % ("ok  " if ok else "FAIL", label, code, prefix, got, last[:90]))

# stand-ins made from the real answers
v2 = u16(D("t2-vch.xml"))
blocks = re.split(r"(?=    <VOUCHER )", v2)
head, vouchers = blocks[0], blocks[1:]
tail_i = vouchers[-1].find("   </COLLECTION>")
tail = vouchers[-1][tail_i:]
vouchers[-1] = vouchers[-1][:tail_i]
assert len(vouchers) == 2, len(vouchers)
journal, payment = vouchers
only_payment = write("before-payment-only.xml", head + payment + tail)
none_before = write("before-none.xml", head + tail)
truncated = write("truncated.xml", v2[: len(v2) // 2])
imp_text = u16(D("t3-imp1.xml"))
imp_truncated = write("imp-truncated.xml", imp_text[: len(imp_text) // 2])
small_envelope = write("small-envelope.xml", "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION></COLLECTION></DATA></BODY></ENVELOPE>")
ledger_text = u16(D("t2-ledgers.xml"))
not_xml = write("notxml.xml", "hello")
wrong_company = write("wrong-company.xml", u16(D("t0-marks.xml")).replace("BRIDGE LINE BREAK LAB", "SOME OTHER BOOK"))
other_book = write("other-book-ledgers.xml", u16(D("t2-ledgers.xml")).replace("224e7f8c-a0aa-45ce-889a-ad4b7fdbe852</BRIDGECOMPANYGUID>", "-".join(["11111111", "0000", "0000", "0000", "000000000000"]) + "</BRIDGECOMPANYGUID>", 1))
dup_lines = write("dup-ledger.xml", u16(D("t2-ledgers.xml")).replace('<LEDGER NAME="Cash" ', '<LEDGER NAME="Lab Plain Ledger" ', 1))
no_counter = write("no-counter.xml", u16(D("t3-imp1.xml")).replace("<ERRORS>0</ERRORS>", ""))
imp_err = write("imp-errors.xml", u16(D("t3-imp1.xml")).replace("<CREATED>1</CREATED>", "<CREATED>0</CREATED>").replace("<ERRORS>0</ERRORS>", "<ERRORS>1</ERRORS>").replace("</RESPONSE>", " <LINEERROR>Duplicate Entry!</LINEERROR>\n</RESPONSE>"))
imp_two = write("imp-two.xml", u16(D("t3-imp1.xml")).replace("<CREATED>1</CREATED>", "<CREATED>2</CREATED>"))
imp_extra = write("imp-extra.xml", u16(D("t3-imp1.xml")).replace("<IGNORED>0</IGNORED>", "<IGNORED>1</IGNORED>"))

# marks
control("marks real answer", ["marks", D("t0-marks.xml")], 0, "PASS")
control("marks of another book", ["marks", wrong_company], 3, "STOP")
control("marks prev movement", ["marks", D("t5-marks.xml"), "--prev", D("t0-marks.xml")], 0, "PASS")
control("marks missing file", ["marks", os.path.join(tmp, "nope.xml")], 2, "CHECK-ERROR")
control("unknown step", ["frob", D("t0-marks.xml")], 2, "CHECK-ERROR")
control("marks unknown option", ["marks", D("t0-marks.xml"), "--wat", "x"], 2, "CHECK-ERROR")
# imp
control("imp created (real)", ["imp", D("t3-imp1.xml"), "--expect", "created"], 0, "PASS")
control("imp altered (real)", ["imp", D("t5-led3.xml"), "--expect", "altered"], 0, "PASS")
control("altered answer expected as created", ["imp", D("t5-led3.xml"), "--expect", "created"], 1, "FINDING")
control("created answer expected as altered", ["imp", D("t3-imp1.xml"), "--expect", "altered"], 1, "FINDING")
control("imp errors with the LINEERROR text", ["imp", imp_err, "--expect", "errors", "--lineerror", "Duplicate Entry!"], 0, "PASS")
control("imp errors with another text", ["imp", imp_err, "--expect", "errors", "--lineerror", "Something else"], 1, "FINDING")
control("imp CREATED 2", ["imp", imp_two, "--expect", "created"], 1, "FINDING")
control("imp IGNORED 1", ["imp", imp_extra, "--expect", "created"], 1, "FINDING")
control("imp counter missing", ["imp", no_counter, "--expect", "created"], 3, "STOP")
control("imp not a response", ["imp", small_envelope, "--expect", "created"], 3, "STOP: root is ENVELOPE")
control("imp truncated", ["imp", imp_truncated, "--expect", "created"], 3, "STOP: the answer is not a complete XML document")
control("imp above its cap", ["imp", D("t2-vch.xml"), "--expect", "created"], 3, "STOP: the answer is")
control("imp not xml", ["imp", not_xml, "--expect", "created"], 3, "STOP")
control("imp without --expect", ["imp", D("t3-imp1.xml")], 2, "CHECK-ERROR")
control("imp record: created", ["imp", D("t3-imp1.xml"), "--expect", "record"], 0, "PASS: recorded, the answer reads as CREATED")
control("imp record: altered", ["imp", D("t5-led3.xml"), "--expect", "record"], 0, "PASS: recorded, the answer reads as ALTERED")
control("imp record: errors", ["imp", imp_err, "--expect", "record"], 0, "PASS: recorded, the answer reads as ERRORS")
control("imp record: other counters", ["imp", imp_extra, "--expect", "record"], 0, "PASS: recorded, the answer reads as OTHER")
control("imp record: truncated", ["imp", imp_truncated, "--expect", "record"], 3, "STOP")
# ledgers
base = ["--marks", D("t0-marks.xml"), "--before", D("t1-ledgers.xml")]
control("ledgers one added (real)", ["ledgers", D("t2-ledgers.xml")] + base + ["--add", "Lab Line Break Ledger|Indirect Incomes"], 0, "PASS")
control("ledgers wrong parent", ["ledgers", D("t2-ledgers.xml")] + base + ["--add", "Lab Line Break Ledger|Indirect Expenses"], 1, "FINDING")
control("ledgers add not named", ["ledgers", D("t2-ledgers.xml")] + base, 1, "FINDING")
control("ledgers add not there", ["ledgers", D("t2-ledgers.xml")] + base + ["--add", "Nope|Indirect Incomes"], 1, "FINDING")
control("ledgers add with the CR LF spelling of a plain name", ["ledgers", D("t2-ledgers.xml")] + base + ["--add", "Lab Line Break Ledger<CRLF>|Indirect Incomes"], 1, "FINDING")
control("ledgers none new, none named", ["ledgers", D("t1-ledgers.xml")] + base, 0, "PASS")
control("ledgers other book's GUID", ["ledgers", other_book] + base + ["--add", "Lab Line Break Ledger|Indirect Incomes"], 3, "STOP")
control("ledgers duplicate NAME", ["ledgers", dup_lines] + base, 3, "STOP")
control("ledgers truncated", ["ledgers", truncated] + base, 3, "STOP")
mv = ["--marks", D("t0-marks.xml"), "--before", D("t4-ledgers.xml")]
control("ledgers moved group, named (real)", ["ledgers", D("t5-ledgers.xml")] + mv + ["--moved", "LAB CASE PROBE|Indirect Incomes"], 0, "PASS")
control("ledgers moved group, not named", ["ledgers", D("t5-ledgers.xml")] + mv, 1, "FINDING")
control("ledgers moved to another group than named", ["ledgers", D("t5-ledgers.xml")] + mv + ["--moved", "LAB CASE PROBE|Indirect Expenses"], 1, "FINDING")
control("ledgers missing --before", ["ledgers", D("t2-ledgers.xml"), "--marks", D("t0-marks.xml")], 2, "CHECK-ERROR")
control("ledgers: the absent names are absent", ["ledgers", D("t1-ledgers.xml")] + base + ["--absent", "Lab Order Probe", "--absent", "Lab Order Probe<CRLF>"], 0, "PASS")
control("ledgers: an absent name is there", ["ledgers", D("t1-ledgers.xml")] + base + ["--absent", "Lab Plain Ledger"], 1, "FINDING")
control("ledgers: the CR LF name is there, asked absent", ["ledgers", D("t1-ledgers.xml")] + base + ["--absent", "Lab Line Break Ledger<CRLF>"], 1, "FINDING")
# names
control("names compare (real)", ["names", D("t5-ledgers.xml"), "--marks", D("t0-marks.xml"), "--before", D("t3-ledgers.xml")], 0, "PASS")
# vch
lg = D("t3-ledgers.xml")
control("vch fold-only landed on the plain twin (real)", ["vch", D("t3-vch.xml"), "--ledgers", lg, "--before", D("t2-vch.xml"), "--amount", "611.00"], 0, "PASS")
control("vch exact plain name", ["vch", D("t3-vch.xml"), "--ledgers", lg, "--before", D("t2-vch.xml"), "--amount", "611.00", "--exact", "Lab Line Break Ledger"], 0, "PASS")
control("vch expected the CR LF twin, it landed on the plain one", ["vch", D("t3-vch.xml"), "--ledgers", lg, "--before", D("t2-vch.xml"), "--amount", "611.00", "--exact", "Lab Line Break Ledger<CRLF>"], 1, "FINDING")
control("vch allowed set holds it", ["vch", D("t3-vch.xml"), "--ledgers", lg, "--before", D("t2-vch.xml"), "--amount", "611.00", "--allowed", "Lab Line Break Ledger<CRLF>", "--allowed", "Lab Line Break Ledger"], 0, "PASS")
control("vch allowed set misses it", ["vch", D("t3-vch.xml"), "--ledgers", lg, "--before", D("t2-vch.xml"), "--amount", "611.00", "--allowed", "Lab Line Break Ledger<CRLF>"], 1, "FINDING")
control("vch wrong amount", ["vch", D("t3-vch.xml"), "--ledgers", lg, "--before", D("t2-vch.xml"), "--amount", "999.00"], 1, "FINDING")
control("vch the CR LF twin, named with <CRLF>", ["vch", D("t2-vch.xml"), "--ledgers", D("t2-ledgers.xml"), "--before", only_payment, "--amount", "500.00", "--exact", "Lab Line Break Ledger<CRLF>"], 0, "PASS")
control("vch the CR LF twin, named without the break", ["vch", D("t2-vch.xml"), "--ledgers", D("t2-ledgers.xml"), "--before", only_payment, "--amount", "500.00", "--exact", "Lab Line Break Ledger"], 1, "FINDING")
control("vch two new vouchers", ["vch", D("t2-vch.xml"), "--ledgers", D("t2-ledgers.xml"), "--before", none_before, "--amount", "500.00"], 1, "FINDING")
control("vch no new voucher", ["vch", D("t2-vch.xml"), "--ledgers", D("t2-ledgers.xml"), "--before", D("t2-vch.xml"), "--amount", "500.00"], 1, "FINDING")
control("vch truncated answer", ["vch", truncated, "--ledgers", lg, "--before", D("t2-vch.xml"), "--amount", "611.00"], 3, "STOP")
control("vch ledger list truncated", ["vch", D("t3-vch.xml"), "--ledgers", truncated, "--before", D("t2-vch.xml"), "--amount", "611.00"], 3, "STOP")
# same
control("same: an answer with itself", ["same", D("t0-marks.xml"), D("t0-marks.xml")], 0, "PASS")
control("same: two different ledger lists", ["same", D("t1-ledgers.xml"), D("t2-ledgers.xml")], 1, "FINDING")
control("same: truncated", ["same", truncated, D("t2-vch.xml")], 3, "STOP")
real_co = u16(D("company-list-shape.xml"))
co_ok = write("co-ok.xml", real_co.replace("BRIDGE SHAPE LAB", "BRIDGE LINE BREAK LAB"))
co_edu = write("co-edu.xml", real_co.replace("BRIDGE SHAPE LAB", "BRIDGE LINE BREAK LAB").replace('<EDUMODE TYPE="Logical">No', '<EDUMODE TYPE="Logical">Yes'))
control("co: the real company-list shape, the right book", ["co", co_ok], 0, "PASS")
control("co: the real company list of another book", ["co", D("company-list-shape.xml")], 3, "STOP: the loaded companies")
control("co: Education mode", ["co", co_edu], 3, "STOP: Education")
control("co: an answer with no company row", ["co", small_envelope], 3, "STOP: the loaded companies")
control("co: another book", ["co", wrong_company], 3, "STOP: the loaded companies")

print("%d control(s) did not fire" % failures if failures else "all controls fired")
sys.exit(1 if failures else 0)
