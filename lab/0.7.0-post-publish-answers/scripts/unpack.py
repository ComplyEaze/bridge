# Lab helper (0.7.0 post-publish, not product code): unpack the release's Windows .mcpb into mcpb\, after checking no
# entry has an absolute or parent path; print the entries and the manifest's name and version.
import json, zipfile
BS = chr(92)
z = zipfile.ZipFile('<lab folder>/rc070-post/bridge-tally-mcp-v0.7.0-windows-x64.mcpb'); n = z.namelist()
print('entries', len(n))
for x in n: print(' ', x)
bad = [x for x in n if x.startswith(('/', BS)) or '..' in x.replace(BS, '/').split('/') or ':' in x]
print('absolute or parent paths', bad)
if bad: raise SystemExit('stop: unsafe entry')
z.extractall('<LAB_DIR>/mcpb')
m = json.load(open('<LAB_DIR>/mcpb/manifest.json', encoding='utf-8'))
print('manifest name', m.get('name'), 'version', m.get('version'))
