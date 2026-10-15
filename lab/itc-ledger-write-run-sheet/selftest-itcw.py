#!/usr/bin/env python3
"""selftest-itcw.py SHEET_DIR WORK REAL_S01 REAL_S02 COMMITTED_LEDGER_ANSWER : known-answer controls for check.py of the ITC write sheet.
Positive controls: the REAL seeding answers of BRIDGE SHAPE LAB (a01, a02) and a ledger answer DERIVED from the committed
live ledger answer of another synthetic book (GUID replaced by SHAPE's; labelled derived). Every negative is one change and
asserts its exit code and a reason string. The generic `led` form of check.py exists for these controls only (it needs the
switch BRIDGE_ITCW_ENGINE=1, which this script sets); the sheet's steps use led5, led6 and led7.
Inputs, from the public lab branches of this repository: REAL_S01 and REAL_S02 are `lab/seeding-answers/answers/ans-s01.xml` and
`ans-s02.xml`; COMMITTED_LEDGER_ANSWER is `lab/itc-ledgers-run-sheet/committed/ledger_gst_details_live.utf16le.xml`.
Run: python3 -I selftest-itcw.py . WORKDIR ans-s01.xml ans-s02.xml ledger_gst_details_live.utf16le.xml (WORKDIR a new directory)."""
import os, re, subprocess, sys, shutil
sheet, work, s01, s02, committed = sys.argv[1:6]
os.makedirs(work, exist_ok=True)
CHECK = os.path.join(sheet, "check.py")
res = {"ok": 0, "bad": 0}
os.environ["BRIDGE_ITCW_ENGINE"] = "1"

def run(*args):
    p = subprocess.run([sys.executable, "-I", CHECK, *args], capture_output=True, text=True, cwd=work)
    return p.returncode, p.stdout

def expect(label, code, want, *args, has=None):
    rc, out = run(*args)
    ok = rc == want and (has is None or has in out)
    res["ok" if ok else "bad"] += 1
    print(("ok   " if ok else "FAIL ") + label + f" (exit {rc}, want {want})" + ("" if ok else "\n     " + out.strip()[-400:]))
    return out

def rd(p):
    b = open(p, "rb").read(); return b[2:].decode("utf-16-le") if b[:2] == b"\xff\xfe" else b.decode("utf-16-le")
def wr(name, t, bom=True):
    p = os.path.join(work, name); open(p, "wb").write((b"\xff\xfe" if bom else b"") + t.encode("utf-16-le")); return p

a01 = wr("a01.xml", rd(s01)); a02 = wr("a02.xml", rd(s02))
shape_guid = re.search(r"<GUID[^>]*>([^<]*)</GUID>", rd(s02)).group(1).strip()
led = rd(committed)
pilot = re.search(r"<BRIDGECOMPANYGUID[^>]*>([^<]*)</BRIDGECOMPANYGUID>", led).group(1)
base_t = led.replace(pilot, shape_guid)                          # DERIVED: another synthetic book's ledger read under SHAPE's GUID
base = wr("base.xml", base_t)
NAME, PARENT = "ITC Test Ledger", "Purchase Accounts"

def row(name=NAME, itc="Yes", parent=PARENT, frm="20250401", extra_entry="", guid_suffix="000000f0", ob="0.00", src="Specify Details Here", tax="Taxable"):
    itc_el = f'<GSTINELIGIBLEITC TYPE="Logical">{itc}</GSTINELIGIBLEITC>' if itc else ""
    entry = (f'<GSTDETAILS.LIST><APPLICABLEFROM TYPE="Date">{frm}</APPLICABLEFROM><TAXABILITY TYPE="String">{tax}</TAXABILITY>'
             f'<SRCOFGSTDETAILS TYPE="String">{src}</SRCOFGSTDETAILS>{itc_el}<STATEWISEDETAILS.LIST><STATENAME TYPE="String"> Any</STATENAME>'
             f'<RATEDETAILS.LIST><GSTRATEDUTYHEAD TYPE="String">IGST</GSTRATEDUTYHEAD><GSTRATE TYPE="Number"> 18</GSTRATE></RATEDETAILS.LIST></STATEWISEDETAILS.LIST></GSTDETAILS.LIST>')
    return (f'<LEDGER NAME="{name}" RESERVEDNAME=""><GUID TYPE="String">{shape_guid}-{guid_suffix}</GUID><PARENT TYPE="String">{parent}</PARENT>'
            f'<ALTERID TYPE="Number"> 300</ALTERID><MASTERID TYPE="Number"> 290</MASTERID><OPENINGBALANCE TYPE="Amount">{ob}</OPENINGBALANCE><BRIDGECOMPANYGUID TYPE="String">{shape_guid}</BRIDGECOMPANYGUID>'
            f'{entry}{extra_entry}</LEDGER>')

def with_row(t, r):
    i = t.rindex("</COLLECTION>"); return t[:i] + r + t[i:]
yes = wr("led-yes.xml", with_row(base_t, row(itc="Yes")))
no = wr("led-no.xml", with_row(base_t, row(itc="No")))
absent = wr("led-absent.xml", with_row(base_t, row(itc="")))
C = ["--company", a02]
B = ["--before", base, "--created", NAME, "--parent", PARENT]

expect("a01 real SHAPE company list", 0, 0, "a01", a01)
expect("a01 on a02 is a STOP (no loaded-company list)", 3, 3, "a01", a02)
out = expect("a02 real SHAPE extent", 0, 0, "a02", a02, has="ALTMSTID")
shutil.copy(a02, os.path.join(work, "a02b.xml"))
expect("a02 --marks-from prints the movement", 0, 0, "a02", a02, "--marks-from", os.path.join(work, "a02b.xml"), has="marks since a02b.xml")
t02 = rd(s02).replace(shape_guid, "00000000-0000-0000-0000-000000000000")
expect("a02 --marks-from another GUID is a STOP", 3, 3, "a02", a02, "--marks-from", wr("a02-other.xml", t02), has="another GUID")
expect("a02 for another company is a STOP", 3, 3, "a02", wr("a02-name.xml", rd(s02).replace("BRIDGE SHAPE LAB", "BRIDGE PILOT LAB")), has="is not for BRIDGE SHAPE LAB")
expect("a02 mark missing is a FINDING", 1, 1, "a02", wr("a02-mark.xml", re.sub(r"<ALTMSTID[^>]*>[^<]*</ALTMSTID>", "", rd(s02))), has="missing")

expect("base derived book", 0, 0, "base", base, *C, has="ledgers,")
expect("base with --created absent", 0, 0, "base", base, *C, "--created", NAME)
expect("base holding the created name is a STOP", 3, 3, "base", yes, *C, "--created", NAME, has="already in the book")
expect("base without --company is exit 2", 2, 2, "base", base)
expect("base with another book's GUID is a STOP", 3, 3, "base", wr("base-g.xml", base_t.replace(shape_guid, "11111111-1111-1111-1111-111111111111", 1)), *C, has="wrong company")
expect("base truncated is a STOP", 3, 3, "base", wr("base-trunc.xml", base_t[: len(base_t) // 2]), *C, has="not a complete XML envelope")
open(os.path.join(work, "base-empty.xml"), "wb").close()
expect("base empty is a STOP", 3, 3, "base", os.path.join(work, "base-empty.xml"), *C, has="empty")
expect("base with LINEERROR is a STOP", 3, 3, "base", wr("base-le.xml", base_t.replace("</COLLECTION>", "<LINEERROR>x</LINEERROR></COLLECTION>")), *C, has="LINEERROR")
expect("base STATUS 0 is a STOP", 3, 3, "base", wr("base-st.xml", base_t.replace("<STATUS>1</STATUS>", "<STATUS>0</STATUS>")), *C, has="STATUS is 0")
expect("base ledger name repeats is a STOP", 3, 3, "base", wr("base-dup.xml", with_row(base_t, row(name="Cash"))), *C, has="repeats")
expect("base no LEDGER row is a STOP", 3, 3, "base", wr("base-none.xml", re.sub(r"<LEDGER .*?</LEDGER>", "", base_t, flags=re.S)), *C, has="no LEDGER row")
expect("base above its cap is a STOP", 3, 3, "base", wr("base-big.xml", base_t.replace("</COLLECTION>", "<X>" + "a" * 120000 + "</X></COLLECTION>")), *C, has="above the cap")
expect("base ITC reading Maybe is a STOP", 3, 3, "base", wr("base-maybe.xml", with_row(base_t, row(name="Other", itc="Maybe"))), *C, has="not Yes or No")
expect("base two entries on one date is a STOP", 3, 3, "base", wr("base-2d.xml", with_row(base_t, row(name="Other", extra_entry=row().split("</GSTDETAILS.LIST>")[0].split("<GSTDETAILS.LIST>", 1)[1].join(["<GSTDETAILS.LIST>", "</GSTDETAILS.LIST>"])))), *C, has="two entries on")
expect("base unreadable date is a STOP", 3, 3, "base", wr("base-date.xml", with_row(base_t, row(name="Other", frm="soon"))), *C, has="APPLICABLEFROM")

o = expect("led Yes", 0, 0, "led", yes, *C, *B, has="wire GSTINELIGIBLEITC Yes")
expect("led absent flag", 0, 0, "led", absent, *C, *B, has="wire GSTINELIGIBLEITC absent")
o = expect("led --prev Yes->No", 0, 0, "led", no, *C, *B, "--prev", yes, has="GSTINELIGIBLEITC Yes->No; nothing else moved")
expect("led --prev absent->Yes", 0, 0, "led", yes, *C, *B, "--prev", absent, has="absent->Yes")
expect("led --prev with another leaf moved is reported", 0, 0, "led", wr("led-rate.xml", with_row(base_t, row(itc="No").replace(" 18", " 12"))), *C, *B, "--prev", yes, has="also moved")
expect("led --prev new entry is reported", 0, 0, "led", wr("led-new.xml", with_row(base_t, row(itc="No", extra_entry=row(frm="20250922").split("<GSTDETAILS.LIST>", 1)[1].join(["<GSTDETAILS.LIST>", ""]).split("</LEDGER>")[0].replace("</GSTDETAILS.LIST>", "</GSTDETAILS.LIST>")))), *C, *B, "--prev", yes, has="new since")
expect("led wrong parent is a STOP", 3, 3, "led", wr("led-par.xml", with_row(base_t, row(parent="Sales Accounts"))), *C, *B, has="expected 'Purchase Accounts'")
expect("led another ledger created is a STOP", 3, 3, "led", wr("led-two.xml", with_row(with_row(base_t, row()), row(name="Extra", guid_suffix="000000f1"))), *C, *B, has="expected")
expect("led created ledger missing is a STOP", 3, 3, "led", base, *C, *B, has="not in the answer")
expect("led other ledger removed is a STOP", 3, 3, "led", wr("led-gone.xml", with_row(re.sub(r'<LEDGER NAME="Cash".*?</LEDGER>', "", base_t, count=1, flags=re.S), row())), *C, *B, has="something else was created or removed")
cash = re.search(r'<LEDGER NAME="Cash".*?</LEDGER>', base_t, flags=re.S).group(0)
expect("led another ledger row changed is a FINDING", 1, 1, "led", wr("led-chg.xml", with_row(base_t.replace(cash, cash.replace("</LEDGER>", "<NOTE>x</NOTE></LEDGER>")), row())), *C, *B, has="differ on")
expect("led --before already holding the created ledger is a STOP", 3, 3, "led", yes, *C, "--before", yes, "--created", NAME, "--parent", PARENT, has="not the book before")
expect("led without --before is exit 2", 2, 2, "led", yes, *C, "--created", NAME, "--parent", PARENT)
expect("led --prev without the ledger is a STOP", 3, 3, "led", yes, *C, *B, "--prev", base, has="no 'ITC Test Ledger'")
expect("led wrong GUID on the created row is a STOP", 3, 3, "led", wr("led-g.xml", with_row(base_t, row().replace(f'<GUID TYPE="String">{shape_guid}', '<GUID TYPE="String">22222222-2222'))), *C, *B, has="wrong company")

EXP = "20250401:Taxable:Specify Details Here:18"
expect("led --expect-entry holds", 0, 0, "led", yes, *C, *B, "--expect-entry", EXP)
expect("led --expect-entry wrong date is a FINDING", 1, 1, "led", wr("led-d26.xml", with_row(base_t, row(frm="20260401", src="As per Company/Group", tax="", itc=""))), *C, *B, "--expect-entry", EXP, has="EXPECT exactly one entry")
expect("led --expect-entry wrong source is a FINDING", 1, 1, "led", wr("led-src.xml", with_row(base_t, row(src="As per Company/Stock Group"))), *C, *B, "--expect-entry", EXP, has="EXPECT exactly one entry")
expect("led --expect-entry wrong taxability is a FINDING", 1, 1, "led", wr("led-tax.xml", with_row(base_t, row(tax="Exempt"))), *C, *B, "--expect-entry", EXP, has="EXPECT exactly one entry")
expect("led --expect-entry wrong rate is a FINDING", 1, 1, "led", wr("led-r.xml", with_row(base_t, row().replace(" 18", " 12"))), *C, *B, "--expect-entry", EXP, has="EXPECT exactly one entry")
expect("led --expect-entry with two entries is a FINDING", 1, 1, "led", wr("led-2e.xml", with_row(base_t, row(extra_entry=row(frm="20250922").split("<GSTDETAILS.LIST>", 1)[1].join(["<GSTDETAILS.LIST>", ""]).split("</LEDGER>")[0]))), *C, *B, "--expect-entry", EXP, has="2 entries")
expect("led --expect-entry without a value is exit 2", 2, 2, "led", yes, *C, *B, "--expect-entry")
expect("led --expect-entry and --own-only together is exit 2", 2, 2, "led", yes, *C, *B, "--expect-entry", EXP, "--own-only")
own = wr("led-own.xml", with_row(base_t, row(src="As per Company/Stock Group", tax="", itc="Yes").replace(' 18', '').replace('<GSTRATE TYPE="Number"></GSTRATE>', '')))
expect("led --own-only on Tally's own entry holds", 0, 0, "led", wr("led-own2.xml", with_row(base_t, row(src="As per Company/Stock Group", tax="", itc="Yes").replace('<GSTRATE TYPE="Number"> 18</GSTRATE>', ''))), *C, *B, "--own-only")
expect("led --own-only with a keyed entry is a FINDING", 1, 1, "led", yes, *C, *B, "--own-only", has="not one")
expect("led --prev without a file is exit 2", 2, 2, "led", yes, *C, *B, "--prev")
expect("led opening balance keyed is a FINDING", 1, 1, "led", wr("led-ob.xml", with_row(base_t, row(ob="100.00"))), *C, *B, has="zero opening balance")
expect("led --prev ledger row moved (opening balance) is a FINDING", 1, 1, "led", wr("led-ob2.xml", with_row(base_t, row(ob="5.00"))), *C, *B, "--prev", yes, has="ledger row unchanged outside")
expect("led --prev ledger deleted and re-created (new GUID) is a FINDING", 1, 1, "led", wr("led-regen.xml", with_row(base_t, row(guid_suffix="000000f9"))), *C, *B, "--prev", yes, has="moved:")
expect("led --prev ledger row equal prints so", 0, 0, "led", no, *C, *B, "--prev", yes, has="outside its GST details equals")
voucher = wr("a02-v.xml", re.sub(r"(<ALTVCHID[^>]*>)[^<]*", r"\g<1> 113", rd(s02)))
expect("a02 --marks-from with ALTVCHID moved is a FINDING", 1, 1, "a02", voucher, "--marks-from", a02, has="ALTVCHID unchanged")
expect("base above 120000 B is a STOP", 3, 3, "base", wr("base-120k.xml", base_t.replace("</COLLECTION>", "<X>" + "a" * 70000 + "</X></COLLECTION>")), *C, has="above the cap")
expect("base of 100588 B (the size the lab read) is accepted", 0, 0, "base", wr("base-100k.xml", base_t.replace("</COLLECTION>", "<X>" + "a" * 24560 + "</X></COLLECTION>")), *C)
sub_reason = row(itc="No").replace("</LEDGER>", "<GSTINELIGIBLEITCREASON>x</GSTINELIGIBLEITCREASON></LEDGER>")
expect("led a new leaf named like the flag is reported, not hidden", 0, 0, "led", wr("led-reason.xml", with_row(base_t, row(itc="No").replace("</GSTDETAILS.LIST>", "<GSTINELIGIBLEITCREASON>x</GSTINELIGIBLEITCREASON></GSTDETAILS.LIST>"))), *C, *B, "--prev", yes, has="also moved")
expect("led ledger-level leaf moved is a note by default", 0, 0, "led", wr("led-lv.xml", with_row(base_t, row(itc="No").replace("</LEDGER>", "<GSTAPPLICABLE>Yes</GSTAPPLICABLE></LEDGER>"))), *C, *B, "--prev", yes, has="recorded, not a finding")
expect("led ledger-level leaf moved is a FINDING with --strict-row", 1, 1, "led", wr("led-lv2.xml", with_row(base_t, row(itc="No").replace("</LEDGER>", "<GSTAPPLICABLE>Yes</GSTAPPLICABLE></LEDGER>"))), *C, *B, "--prev", yes, "--strict-row", has="and every other leaf")
expect("led prints ALTERID movement", 0, 0, "led", no, *C, *B, "--prev", yes, has="ALTERID 300->300")
expect("led --own-only with two entries is a FINDING", 1, 1, "led", wr("led-own3.xml", with_row(base_t, row(src="As per Company/Stock Group", tax="", itc="Yes", extra_entry=row(frm="20250922", src="As per Company/Stock Group", tax="", itc="Yes").split("<GSTDETAILS.LIST>", 1)[1].join(["<GSTDETAILS.LIST>", ""]).split("</LEDGER>")[0]).replace('<GSTRATE TYPE="Number"> 18</GSTRATE>', ''))), *C, *B, "--own-only", has="at most one entry")
expect("led --own-only with an entry on another date is a note", 0, 0, "led", wr("led-own4.xml", with_row(base_t, row(frm="20260401", src="As per Company/Stock Group", tax="", itc="Yes").replace('<GSTRATE TYPE="Number"> 18</GSTRATE>', ''))), *C, *B, "--own-only", has="not the book's first day")
expect("base: a name differing only in case is a STOP", 3, 3, "base", wr("base-case.xml", with_row(base_t, row(name="ITC TEST LEDGER"))), *C, "--created", NAME, has="not case-sensitive")
def alt(t, n): return t.replace('<ALTERID TYPE="Number"> 300</ALTERID>', f'<ALTERID TYPE="Number"> {n}</ALTERID>')
expect("led --alterid-plus-1 holds at +1", 0, 0, "led", wr("led-a301.xml", with_row(base_t, alt(row(itc="No"), 301))), *C, *B, "--prev", yes, "--alterid-plus-1")
expect("led --alterid-plus-1 +2 is a FINDING", 1, 1, "led", wr("led-a302.xml", with_row(base_t, alt(row(itc="No"), 302))), *C, *B, "--prev", yes, "--alterid-plus-1", has="exactly 1")
expect("led --alterid-plus-1 +0 is a FINDING", 1, 1, "led", no, *C, *B, "--prev", yes, "--alterid-plus-1", has="exactly 1")
expect("led unknown option is exit 2", 2, 2, "led", yes, *C, *B, "--strict_row", has="unknown option")
expect("led --strict-row without --prev is exit 2", 2, 2, "led", yes, *C, *B, "--strict-row")
expect("led --alterid-plus-1 without --prev is exit 2", 2, 2, "led", yes, *C, *B, "--alterid-plus-1")
expect("base unknown option is exit 2", 2, 2, "base", base, *C, "--frobnicate")
expect("led an en-dash option is exit 2", 2, 2, "led", yes, *C, *B, "\u2013strict-row", has="unknown option")
expect("led a single-dash option is exit 2", 2, 2, "led", yes, *C, *B, "-alterid-plus-1", has="unknown option")
expect("led an option with its dashes missing is exit 2", 2, 2, "led", yes, *C, *B, "--prev", yes, "alterid-plus-1", has="belong to no option")
expect("same with a third file is exit 2", 2, 2, "same", base, yes, no, has="exactly two")
expect("same with an option is exit 2", 2, 2, "same", base, yes, "--alterid-plus-1", has="exactly two")
expect("a02 --marks-from without a value is exit 2", 2, 2, "a02", a02, "--marks-from", has="no value")
expect("base --created without a value is exit 2", 2, 2, "base", base, *C, "--created", has="no value")
expect("a repeated option is exit 2", 2, 2, "led", yes, *C, *B, "--prev", yes, "--prev", no, has="given 2 times")
expect("a02 --marks-from the answer itself is exit 2", 2, 2, "a02", a02, "--marks-from", a02, has="proves nothing")
expect("led --created empty value is exit 2", 2, 2, "led", yes, *C, "--before", base, "--created", " ", "--parent", PARENT, has="no value")
expect("led prints the checks in force", 0, 0, "led", wr("led-a301b.xml", with_row(base_t, alt(row(itc="No"), 301))), *C, *B, "--prev", yes, "--strict-row", "--alterid-plus-1", "--expect-entry", EXP, has="checks in force: --prev --expect-entry --strict-row --alterid-plus-1")
expect("led without the optional checks says so", 0, 0, "led", yes, *C, *B, has="none of the optional ones")
expect("a02 with --strict-row is exit 2", 2, 2, "a02", a02, "--strict-row", has="do not belong")
expect("led with --marks-from is exit 2", 2, 2, "led", yes, *C, *B, "--marks-from", a02, has="do not belong")
expect("base with --prev is exit 2", 2, 2, "base", base, *C, "--prev", base, has="do not belong")
B5 = ["--before", base, "--created", NAME, "--parent", PARENT]
def mk(label, ans, *extra): return (label, ans, *C, *B5, *extra)
os.environ.pop("BRIDGE_ITCW_ENGINE")
expect("led (engine) without the engine switch is exit 2", 2, 2, "led", yes, *C, *B5, has="use led5")
own_ok = wr("led-own5.xml", with_row(base_t, row(src="As per Company/Stock Group", tax="", itc="Yes").replace('<GSTRATE TYPE="Number"> 18</GSTRATE>', '')))
expect("led5 with its exact options", 0, 0, "led5", own_ok, *C, *B5, "--own-only", has="led5:")
expect("led5 without --own-only is exit 2", 2, 2, "led5", own_ok, *C, *B5, has="missing")
expect("led5 with --prev is exit 2", 2, 2, "led5", own_ok, *C, *B5, "--own-only", "--prev", yes, has="not allowed")
E6 = ["--prev", yes, "--expect-entry", EXP, "--alterid-plus-1"]
y301 = wr("led-y301.xml", with_row(base_t, alt(row(itc="Yes"), 301)))
n302 = wr("led-n302.xml", with_row(base_t, alt(row(itc="No"), 302)))
expect("led6 with its exact options", 0, 0, "led6", y301, *C, *B5, *E6, has="led6:")
expect("led6 without --alterid-plus-1 is exit 2", 2, 2, "led6", y301, *C, *B5, "--prev", yes, "--expect-entry", EXP, has="missing")
expect("led6 without --expect-entry is exit 2", 2, 2, "led6", y301, *C, *B5, "--prev", yes, "--alterid-plus-1", has="missing")
expect("led6 with --strict-row is exit 2", 2, 2, "led6", y301, *C, *B5, *E6, "--strict-row", has="not allowed")
expect("led7 with its exact options", 0, 0, "led7", wr("led-n301.xml", with_row(base_t, alt(row(itc="No"), 301))), *C, *B5, *E6, "--strict-row", has="led7:")
expect("led7 without --strict-row is exit 2", 2, 2, "led7", n302, *C, *B5, *E6, has="missing")
os.environ["BRIDGE_ITCW_ENGINE"] = "1"
expect("same identical", 0, 0, "same", base, shutil.copy(base, os.path.join(work, "base2.xml")) and os.path.join(work, "base2.xml"), has="identical")
expect("same with only header counters different", 0, 0, "same", base, wr("base-hdr.xml", base_t.replace("<LEDGER>10</LEDGER>", "<LEDGER>11</LEDGER>")), has="header counters differ")
expect("same different under COLLECTION is a FINDING", 1, 1, "same", base, yes, has="DIFFERENT under COLLECTION")
expect("same one file twice is exit 2", 2, 2, "same", base, base)
expect("same a truncated second file is a STOP", 3, 3, "same", base, os.path.join(work, "base-trunc.xml"))
expect("unknown step is exit 2", 2, 2, "nope", base)
print(f"\nselftest: {res['ok']} as expected, {res['bad']} not")
sys.exit(1 if res["bad"] else 0)
