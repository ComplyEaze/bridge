#!/usr/bin/env python3 -I
"""List the cells an exported workbook fills that its blank template does not.

Run: python -I x1-cells.py TEMPLATE EXPORT [--max-per-sheet N]
     python -I x1-cells.py --self-test

Both files are Excel workbooks (.xlsx or .xlsm), read as the zip of XML parts they are: no Excel,
no macro and no third-party package is used. For every worksheet of EXPORT it prints, by sheet name,
each cell whose value or formula differs from the same cell of TEMPLATE (a cell absent from TEMPLATE
counts as empty), as `sheet | cell | kind | value`, at most N cells a sheet (default 400) followed by
a count of the rest, then one summary line per sheet: `SHEET <name>: changed <n>, formula cells
changed <m>`. Output is ASCII only; any other character prints as \\uXXXX.

Exit codes: 0 = compared (whatever the result); 2 = a file could not be read or is not a workbook
(never read as "nothing changed"); 3 = the two workbooks do not have the same sheet names (the
export is not a copy of this template: compare nothing).
"""
import re
import sys
import zipfile
import xml.etree.ElementTree as ET

NS = {"m": "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
      "r": "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
      "pr": "http://schemas.openxmlformats.org/package/2006/relationships"}


def ascii_only(text):
    return "".join(ch if 32 <= ord(ch) < 127 else "\\u%04x" % ord(ch) for ch in text)


def read_workbook(path):
    """Return {sheet name: {cell ref: (kind, text)}} with kind 'formula' or 'value'."""
    with zipfile.ZipFile(path) as book:
        names = set(book.namelist())
        shared = []
        if "xl/sharedStrings.xml" in names:
            root = ET.fromstring(book.read("xl/sharedStrings.xml"))
            for item in root.findall("m:si", NS):
                shared.append("".join(t.text or "" for t in item.iter("{%s}t" % NS["m"])))
        rels = ET.fromstring(book.read("xl/_rels/workbook.xml.rels"))
        target = {rel.get("Id"): rel.get("Target") for rel in rels.findall("pr:Relationship", NS)}
        workbook = ET.fromstring(book.read("xl/workbook.xml"))
        sheets = {}
        for sheet in workbook.find("m:sheets", NS).findall("m:sheet", NS):
            rid = sheet.get("{%s}id" % NS["r"])
            part = target[rid].lstrip("/")
            part = part if part.startswith("xl/") else "xl/" + part
            cells = {}
            for cell in ET.fromstring(book.read(part)).iter("{%s}c" % NS["m"]):
                ref = cell.get("r")
                formula = cell.find("m:f", NS)
                value = cell.find("m:v", NS)
                inline = cell.find("m:is", NS)
                if formula is not None and (formula.text or formula.get("t")):
                    cells[ref] = ("formula", formula.text or "(shared)")
                    continue
                if value is not None and value.text is not None:
                    text = value.text
                    if cell.get("t") == "s":
                        text = shared[int(text)]
                elif inline is not None:
                    text = "".join(t.text or "" for t in inline.iter("{%s}t" % NS["m"]))
                else:
                    continue
                if text != "":
                    cells[ref] = ("value", text)
            sheets[sheet.get("name")] = cells
        return sheets


def compare(template, export, limit, out):
    if list(template) != list(export):
        out("STOP: the sheet names differ: template %s, export %s" % (
            ascii_only(str(list(template))), ascii_only(str(list(export)))))
        return 3
    for name, cells in export.items():
        before = template[name]
        changed = [ref for ref in cells if before.get(ref) != cells[ref]]
        changed += [ref for ref in before if ref not in cells]
        changed = sorted(set(changed), key=lambda ref: (int(re.sub(r"[A-Z]+", "", ref)), len(ref), ref))
        formulas = 0
        for index, ref in enumerate(changed):
            kind, text = cells[ref] if ref in cells else ("cleared", before[ref][1])
            formulas += kind == "formula"
            if index < limit:
                out("%s | %s | %s | %s" % (ascii_only(name), ref, kind, ascii_only(text[:120])))
        if len(changed) > limit:
            out("%s | ... %d more changed cells not printed" % (ascii_only(name), len(changed) - limit))
        out("SHEET %s: changed %d, formula cells changed %d" % (ascii_only(name), len(changed), formulas))
    return 0


def minimal_workbook(path, cells, sheet="4"):
    """Write a one-sheet workbook holding `cells` (ref -> number, text, or '=formula')."""
    def cell(ref, value):
        if value.startswith("="):
            return '<c r="%s"><f>%s</f></c>' % (ref, value[1:])
        if re.fullmatch(r"-?[0-9.]+", value):
            return '<c r="%s"><v>%s</v></c>' % (ref, value)
        return '<c r="%s" t="inlineStr"><is><t>%s</t></is></c>' % (ref, value)
    rows = "".join(cell(ref, value) for ref, value in cells.items())
    with zipfile.ZipFile(path, "w") as book:
        book.writestr("xl/workbook.xml", '<workbook xmlns="%s" xmlns:r="%s"><sheets><sheet name="%s" sheetId="1" r:id="rId1"/></sheets></workbook>' % (NS["m"], NS["r"], sheet))
        book.writestr("xl/_rels/workbook.xml.rels", '<Relationships xmlns="%s"><Relationship Id="rId1" Target="worksheets/sheet1.xml"/></Relationships>' % NS["pr"])
        book.writestr("xl/worksheets/sheet1.xml", '<worksheet xmlns="%s"><sheetData><row r="1">%s</row></sheetData></worksheet>' % (NS["m"], rows))


def self_test():
    import os
    import tempfile
    folder = tempfile.mkdtemp()
    blank, filled = os.path.join(folder, "blank.xlsx"), os.path.join(folder, "filled.xlsx")
    minimal_workbook(blank, {"A1": "Table 4", "C5": "=SUM(A1:B1)"})
    minimal_workbook(filled, {"A1": "Table 4", "C5": "=SUM(A1:B1)", "D7": "1000.50", "E7": "Tax"})
    lines = []
    code = compare(read_workbook(blank), read_workbook(filled), 400, lines.append)
    assert code == 0, code
    assert lines == ["4 | D7 | value | 1000.50", "4 | E7 | value | Tax", "SHEET 4: changed 2, formula cells changed 0"], lines
    lines = []
    assert compare(read_workbook(blank), read_workbook(blank), 400, lines.append) == 0
    assert lines == ["SHEET 4: changed 0, formula cells changed 0"], lines
    other = os.path.join(folder, "other.xlsx")
    minimal_workbook(other, {"A1": "x"}, sheet="5")
    lines = []
    assert compare(read_workbook(blank), read_workbook(other), 400, lines.append) == 3, lines
    broken = os.path.join(folder, "broken.xlsx")
    open(broken, "wb").write(b"not a workbook")
    assert main(["x1-cells.py", blank, broken]) == 2
    print("self-test: ok")
    return 0


def main(argv):
    if argv[1:] == ["--self-test"]:
        return self_test()
    args = argv[1:]
    limit = 400
    if "--max-per-sheet" in args:
        at = args.index("--max-per-sheet")
        limit = int(args[at + 1])
        del args[at:at + 2]
    if len(args) != 2:
        print(__doc__.strip().splitlines()[2])
        return 2
    try:
        template, export = read_workbook(args[0]), read_workbook(args[1])
    except (OSError, KeyError, zipfile.BadZipFile, ET.ParseError, ValueError, IndexError) as error:
        print("could not read: %s" % ascii_only(repr(error)))
        return 2
    return compare(template, export, limit, print)


if __name__ == "__main__":
    sys.exit(main(sys.argv))
