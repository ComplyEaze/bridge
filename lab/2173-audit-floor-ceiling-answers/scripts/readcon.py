# Lab helper (not product code): read the visible text of another process's console (the relay's), without typing into it:
# attach to its console, open CONOUT$, read the screen buffer's lines. Usage: python -I readcon.py <pid> [lines]
import ctypes, ctypes.wintypes as wt, sys
k = ctypes.windll.kernel32
pid = int(sys.argv[1]); nlines = int(sys.argv[2]) if len(sys.argv) > 2 else 12
k.FreeConsole()
if not k.AttachConsole(pid): sys.stderr.write(f'attach failed {ctypes.GetLastError()}\n'); sys.exit(1)
h = k.CreateFileW('CONOUT$', 0xC0000000, 3, None, 3, 0, None)
class COORD(ctypes.Structure): _fields_ = [('X', wt.SHORT), ('Y', wt.SHORT)]
class SMALL_RECT(ctypes.Structure): _fields_ = [('Left', wt.SHORT), ('Top', wt.SHORT), ('Right', wt.SHORT), ('Bottom', wt.SHORT)]
class CSBI(ctypes.Structure): _fields_ = [('dwSize', COORD), ('dwCursorPosition', COORD), ('wAttributes', wt.WORD), ('srWindow', SMALL_RECT), ('dwMaximumWindowSize', COORD)]
info = CSBI(); k.GetConsoleScreenBufferInfo(h, ctypes.byref(info))
width = info.dwSize.X; out = []
for y in range(0, min(info.dwCursorPosition.Y + 1, nlines + info.dwCursorPosition.Y + 1)):
    buf = ctypes.create_unicode_buffer(width); n = wt.DWORD()
    k.ReadConsoleOutputCharacterW(h, buf, width, COORD(0, y), ctypes.byref(n))
    out.append(buf.value.rstrip())
k.FreeConsole()
with open(sys.argv[3] if len(sys.argv) > 3 else 'NUL', 'w', encoding='utf-8') as f: f.write('\n'.join(out) + '\n')
print('\n'.join(l for l in out if l)[-4000:])
