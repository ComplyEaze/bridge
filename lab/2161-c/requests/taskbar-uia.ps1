# taskbar-uia.ps1 (row 2161-C v2) - the taskbar's buttons as UI Automation lists them, read through the COM client in
# UIAutomationCore.dll (CUIAutomation8, else CUIAutomation; IUIAutomation), not the managed System.Windows.Automation client.
# In v1 the managed client read buttons=0 and in_task_list=0 on the lab's Windows 10 while PrintWindow showed 6 running
# programs' buttons; that the COM client reaches them is unverified, and B2 checks it before any cell runs.
# Read-only: it changes no window, sends no key or click, invokes no pattern, ends nothing and sends nothing to Tally.
# Written on a Mac and NOT RUN anywhere; its C# part was compiled there with Mono's mcs (no errors or warnings), the
# PowerShell around it was not run. Every slot it calls, and every GUID, was checked against Wine's uiautomationclient.idl.
# Per-monitor DPI aware (SetProcessDpiAwarenessContext(-4)), so every rectangle is in physical pixels.
# Writes to -Out (a new file each time; it refuses one that exists), one line each:
#   E <stamp> dpi_context=ok|refused tray=0x.. client=com8|com
#   B <stamp> n=<n> parent_class=.. class=.. offscreen=0|1 rect=l,t,r,b|empty name=...
#                       every element of control type Button under the taskbar (Shell_TrayWnd), in the order UI Automation
#                       finds them; parent_class MSTaskListWClass marks a running application's button on Windows 10
#   Z <stamp> buttons=<n> in_task_list=<n>
#                       in_task_list counts the B lines whose parent_class is MSTaskListWClass
# The lines are v1's, with client= added to E.
param([Parameter(Mandatory = $true)][string]$Out)
if (Test-Path $Out) { throw ('exists already: ' + $Out) }
Add-Type -TypeDefinition @"
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

// Only the vtable slots this read calls are typed; each _n placeholder keeps the slot order of UIAutomationClient.h
// and is never called.
[ComImport, Guid("352ffba8-0973-437c-a61f-f64cafd81df9"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomationCondition { }

[ComImport, Guid("14314595-b4bc-4055-95f2-58f2e42c9855"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomationElementArray {
    int Length();                                         // 0 get_Length
    IUIAutomationElement GetElement(int index);           // 1
}

[ComImport, Guid("d22108aa-8ac5-49a5-837b-37bbb3d7591e"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomationElement {
    void _0(); void _1(); void _2();                      // SetFocus, GetRuntimeId, FindFirst
    IUIAutomationElementArray FindAll(int scope, IUIAutomationCondition condition);   // 3
    void _4(); void _5(); void _6();                      // FindFirstBuildCache, FindAllBuildCache, BuildUpdatedCache
    [return: MarshalAs(UnmanagedType.Struct)] object GetCurrentPropertyValue(int propertyId);   // 7
}

[ComImport, Guid("4042c624-389c-4afc-a630-9df854a541fc"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomationTreeWalker {
    IUIAutomationElement GetParentElement(IUIAutomationElement element);   // 0
}

[ComImport, Guid("30cbe57d-d9d0-452a-ab13-7ac5ac4825ee"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomation {
    void _0(); void _1(); void _2();                      // CompareElements, CompareRuntimeIds, GetRootElement
    IUIAutomationElement ElementFromHandle(IntPtr hwnd);  // 3
    void _4(); void _5(); void _6(); void _7(); void _8(); void _9(); void _10();
                                                          // ElementFromPoint .. CreateTreeWalker
    IUIAutomationTreeWalker ControlViewWalker();          // 11 get_ControlViewWalker
    void _12(); void _13(); void _14(); void _15(); void _16(); void _17(); void _18(); void _19();
                                                          // get_ContentViewWalker .. CreateFalseCondition
    IUIAutomationCondition CreatePropertyCondition(int propertyId, [MarshalAs(UnmanagedType.Struct)] object value);   // 20
}

public static class TUia {
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr FindWindow(string cls, string name);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr value);
    public static bool MakeAware() { return SetProcessDpiAwarenessContext(new IntPtr(-4)); }

    const int BoundingRectangle = 30001, ControlType = 30003, Name = 30005, ClassName = 30012, IsOffscreen = 30022;
    const int ButtonControlType = 50000, TreeScopeDescendants = 4;

    static string Stamp() { return DateTime.UtcNow.ToString("yyyy-MM-ddTHH:mm:ss.fffZ"); }
    static string OneLine(object value) {
        string s = value as string;
        return s == null ? "" : s.Replace("\r", " ").Replace("\n", " ");
    }

    // client: "com8" when CUIAutomation8 was created, else "com" (CUIAutomation).
    public static IUIAutomation Create(out string client) {
        Type t = Type.GetTypeFromCLSID(new Guid("e22ad333-b25f-460c-83d0-0581107395c9"), false);
        object o = null;
        try { if (t != null) { o = Activator.CreateInstance(t); } } catch (COMException) { o = null; }
        client = "com8";
        if (o == null) {
            o = Activator.CreateInstance(Type.GetTypeFromCLSID(new Guid("ff48dba4-60ef-4201-aa87-54103eef594e"), true));
            client = "com";
        }
        return (IUIAutomation)o;
    }

    public static List<string> Buttons(IUIAutomation uia, IntPtr tray) {
        var lines = new List<string>();
        IUIAutomationElement root = uia.ElementFromHandle(tray);
        IUIAutomationCondition cond = uia.CreatePropertyCondition(ControlType, ButtonControlType);
        IUIAutomationElementArray found = root.FindAll(TreeScopeDescendants, cond);
        IUIAutomationTreeWalker walker = uia.ControlViewWalker();
        int count = found == null ? 0 : found.Length();
        int inList = 0;
        for (int i = 0; i < count; i++) {
            IUIAutomationElement el = found.GetElement(i);
            IUIAutomationElement parent = walker.GetParentElement(el);
            string pcls = parent == null ? "" : OneLine(parent.GetCurrentPropertyValue(ClassName));
            if (pcls == "MSTaskListWClass") { inList++; }
            string rect = "empty";
            double[] b = el.GetCurrentPropertyValue(BoundingRectangle) as double[];
            if (b != null && b.Length == 4 && (b[2] != 0 || b[3] != 0)) {
                rect = string.Format("{0},{1},{2},{3}", (int)b[0], (int)b[1], (int)(b[0] + b[2]), (int)(b[1] + b[3]));
            }
            object off = el.GetCurrentPropertyValue(IsOffscreen);
            int offscreen = (off is bool && (bool)off) ? 1 : 0;
            lines.Add(string.Format("B {0} n={1} parent_class={2} class={3} offscreen={4} rect={5} name={6}",
                Stamp(), i + 1, pcls, OneLine(el.GetCurrentPropertyValue(ClassName)), offscreen, rect,
                OneLine(el.GetCurrentPropertyValue(Name))));
        }
        lines.Add(string.Format("Z {0} buttons={1} in_task_list={2}", Stamp(), count, inList));
        return lines;
    }
}
"@
$dpi = if ([TUia]::MakeAware()) { 'ok' } else { 'refused' }
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
$lines = New-Object System.Collections.Generic.List[string]
$tray = [TUia]::FindWindow('Shell_TrayWnd', $null)
if ($tray -eq [IntPtr]::Zero) { throw 'no Shell_TrayWnd' }
$client = ''
$uia = [TUia]::Create([ref]$client)
$lines.Add("E $(Stamp) dpi_context=$dpi tray=0x$($tray.ToInt64().ToString('x')) client=$client")
foreach ($l in [TUia]::Buttons($uia, $tray)) { $lines.Add($l) }
[System.IO.File]::WriteAllText($Out, ($lines -join "`r`n") + "`r`n", [System.Text.Encoding]::UTF8)
$lines
