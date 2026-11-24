// Lab helper (row 2569-S; not product code). ASCII, C# 5 (Windows PowerShell 5.1 compiles it with Add-Type).
// Read-only: reads what one running x64 process received at its start, from its own process parameters block
// (RTL_USER_PROCESS_PARAMETERS, reached by PEB+0x20): the STARTUPINFO flags and show state Windows copied there,
// the console handle and standard handles, the image path and command line; the image's PE subsystem; and its
// parent's pid. Optionally lists the process's top-level windows (hidden ones included). Opens processes for
// QUERY_INFORMATION and VM_READ only. Sends no message to any window, moves nothing, ends nothing.
// Every read prints one RESULT line: OK, or a typed failure (OPEN_FAILED, QUERY_FAILED, READ_FAILED,
// PARTIAL_READ, WOW64_TARGET, HOST_NOT_64BIT, LAYOUT_CHECK_FAILED, PARENT_UNREADABLE). A failure is never
// printed as an empty or zero value. Creation flags (CREATE_NO_WINDOW and the like) are not stored anywhere a
// reader can reach after the start; only their effects (console handle, console window) can be read.
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

public interface IStartInfoMemory
{
    // Returns the bytes read (possibly fewer than asked), or null when the read failed outright.
    byte[] Read(long address, int length, out int error);
}

public static class StartInfo
{
    public const int ParamsReadLength = 0xF0;
    public const int MaxString = 32766;
    const uint QueryInformation = 0x0400;
    const uint QueryLimited = 0x1000;
    const uint VmRead = 0x0010;

    [DllImport("ntdll.dll")]
    static extern int NtQueryInformationProcess(IntPtr process, int infoClass, byte[] info, int length, out int returned);
    [DllImport("kernel32.dll", SetLastError = true)]
    static extern IntPtr OpenProcess(uint access, bool inherit, int pid);
    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool CloseHandle(IntPtr handle);
    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool ReadProcessMemory(IntPtr process, IntPtr address, byte[] buffer, IntPtr size, out IntPtr read);
    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool IsWow64Process(IntPtr process, out bool wow64);
    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    static extern bool QueryFullProcessImageNameW(IntPtr process, int flags, StringBuilder name, ref int size);
    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool GetProcessTimes(IntPtr process, out long creation, out long exit, out long kernel, out long user);

    delegate bool EnumWindowsProc(IntPtr window, IntPtr lparam);
    [DllImport("user32.dll", SetLastError = true)]
    static extern bool EnumWindows(EnumWindowsProc callback, IntPtr lparam);
    [DllImport("user32.dll")]
    static extern uint GetWindowThreadProcessId(IntPtr window, out uint pid);
    [DllImport("user32.dll")]
    static extern bool IsWindowVisible(IntPtr window);
    [DllImport("user32.dll")]
    static extern bool IsIconic(IntPtr window);
    [DllImport("user32.dll")]
    static extern int GetWindowLong(IntPtr window, int index);
    [DllImport("user32.dll")]
    static extern IntPtr GetWindow(IntPtr window, uint command);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    static extern int GetClassNameW(IntPtr window, StringBuilder name, int max);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    static extern int GetWindowTextW(IntPtr window, StringBuilder text, int max);
    [StructLayout(LayoutKind.Sequential)]
    struct Rect { public int Left, Top, Right, Bottom; }
    [DllImport("user32.dll")]
    static extern bool GetWindowRect(IntPtr window, out Rect rect);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool FreeConsole();
    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool AttachConsole(int pid);
    [DllImport("kernel32.dll")]
    static extern IntPtr GetConsoleWindow();

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    struct StartupInfoW
    {
        public int cb; public string lpReserved; public string lpDesktop; public string lpTitle;
        public int dwX, dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags;
        public short wShowWindow; public short cbReserved2; public IntPtr lpReserved2;
        public IntPtr hStdInput, hStdOutput, hStdError;
    }
    [StructLayout(LayoutKind.Sequential)]
    struct ProcessInformation { public IntPtr hProcess, hThread; public int dwProcessId, dwThreadId; }
    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    static extern bool CreateProcessW(string application, StringBuilder commandLine, IntPtr processAttributes,
        IntPtr threadAttributes, bool inheritHandles, uint creationFlags, IntPtr environment, string currentDirectory,
        ref StartupInfoW startup, out ProcessInformation info);
    // GetStartupInfoW fills pointers that Windows owns: read them as IntPtr, never as string (the marshaler
    // would release them).
    [StructLayout(LayoutKind.Sequential)]
    struct StartupInfoRaw
    {
        public int cb; public IntPtr lpReserved; public IntPtr lpDesktop; public IntPtr lpTitle;
        public int dwX, dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags;
        public short wShowWindow; public short cbReserved2; public IntPtr lpReserved2;
        public IntPtr hStdInput, hStdOutput, hStdError;
    }
    [DllImport("kernel32.dll")]
    static extern void GetStartupInfoW(out StartupInfoRaw startup);
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode)]
    static extern IntPtr GetCommandLineW();
    [DllImport("kernel32.dll")]
    static extern uint WaitForSingleObject(IntPtr handle, uint milliseconds);
    [DllImport("kernel32.dll")]
    static extern bool GetExitCodeProcess(IntPtr process, out uint code);
    [DllImport("kernel32.dll")]
    static extern int GetCurrentProcessId();

    // ---- the parsed block (pure, testable without Windows) ----

    public class Params
    {
        public string Failure;              // null when every field below was read
        public uint MaximumLength, Length, Flags, ConsoleFlags;
        public long ConsoleHandle, StdIn, StdOut, StdErr;
        public uint StartingX, StartingY, CountX, CountY, CountCharsX, CountCharsY, FillAttribute;
        public uint WindowFlags, ShowWindowFlags;
        public string CurrentDirectory, ImagePath, CommandLine, WindowTitle, DesktopInfo, ShellInfo;
        public int RuntimeDataLength;
    }

    static uint U32(byte[] b, int at) { return BitConverter.ToUInt32(b, at); }
    static long I64(byte[] b, int at) { return BitConverter.ToInt64(b, at); }

    // Reads exactly `length` bytes or sets `failure` to READ_FAILED / PARTIAL_READ naming `what`.
    static byte[] Exact(IStartInfoMemory memory, long address, int length, string what, out string failure)
    {
        failure = null;
        int error;
        byte[] got = memory.Read(address, length, out error);
        if (got == null)
        {
            failure = "READ_FAILED at=" + what + " address=0x" + address.ToString("X") + " error=" + error;
            return null;
        }
        if (got.Length != length)
        {
            failure = "PARTIAL_READ at=" + what + " got=" + got.Length + " want=" + length + " error=" + error;
            return null;
        }
        return got;
    }

    static string UnicodeString(IStartInfoMemory memory, byte[] block, int at, long blockBase, bool normalized,
        string what, out string failure)
    {
        failure = null;
        int length = BitConverter.ToUInt16(block, at);
        int maximum = BitConverter.ToUInt16(block, at + 2);
        long buffer = I64(block, at + 8);
        if (length == 0) { return ""; }
        if ((length & 1) != 0 || length > maximum || length > MaxString * 2)
        {
            failure = "LAYOUT_CHECK_FAILED at=" + what + " length=" + length + " maximum=" + maximum;
            return null;
        }
        if (!normalized) { buffer += blockBase; }
        byte[] raw = Exact(memory, buffer, length, what, out failure);
        if (raw == null) { return null; }
        return Encoding.Unicode.GetString(raw);
    }

    // Parses the parameters block at `blockBase`. Offsets are the x64 layout of RTL_USER_PROCESS_PARAMETERS.
    public static Params ParseParams(IStartInfoMemory memory, long blockBase)
    {
        Params p = new Params();
        string failure;
        byte[] b = Exact(memory, blockBase, ParamsReadLength, "parameters", out failure);
        if (b == null) { p.Failure = failure; return p; }
        p.MaximumLength = U32(b, 0x00);
        p.Length = U32(b, 0x04);
        p.Flags = U32(b, 0x08);
        if (p.Length < ParamsReadLength || p.Length > 0x100000 || p.MaximumLength < p.Length)
        {
            p.Failure = "LAYOUT_CHECK_FAILED at=parameters length=" + p.Length + " maximum=" + p.MaximumLength;
            return p;
        }
        bool normalized = (p.Flags & 0x1) != 0;
        p.ConsoleHandle = I64(b, 0x10);
        p.ConsoleFlags = U32(b, 0x18);
        p.StdIn = I64(b, 0x20);
        p.StdOut = I64(b, 0x28);
        p.StdErr = I64(b, 0x30);
        p.StartingX = U32(b, 0x88);
        p.StartingY = U32(b, 0x8C);
        p.CountX = U32(b, 0x90);
        p.CountY = U32(b, 0x94);
        p.CountCharsX = U32(b, 0x98);
        p.CountCharsY = U32(b, 0x9C);
        p.FillAttribute = U32(b, 0xA0);
        p.WindowFlags = U32(b, 0xA4);
        p.ShowWindowFlags = U32(b, 0xA8);
        p.CurrentDirectory = UnicodeString(memory, b, 0x38, blockBase, normalized, "current_directory", out failure);
        if (failure != null) { p.Failure = failure; return p; }
        p.ImagePath = UnicodeString(memory, b, 0x60, blockBase, normalized, "image_path", out failure);
        if (failure != null) { p.Failure = failure; return p; }
        p.CommandLine = UnicodeString(memory, b, 0x70, blockBase, normalized, "command_line", out failure);
        if (failure != null) { p.Failure = failure; return p; }
        p.WindowTitle = UnicodeString(memory, b, 0xB0, blockBase, normalized, "window_title", out failure);
        if (failure != null) { p.Failure = failure; return p; }
        p.DesktopInfo = UnicodeString(memory, b, 0xC0, blockBase, normalized, "desktop_info", out failure);
        if (failure != null) { p.Failure = failure; return p; }
        p.ShellInfo = UnicodeString(memory, b, 0xD0, blockBase, normalized, "shell_info", out failure);
        if (failure != null) { p.Failure = failure; return p; }
        p.RuntimeDataLength = BitConverter.ToUInt16(b, 0xE0);
        return p;
    }

    static readonly string[] ShowNames = { "SW_HIDE", "SW_SHOWNORMAL", "SW_SHOWMINIMIZED", "SW_SHOWMAXIMIZED",
        "SW_SHOWNOACTIVATE", "SW_SHOW", "SW_MINIMIZE", "SW_SHOWMINNOACTIVE", "SW_SHOWNA", "SW_RESTORE",
        "SW_SHOWDEFAULT", "SW_FORCEMINIMIZE" };
    static readonly uint[] FlagBits = { 0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200, 0x800, 0x1000,
        0x2000, 0x8000 };
    static readonly string[] FlagNames = { "USESHOWWINDOW", "USESIZE", "USEPOSITION", "USECOUNTCHARS",
        "USEFILLATTRIBUTE", "RUNFULLSCREEN", "FORCEONFEEDBACK", "FORCEOFFFEEDBACK", "USESTDHANDLES", "USEHOTKEY",
        "TITLEISLINKNAME", "TITLEISAPPID", "PREVENTPINNING", "UNTRUSTEDSOURCE" };

    // The block keeps the image path as the parent gave it (a client may pass forward slashes); Windows reports
    // it with backslashes. Two paths name the same image when they are equal after the slash direction, a
    // \\?\ or \??\ prefix and the case are set aside.
    public static string NormalImage(string path)
    {
        if (path == null) { return null; }
        string t = path.Replace('/', '\\');
        if (t.StartsWith("\\\\?\\") || t.StartsWith("\\??\\")) { t = t.Substring(4); }
        return t.ToUpperInvariant();
    }

    public static bool SameImage(string block, string queried)
    {
        string a = NormalImage(block);
        string b = NormalImage(queried);
        return a != null && b != null && a.Length > 0 && a == b;
    }

    public static string FlagText(uint flags)
    {
        List<string> names = new List<string>();
        uint rest = flags;
        for (int i = 0; i < FlagBits.Length; i++)
        {
            if ((flags & FlagBits[i]) != 0) { names.Add(FlagNames[i]); rest &= ~FlagBits[i]; }
        }
        if (rest != 0) { names.Add("other=0x" + rest.ToString("X")); }
        return names.Count == 0 ? "none" : string.Join("|", names.ToArray());
    }

    public static string ShowText(uint show)
    {
        return show < ShowNames.Length ? ShowNames[show] : "unknown";
    }

    static string Chars(string s) { return s == null ? "<unread>" : s.Length.ToString(); }

    static string Cap(string s, int max)
    {
        if (s == null) { return "<unread>"; }
        StringBuilder o = new StringBuilder();
        foreach (char c in s)
        {
            if (c < 0x20 || c > 0x7E) { o.Append("\\u" + ((int)c).ToString("X4")); } else { o.Append(c); }
        }
        string t = o.ToString();
        if (t.Length > max) { return t.Substring(0, max) + "...(" + (t.Length - max) + " more chars)"; }
        return t;
    }

    public static List<string> Describe(Params p)
    {
        return Describe(p, true);
    }

    // `full` false prints the image's file name only and the lengths of the command line, folder and title:
    // used for ancestors above the client, whose text may hold user paths or session values.
    public static List<string> Describe(Params p, bool full)
    {
        List<string> lines = new List<string>();
        if (p.Failure != null) { lines.Add("RESULT " + p.Failure); return lines; }
        lines.Add("RESULT OK");
        lines.Add("params length=" + p.Length + " maximum=" + p.MaximumLength + " flags=0x" + p.Flags.ToString("X")
            + " normalized=" + ((p.Flags & 0x1) != 0));
        lines.Add("window_flags=0x" + p.WindowFlags.ToString("X") + " (" + FlagText(p.WindowFlags) + ")");
        lines.Add("show_window_flags=" + p.ShowWindowFlags + " (" + ShowText(p.ShowWindowFlags) + ") used_for_first_show="
            + ((p.WindowFlags & 0x1) != 0 ? "yes (USESHOWWINDOW set)" : "no (USESHOWWINDOW clear)"));
        lines.Add("starting x=" + p.StartingX + " y=" + p.StartingY + " size=" + p.CountX + "x" + p.CountY
            + " chars=" + p.CountCharsX + "x" + p.CountCharsY + " fill=0x" + p.FillAttribute.ToString("X"));
        lines.Add("console_handle=0x" + p.ConsoleHandle.ToString("X") + " console_flags=0x" + p.ConsoleFlags.ToString("X")
            + " (read after start: the connection the process holds, not the creation flag)");
        lines.Add("std_in=0x" + p.StdIn.ToString("X") + " std_out=0x" + p.StdOut.ToString("X") + " std_err=0x"
            + p.StdErr.ToString("X"));
        if (!full)
        {
            string leaf = p.ImagePath == null ? null : p.ImagePath.Replace('/', '\\');
            int cut = leaf == null ? -1 : leaf.LastIndexOf('\\');
            if (cut >= 0) { leaf = leaf.Substring(cut + 1); }
            lines.Add("image_name=[" + Cap(leaf, 80) + "] (path, command line, folder and title not printed: chars "
                + Chars(p.CommandLine) + ", " + Chars(p.CurrentDirectory) + ", " + Chars(p.WindowTitle) + ")");
            return lines;
        }
        lines.Add("image_path=[" + Cap(p.ImagePath, 260) + "]");
        lines.Add("command_line=[" + Cap(p.CommandLine, 300) + "]");
        lines.Add("current_directory=[" + Cap(p.CurrentDirectory, 260) + "]");
        lines.Add("window_title=[" + Cap(p.WindowTitle, 120) + "] desktop=[" + Cap(p.DesktopInfo, 60) + "] shell_info=["
            + Cap(p.ShellInfo, 60) + "] runtime_data_bytes=" + p.RuntimeDataLength);
        return lines;
    }

    // ---- live reads ----

    class LiveMemory : IStartInfoMemory
    {
        readonly IntPtr process;
        public LiveMemory(IntPtr process) { this.process = process; }
        public byte[] Read(long address, int length, out int error)
        {
            byte[] buffer = new byte[length];
            IntPtr read;
            bool ok = ReadProcessMemory(process, new IntPtr(address), buffer, new IntPtr(length), out read);
            error = ok ? 0 : Marshal.GetLastWin32Error();
            int n = (int)read.ToInt64();
            if (!ok && n == 0) { return null; }
            if (n == length) { return buffer; }
            byte[] part = new byte[n];
            Array.Copy(buffer, part, n);
            return part;
        }
    }

    static string Utc(long fileTime)
    {
        return DateTime.FromFileTimeUtc(fileTime).ToString("yyyy-MM-ddTHH:mm:ss.fffZ");
    }

    // Opens `pid`; on failure returns IntPtr.Zero and the typed failure text.
    static IntPtr Open(int pid, out string failure, out bool limited)
    {
        failure = null;
        limited = false;
        IntPtr h = OpenProcess(QueryInformation | VmRead, false, pid);
        if (h != IntPtr.Zero) { return h; }
        int first = Marshal.GetLastWin32Error();
        h = OpenProcess(QueryLimited | VmRead, false, pid);
        if (h != IntPtr.Zero) { limited = true; return h; }
        int second = Marshal.GetLastWin32Error();
        string why = first == 87 ? "pid not running" : (first == 5 ? "access denied" : "see error");
        failure = "OPEN_FAILED error=" + first + "/" + second + " (" + why + ")";
        return IntPtr.Zero;
    }

    public class Reading
    {
        public List<string> Lines = new List<string>();
        public bool Ok;
        public int ParentPid = -1;
        public long Created;
        public Params P;
    }

    public static Reading ReadProcess(int pid)
    {
        return ReadProcess(pid, true);
    }

    public static Reading ReadProcess(int pid, bool full)
    {
        Reading r = new Reading();
        r.Lines.Add("PROCESS pid=" + pid);
        if (!Environment.Is64BitProcess) { r.Lines.Add("RESULT HOST_NOT_64BIT (run 64-bit Windows PowerShell)"); return r; }
        string failure;
        bool limited;
        IntPtr h = Open(pid, out failure, out limited);
        if (h == IntPtr.Zero) { r.Lines.Add("RESULT " + failure); return r; }
        try
        {
            bool wow;
            if (!IsWow64Process(h, out wow)) { r.Lines.Add("RESULT QUERY_FAILED IsWow64Process error=" + Marshal.GetLastWin32Error()); return r; }
            if (wow) { r.Lines.Add("RESULT WOW64_TARGET (a 32-bit process; its block is not read)"); return r; }
            long created, exited, kernel, user;
            if (GetProcessTimes(h, out created, out exited, out kernel, out user))
            {
                r.Created = created;
                r.Lines.Add("created=" + Utc(created));
            }
            else { r.Lines.Add("created=<unread error=" + Marshal.GetLastWin32Error() + ">"); }
            StringBuilder name = new StringBuilder(1024);
            int size = name.Capacity;
            string queried = QueryFullProcessImageNameW(h, 0, name, ref size) ? name.ToString() : null;
            byte[] pbi = new byte[48];
            int returned;
            int status = NtQueryInformationProcess(h, 0, pbi, pbi.Length, out returned);
            if (status != 0 || returned != 48)
            {
                r.Lines.Add("RESULT QUERY_FAILED status=0x" + status.ToString("X8") + " returned=" + returned);
                return r;
            }
            long peb = I64(pbi, 8);
            r.ParentPid = (int)I64(pbi, 40);
            LiveMemory memory = new LiveMemory(h);
            byte[] pointer = Exact(memory, peb + 0x20, 8, "peb_process_parameters", out failure);
            if (pointer == null) { r.Lines.Add("RESULT " + failure); return r; }
            Params p = ParseParams(memory, I64(pointer, 0));
            if (p.Failure == null)
            {
                if (!SameImage(p.ImagePath, queried))
                {
                    p.Failure = "LAYOUT_CHECK_FAILED at=image_path block=[" + Cap(p.ImagePath, 200) + "] queried=["
                        + Cap(queried, 200) + "]";
                }
            }
            if (p.Failure != null && p.Failure.StartsWith("LAYOUT_CHECK_FAILED at=image_path"))
            {
                // The block was read but its image path is not the process's: print its values, each marked.
                string why = p.Failure;
                p.Failure = null;
                r.Lines.Add("RESULT " + why);
                foreach (string l in Describe(p, full)) { if (!l.StartsWith("RESULT")) { r.Lines.Add("UNCALIBRATED " + l); } }
                p.Failure = why;
            }
            else { r.Lines.AddRange(Describe(p, full || p.Failure != null)); }
            r.P = p;
            r.Ok = p.Failure == null;
            r.Lines.Add("open_access=" + (limited ? "query_limited|vm_read" : "query|vm_read") + " parent_pid=" + r.ParentPid);
            if (r.Ok) { r.Lines.Add("pe_subsystem=" + Subsystem(queried)); }
            return r;
        }
        finally { CloseHandle(h); }
    }

    public static string Subsystem(string path)
    {
        try
        {
            byte[] head = new byte[4096];
            int n;
            using (System.IO.FileStream f = new System.IO.FileStream(path, System.IO.FileMode.Open,
                System.IO.FileAccess.Read, System.IO.FileShare.ReadWrite | System.IO.FileShare.Delete))
            {
                n = f.Read(head, 0, head.Length);
            }
            return SubsystemOf(head, n);
        }
        catch (Exception e) { return "READ_FAILED (" + e.GetType().Name + ")"; }
    }

    public static string SubsystemOf(byte[] head, int n)
    {
        if (n < 0x40 || head[0] != 'M' || head[1] != 'Z') { return "LAYOUT_CHECK_FAILED (no MZ header)"; }
        int pe = BitConverter.ToInt32(head, 0x3C);
        if (pe < 0 || pe + 24 + 70 > n) { return "LAYOUT_CHECK_FAILED (PE header beyond the bytes read)"; }
        if (head[pe] != 'P' || head[pe + 1] != 'E' || head[pe + 2] != 0 || head[pe + 3] != 0)
        {
            return "LAYOUT_CHECK_FAILED (no PE signature)";
        }
        int value = BitConverter.ToUInt16(head, pe + 24 + 68);
        string label = value == 2 ? "WINDOWS_GUI" : (value == 3 ? "WINDOWS_CUI" : "other");
        return value + " (" + label + ")";
    }

    // Levels of a chain printed in full: the target (the server) and its parent (the lab's client).
    public const int FullLevels = 1;

    // The process and up to `depth` ancestors, each checked for pid reuse by its creation time.
    public static List<string> ReadChain(int pid, int depth)
    {
        List<string> lines = new List<string>();
        Reading child = ReadProcess(pid);
        lines.AddRange(child.Lines);
        for (int level = 1; level <= depth; level++)
        {
            if (!child.Ok || child.ParentPid <= 0) { break; }
            Reading parent = ReadProcess(child.ParentPid, level <= FullLevels);
            lines.Add("ANCESTOR level=" + level + " of pid " + pid);
            if (parent.Created != 0 && child.Created != 0 && parent.Created > child.Created)
            {
                lines.Add("PROCESS pid=" + child.ParentPid);
                lines.Add("RESULT PARENT_UNREADABLE (pid reused: created after its child)");
                break;
            }
            lines.AddRange(parent.Lines);
            child = parent;
        }
        return lines;
    }

    public static List<string> Windows(int pid, int max)
    {
        List<string> lines = new List<string>();
        List<IntPtr> found = new List<IntPtr>();
        bool ok = EnumWindows(delegate(IntPtr w, IntPtr l)
        {
            uint owner;
            GetWindowThreadProcessId(w, out owner);
            if (owner == (uint)pid) { found.Add(w); }
            return true;
        }, IntPtr.Zero);
        if (!ok) { lines.Add("WINDOWS_ENUM_FAILED error=" + Marshal.GetLastWin32Error()); return lines; }
        lines.Add("windows " + found.Count + " (top-level, hidden included)");
        for (int i = 0; i < found.Count && i < max; i++)
        {
            IntPtr w = found[i];
            StringBuilder cls = new StringBuilder(64);
            GetClassNameW(w, cls, cls.Capacity);
            StringBuilder title = new StringBuilder(256);
            GetWindowTextW(w, title, title.Capacity);
            Rect rect;
            GetWindowRect(w, out rect);
            int ex = GetWindowLong(w, -20);
            IntPtr owner = GetWindow(w, 4);
            string ownerText = owner == IntPtr.Zero ? "none" : (owner.ToInt64() + " owner_iconic=" + IsIconic(owner)
                + " owner_visible=" + IsWindowVisible(owner));
            lines.Add("WINDOW handle=" + w.ToInt64() + " class=" + Cap(cls.ToString(), 40) + " title=[" + Cap(title.ToString(), 60)
                + "] visible=" + IsWindowVisible(w) + " iconic=" + IsIconic(w) + " topmost=" + ((ex & 0x8) != 0)
                + " toolwindow=" + ((ex & 0x80) != 0) + " rect=(" + rect.Left + "," + rect.Top + "," + rect.Right + ","
                + rect.Bottom + ") owner=" + ownerText);
        }
        if (found.Count > max) { lines.Add("windows omitted " + (found.Count - max)); }
        return lines;
    }

    // Attaches this process to `pid`'s console, reads its window, and detaches. Writes nothing to the console.
    public static List<string> ConsoleOf(int pid)
    {
        List<string> lines = new List<string>();
        FreeConsole();
        if (!AttachConsole(pid))
        {
            int error = Marshal.GetLastWin32Error();
            string why = error == 6 ? "NO_CONSOLE (the process has none)" : "ATTACH_FAILED";
            lines.Add("CONSOLE pid=" + pid + " " + why + " error=" + error);
            return lines;
        }
        IntPtr w = GetConsoleWindow();
        string text = w == IntPtr.Zero ? "window=none" : ("window=" + w.ToInt64() + " visible=" + IsWindowVisible(w)
            + " iconic=" + IsIconic(w));
        FreeConsole();
        lines.Add("CONSOLE pid=" + pid + " attached " + text);
        return lines;
    }

    // ---- self-tests ----

    class FakeMemory : IStartInfoMemory
    {
        public long Base; public byte[] Bytes; public int Cut = -1;
        public byte[] Read(long address, int length, out int error)
        {
            error = 0;
            long start = address - Base;
            if (start < 0 || start >= Bytes.Length) { error = 299; return null; }
            int n = (int)Math.Min(length, Bytes.Length - start);
            if (Cut >= 0 && start + n > Cut) { n = (int)Math.Max(0, Cut - start); error = 299; }
            if (n == 0) { return null; }
            byte[] o = new byte[n];
            Array.Copy(Bytes, start, o, 0, n);
            return o;
        }
    }

    static void PutU32(byte[] b, int at, uint v) { Array.Copy(BitConverter.GetBytes(v), 0, b, at, 4); }
    static void PutI64(byte[] b, int at, long v) { Array.Copy(BitConverter.GetBytes(v), 0, b, at, 8); }

    // A synthetic block: strings stored after the fixed part, as Windows stores them.
    static FakeMemory Synthetic(bool normalized, string image, string command)
    {
        FakeMemory m = new FakeMemory();
        m.Base = 0x7FF612340000;
        byte[] b = new byte[0x400];
        PutU32(b, 0x00, 0x400); PutU32(b, 0x04, 0x400); PutU32(b, 0x08, normalized ? 0x6001u : 0x6000u);
        PutI64(b, 0x10, 0x44); PutU32(b, 0x18, 0); PutI64(b, 0x20, 0x50); PutI64(b, 0x28, 0x54); PutI64(b, 0x30, 0x58);
        PutU32(b, 0x88, 11); PutU32(b, 0x8C, 22); PutU32(b, 0xA4, 0x105); PutU32(b, 0xA8, 7);
        int at = 0x100;
        string[] values = { "C:\\lab\\", image, command, "selftest-title" };
        int[] offsets = { 0x38, 0x60, 0x70, 0xB0 };
        for (int i = 0; i < values.Length; i++)
        {
            byte[] s = Encoding.Unicode.GetBytes(values[i]);
            Array.Copy(s, 0, b, at, s.Length);
            Array.Copy(BitConverter.GetBytes((ushort)s.Length), 0, b, offsets[i], 2);
            Array.Copy(BitConverter.GetBytes((ushort)(s.Length + 2)), 0, b, offsets[i] + 2, 2);
            PutI64(b, offsets[i] + 8, normalized ? m.Base + at : at);
            at += s.Length + 2;
        }
        m.Bytes = b;
        return m;
    }

    // Cases on synthetic blocks; runs anywhere (no Windows call). Prints one line per case.
    public static List<string> ParseSelfTest(out int failures)
    {
        List<string> lines = new List<string>();
        failures = 0;
        string image = "C:\\lab\\bridge_mcp.exe";
        string command = "\"C:\\lab\\bridge_mcp.exe\" --confirm-journal";
        for (int pass = 0; pass < 2; pass++)
        {
            bool normalized = pass == 0;
            FakeMemory m = Synthetic(normalized, image, command);
            Params p = ParseParams(m, m.Base);
            bool ok = p.Failure == null && p.WindowFlags == 0x105 && p.ShowWindowFlags == 7 && p.StartingX == 11
                && p.StartingY == 22 && p.ImagePath == image && p.CommandLine == command && p.WindowTitle == "selftest-title"
                && p.ConsoleHandle == 0x44 && p.StdErr == 0x58 && p.DesktopInfo == "" && p.ShellInfo == "";
            lines.Add("case parse-" + (normalized ? "normalized" : "offsets") + ": " + (ok ? "right" : "WRONG " + p.Failure));
            if (!ok) { failures++; }
        }
        FakeMemory s1 = Synthetic(true, image, command);
        PutU32(s1.Bytes, 0x04, 0x40);
        Params q1 = ParseParams(s1, s1.Base);
        bool r1 = q1.Failure != null && q1.Failure.StartsWith("LAYOUT_CHECK_FAILED at=parameters");
        lines.Add("case short-length: " + (r1 ? "right" : "WRONG " + q1.Failure)); if (!r1) { failures++; }
        FakeMemory s2 = Synthetic(true, image, command);
        s2.Bytes[0x70] = 3;
        Params q2 = ParseParams(s2, s2.Base);
        bool r2 = q2.Failure != null && q2.Failure.StartsWith("LAYOUT_CHECK_FAILED at=command_line");
        lines.Add("case odd-string-length: " + (r2 ? "right" : "WRONG " + q2.Failure)); if (!r2) { failures++; }
        FakeMemory s3 = Synthetic(true, image, command);
        PutI64(s3.Bytes, 0x60 + 8, 0x10);
        Params q3 = ParseParams(s3, s3.Base);
        bool r3 = q3.Failure != null && q3.Failure.StartsWith("READ_FAILED at=image_path");
        lines.Add("case string-outside-memory: " + (r3 ? "right" : "WRONG " + q3.Failure)); if (!r3) { failures++; }
        FakeMemory s4 = Synthetic(true, image, command);
        s4.Cut = 0x80;
        Params q4 = ParseParams(s4, s4.Base);
        bool r4 = q4.Failure != null && q4.Failure.StartsWith("PARTIAL_READ at=parameters");
        lines.Add("case partial-block: " + (r4 ? "right" : "WRONG " + q4.Failure)); if (!r4) { failures++; }
        FakeMemory s5 = new FakeMemory(); s5.Base = 0x1000; s5.Bytes = new byte[16];
        Params q5 = ParseParams(s5, 0x9000);
        bool r5 = q5.Failure != null && q5.Failure.StartsWith("READ_FAILED at=parameters");
        lines.Add("case unreadable-block: " + (r5 ? "right" : "WRONG " + q5.Failure)); if (!r5) { failures++; }
        bool r6 = FlagText(0x105) == "USESHOWWINDOW|USEPOSITION|USESTDHANDLES" && FlagText(0) == "none"
            && FlagText(0x10001) == "USESHOWWINDOW|other=0x10000" && ShowText(7) == "SW_SHOWMINNOACTIVE" && ShowText(99) == "unknown";
        lines.Add("case flag-names: " + (r6 ? "right" : "WRONG")); if (!r6) { failures++; }
        byte[] pe = new byte[512];
        pe[0] = (byte)'M'; pe[1] = (byte)'Z'; PutU32(pe, 0x3C, 0x80);
        pe[0x80] = (byte)'P'; pe[0x81] = (byte)'E';
        Array.Copy(BitConverter.GetBytes((ushort)3), 0, pe, 0x80 + 24 + 68, 2);
        bool r7 = SubsystemOf(pe, pe.Length) == "3 (WINDOWS_CUI)" && SubsystemOf(pe, 0x30).StartsWith("LAYOUT_CHECK_FAILED");
        Array.Copy(BitConverter.GetBytes((ushort)2), 0, pe, 0x80 + 24 + 68, 2);
        r7 = r7 && SubsystemOf(pe, pe.Length) == "2 (WINDOWS_GUI)";
        lines.Add("case pe-subsystem: " + (r7 ? "right" : "WRONG")); if (!r7) { failures++; }
        FakeMemory s6 = Synthetic(true, image, command);
        List<string> brief = Describe(ParseParams(s6, s6.Base), false);
        bool r8 = brief.Contains("image_name=[bridge_mcp.exe] (path, command line, folder and title not printed: chars "
            + command.Length + ", 7, 14)") && !Array.Exists(brief.ToArray(), delegate(string l)
            { return l.StartsWith("command_line=") || l.StartsWith("image_path=") || l.StartsWith("current_directory="); });
        lines.Add("case brief-ancestor: " + (r8 ? "right" : "WRONG")); if (!r8) { failures++; }
        bool r9 = SameImage("C:/lab/bridge_mcp.exe", "C:\\lab\\bridge_mcp.exe")
            && SameImage("\\\\?\\C:\\Lab\\Bridge_MCP.exe", "c:\\lab\\bridge_mcp.exe")
            && SameImage("\\??\\C:\\lab\\bridge_mcp.exe", "C:\\lab\\bridge_mcp.exe")
            && !SameImage("C:/lab/other.exe", "C:\\lab\\bridge_mcp.exe")
            && !SameImage(null, "C:\\lab\\bridge_mcp.exe") && !SameImage("", "")
            && !SameImage("C:\\lab\\bridge_mcp.exe", null);
        lines.Add("case same-image: " + (r9 ? "right" : "WRONG")); if (!r9) { failures++; }
        bool r10;
        try
        {
            List<string> unread = Describe(new Params(), false);
            List<string> unreadFull = Describe(new Params(), true);
            r10 = unread.Contains("image_name=[<unread>] (path, command line, folder and title not printed: chars <unread>, <unread>, <unread>)")
                && unreadFull.Contains("image_path=[<unread>]");
        }
        catch (Exception) { r10 = false; }
        lines.Add("case unread-strings: " + (r10 ? "right" : "WRONG")); if (!r10) { failures++; }
        lines.Add("parse cases 12, wrong " + failures);
        return lines;
    }

    // Live self-test (Windows only): this process against its own startup information, then three children of
    // ping.exe started with known values; each child is read, then left to exit by itself (never ended). The
    // third is started as Python's subprocess starts the server: no application name, and a command line that
    // begins with the exe's path in forward slashes, so its read passes only if the image check accepts it.
    public static List<string> LiveSelfTest(out int failures)
    {
        List<string> lines = new List<string>();
        failures = 0;
        int own = GetCurrentProcessId();
        Reading self = ReadProcess(own);
        StartupInfoRaw si;
        GetStartupInfoW(out si);
        string commandLine = Marshal.PtrToStringUni(GetCommandLineW());
        bool selfOk = self.Ok && self.P.WindowFlags == (uint)si.dwFlags && self.P.ShowWindowFlags == (ushort)si.wShowWindow
            && self.P.CommandLine == commandLine;
        lines.Add("case self: " + (selfOk ? "right" : "WRONG") + " (startupinfo flags=0x" + ((uint)si.dwFlags).ToString("X")
            + " show=" + si.wShowWindow + ")");
        if (!selfOk) { failures++; foreach (string l in self.Lines) { lines.Add("  " + l); } }
        string ping = System.IO.Path.Combine(Environment.SystemDirectory, "PING.EXE");
        string pingForward = ping.Replace('\\', '/');
        for (int c = 0; c < 3; c++)
        {
            StartupInfoW start = new StartupInfoW();
            start.cb = Marshal.SizeOf(typeof(StartupInfoW));
            if (c == 0)
            {
                start.dwFlags = 0x5; start.wShowWindow = 7; start.dwX = 11; start.dwY = 22;
                start.lpTitle = "startinfo-selftest-a";
            }
            ProcessInformation pi;
            StringBuilder cmd = new StringBuilder(c == 2 ? (pingForward + " -n 6 127.0.0.1") : "ping.exe -n 6 127.0.0.1");
            bool made = CreateProcessW(c == 2 ? null : ping, cmd, IntPtr.Zero, IntPtr.Zero, false, 0x08000000, IntPtr.Zero, null,
                ref start, out pi);
            string label = c == 0 ? "child-a (flags 0x5, show 7, x 11, y 22, title)"
                : (c == 1 ? "child-b (flags 0)" : "child-c (no application name, forward-slash command line)");
            if (!made)
            {
                lines.Add("case " + label + ": WRONG CreateProcess error=" + Marshal.GetLastWin32Error());
                failures++;
                continue;
            }
            System.Threading.Thread.Sleep(500);
            Reading r = ReadProcess(pi.dwProcessId);
            bool right;
            if (c == 0)
            {
                right = r.Ok && (r.P.WindowFlags & 0x5) == 0x5 && r.P.ShowWindowFlags == 7 && r.P.StartingX == 11
                    && r.P.StartingY == 22 && r.P.WindowTitle == "startinfo-selftest-a";
            }
            else if (c == 1)
            {
                right = r.Ok && (r.P.WindowFlags & 0x1) == 0;
            }
            else
            {
                right = r.Ok;
                if (r.P != null) { lines.Add("  child-c block image_path=[" + Cap(r.P.ImagePath, 120) + "]"); }
            }
            if (r.Ok) { lines.Add("  read window_flags=0x" + r.P.WindowFlags.ToString("X") + " show=" + r.P.ShowWindowFlags); }
            lines.Add("case " + label + ": " + (right ? "right" : "WRONG"));
            if (!right) { failures++; foreach (string l in r.Lines) { lines.Add("  " + l); } }
            uint waited = WaitForSingleObject(pi.hProcess, 30000);
            uint code;
            string exit = waited == 0 && GetExitCodeProcess(pi.hProcess, out code) ? ("exited code " + code)
                : "NOT EXITED within 30 s (left running, not ended)";
            lines.Add("  child pid " + pi.dwProcessId + " " + exit);
            CloseHandle(pi.hThread);
            CloseHandle(pi.hProcess);
        }
        lines.Add("live cases 4, wrong " + failures);
        return lines;
    }
}
