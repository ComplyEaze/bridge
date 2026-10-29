// SPDX-License-Identifier: Apache-2.0
//! Whether the native approval box lies wholly inside the work area of the
//! monitor it opened on (#1832, #1780).
//!
//! A Windows message box cannot scroll. Measured in the lab (1920x1080 at 125
//! percent, work area 1030 high, this module's DPI-aware process): a box of 44
//! lines is 1,011 high and fits; boxes of 48 and 60 lines are 1,087 and 1,315
//! high, placed with their tops at 0 and their buttons off the screen, and
//! Windows neither clipped nor shrank them. A person who can see
//! only the top of such a box could answer it with `Y` having read none of the
//! lines below the screen's edge. The caps on a text's lines are a guess about
//! the screen; this measures the real box when it opens, in the dialog's own
//! process, and refuses when it is not wholly on the screen.
//!
//! Units. The executable's manifest declares no DPI awareness, so Windows
//! would draw the box DPI-unaware and report its rectangle and its monitor's
//! work area in virtualised units. The first run on a real machine (125
//! percent scale, the box measured physically at 588,116,1353,990 inside a
//! work area 1030 high) was refused as not fitting, which no consistent pair
//! of units can explain; so this module does not rely on the virtualisation.
//! Before it shows anything the dialog process makes itself per-monitor DPI
//! aware (version 2), so that the rectangle and the work area are both
//! physical pixels on every monitor, whatever its scale, and a process that
//! cannot be made so shows no box. A rectangle is compared as it is given,
//! with no slack either way.
//!
//! Three answers leave this module without the person's choosing: the box did
//! not fit (the child exits [`DOES_NOT_FIT_EXIT`], which the parent reads as a
//! text too large for the box), the box could not be measured or shown (exit
//! 0 with no token, which the parent reads as unavailable), and a fitting
//! box's own answer. A `Yes` can never pass for a box that did not fit.
#![cfg_attr(all(not(windows), not(test)), allow(dead_code))]

/// The exit status of the dialog child when its box did not fit the screen.
/// The parent reads it, whatever else the child printed, as a refusal. It is
/// not 3, which the C runtime's `abort` also exits with, nor 101, which a Rust
/// panic exits with: a crashed dialog must not read as a box that was too big.
pub(crate) const DOES_NOT_FIT_EXIT: i32 = 77;

/// A rectangle in screen pixels, as `GetWindowRect` and a monitor's work area
/// give them: `right` and `bottom` are one past the last pixel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Rect {
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
}

impl Rect {
    fn has_area(self) -> bool {
        self.right > self.left && self.bottom > self.top
    }

    /// Whether `self` lies wholly inside `work`. An empty rectangle on either
    /// side is never inside: a box or an area that was not read properly is
    /// not a fit.
    pub(crate) fn is_wholly_inside(self, work: Rect) -> bool {
        self.has_area()
            && work.has_area()
            && self.left >= work.left
            && self.top >= work.top
            && self.right <= work.right
            && self.bottom <= work.bottom
    }
}

/// What the hook saw of the box: nothing yet, a box that fits, or one that
/// does not. Once a box has been seen not to fit it stays so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Seen {
    Nothing,
    Fits,
    DoesNotFit,
}

impl Seen {
    /// The state after one more measurement of a box.
    pub(crate) fn after(self, fits: bool) -> Seen {
        match (self, fits) {
            (Seen::DoesNotFit, _) | (_, false) => Seen::DoesNotFit,
            _ => Seen::Fits,
        }
    }
}

/// The result `MessageBoxW` gives for "the box could not be shown".
const MESSAGEBOX_FAILED: i32 = 0;

/// What the dialog does with the box's result and what the hook saw.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    /// The box fitted: its result is the person's answer.
    Answered(i32),
    /// The box did not fit, whatever the result: exit [`DOES_NOT_FIT_EXIT`].
    DoesNotFit,
    /// The box could not be shown or was never measured: exit 0 with no
    /// token, which the parent reads as unavailable.
    Unavailable,
}

/// Decide from the box's result and what the hook saw. The order matters: a
/// box seen not to fit is a refusal before anything else is looked at, so no
/// result, a `Yes` included, can approve it.
pub(crate) fn outcome(result: i32, seen: Seen) -> Outcome {
    match seen {
        Seen::DoesNotFit => Outcome::DoesNotFit,
        Seen::Nothing => Outcome::Unavailable,
        Seen::Fits if result == MESSAGEBOX_FAILED => Outcome::Unavailable,
        Seen::Fits => Outcome::Answered(result),
    }
}

/// Show a box with `show` (which returns `MessageBoxW`'s result) under the
/// fit check, and return the result only for a box that fitted. Never returns
/// for one that did not: the process exits with a status the parent reads.
/// `show` is given the box's owner: a hidden topmost window this thread makes
/// for the box, or null if Windows refuses one (#1780).
#[cfg(windows)]
pub(crate) fn guarded(show: impl FnOnce(windows_sys::Win32::Foundation::HWND) -> i32) -> i32 {
    if !windows_hook::make_dpi_aware() {
        // Rectangles could not be read in one unit, so no box is shown. Said on
        // the error output, so a run by hand on an older Windows names the cause.
        use std::io::Write;
        let _ = writeln!(
            std::io::stderr(),
            "box-fit: not per-monitor DPI aware (Windows 10 version 1703 or later is needed): no box shown"
        );
        std::process::exit(0);
    }
    let Some(hook) = windows_hook::install() else {
        // No way to measure the box, so none is shown.
        std::process::exit(0);
    };
    let front = windows_hook::Front::start();
    // A box owned by a topmost window stays above Tally where its own topmost
    // flag was seen not to be kept (#1780). Made after the hook, on this
    // thread, never shown and never activated, so the fit check measures only
    // the box; destroyed once the box has returned.
    let owner = windows_hook::TopmostOwner::create();
    let result = show(
        owner
            .as_ref()
            .map_or(std::ptr::null_mut(), windows_hook::TopmostOwner::handle),
    );
    drop(owner);
    let seen = windows_hook::seen();
    drop(front);
    drop(hook);
    match outcome(result, seen) {
        Outcome::Answered(result) => result,
        Outcome::DoesNotFit => std::process::exit(DOES_NOT_FIT_EXIT),
        Outcome::Unavailable => std::process::exit(0),
    }
}

#[cfg(windows)]
mod windows_hook {
    use super::{Rect, Seen};
    use std::ffi::c_void;
    use std::io::Write;
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use windows_sys::core::BOOL;
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows_sys::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        BringWindowToTop, CallNextHookEx, CreateWindowExW, DestroyWindow, EnumThreadWindows,
        FlashWindowEx, GetForegroundWindow, GetWindowRect, GetWindowThreadProcessId,
        IsWindowVisible, PostMessageW, SetForegroundWindow, SetWindowsHookExW, UnhookWindowsHookEx,
        FLASHWINFO, FLASHW_ALL, FLASHW_TIMERNOFG, HCBT_ACTIVATE, HHOOK, IDCANCEL, WH_CBT,
        WM_COMMAND, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
    };

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleW(name: *const u16) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *const c_void;
    }

    /// `DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2`, a pseudo handle.
    const PER_MONITOR_AWARE_V2: isize = -4;

    /// Make this process per-monitor DPI aware (version 2), before any window
    /// exists, so that a window's rectangle and its monitor's work area are
    /// both physical pixels on every monitor. `false` if the process is not
    /// in that mode afterwards: the three functions are looked up when they
    /// are called, so that a Windows that lacks them (before Windows 10
    /// version 1703) still starts the program and simply shows no box.
    pub(super) fn make_dpi_aware() -> bool {
        let user32: Vec<u16> = "user32.dll\0".encode_utf16().collect();
        // SAFETY: `user32` is a NUL-terminated wide string that outlives the
        // call; the module is already loaded by the program's own imports.
        let module = unsafe { GetModuleHandleW(user32.as_ptr()) };
        if module.is_null() {
            return false;
        }
        let find = |name: &'static [u8]| -> *const c_void {
            debug_assert_eq!(name.last(), Some(&0));
            // SAFETY: `module` is a loaded module and `name` is a
            // NUL-terminated ASCII name.
            unsafe { GetProcAddress(module, name.as_ptr()) }
        };
        let set = find(b"SetProcessDpiAwarenessContext\0");
        let get = find(b"GetThreadDpiAwarenessContext\0");
        let same = find(b"AreDpiAwarenessContextsEqual\0");
        if set.is_null() || get.is_null() || same.is_null() {
            return false;
        }
        // SAFETY: each pointer is the address of the exported function of
        // that name, whose signature is the one given here (user32.dll).
        unsafe {
            let set = std::mem::transmute::<
                *const c_void,
                unsafe extern "system" fn(*mut c_void) -> i32,
            >(set);
            let get = std::mem::transmute::<
                *const c_void,
                unsafe extern "system" fn() -> *mut c_void,
            >(get);
            let same = std::mem::transmute::<
                *const c_void,
                unsafe extern "system" fn(*mut c_void, *mut c_void) -> i32,
            >(same);
            set(PER_MONITOR_AWARE_V2 as *mut c_void);
            same(get(), PER_MONITOR_AWARE_V2 as *mut c_void) != 0
        }
    }

    /// A helper thread that, once, when the box first appears, tries to bring
    /// it to the front, and flashes its taskbar button if Windows refuses. A
    /// background process is often refused the foreground, and a box nobody
    /// sees is a box nobody answers. It never takes the foreground back after
    /// that single attempt, so a person who moves to another window is not
    /// pulled away from it. It stops when dropped.
    pub(super) struct Front {
        stop: Arc<AtomicBool>,
        worker: Option<std::thread::JoinHandle<()>>,
    }

    impl Front {
        /// Start the helper for the box this thread is about to show.
        pub(super) fn start() -> Front {
            let stop = Arc::new(AtomicBool::new(false));
            // SAFETY: no arguments; reads this thread's id.
            let owner = unsafe { GetCurrentThreadId() };
            let flag = Arc::clone(&stop);
            let worker = std::thread::Builder::new()
                .spawn(move || raise_once_shown(owner, &flag))
                .ok();
            Front { stop, worker }
        }
    }

    impl Drop for Front {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }

    /// The box's owner (#1780): a hidden, topmost tool window of the box's own
    /// thread, never shown, destroyed when dropped. A window owned by a topmost
    /// window is kept above every window that is not topmost; the owner is a
    /// tool window and the box an owned one, so neither has a taskbar button.
    pub(super) struct TopmostOwner(HWND);

    impl TopmostOwner {
        /// A new owner, or `None` if Windows refuses to make one.
        pub(super) fn create() -> Option<Self> {
            let class: Vec<u16> = "Static".encode_utf16().chain(Some(0)).collect();
            // SAFETY: a system class's NUL-terminated name that outlives the
            // call, no title, no parent, menu, instance or creation data; the
            // handle returned is checked before it is used.
            let window = unsafe {
                CreateWindowExW(
                    WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
                    class.as_ptr(),
                    std::ptr::null(),
                    WS_POPUP,
                    0,
                    0,
                    0,
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                )
            };
            (!window.is_null()).then_some(Self(window))
        }

        /// The owner's handle, for `MessageBoxW`.
        pub(super) fn handle(&self) -> HWND {
            self.0
        }
    }

    impl Drop for TopmostOwner {
        fn drop(&mut self) {
            // SAFETY: the handle came from `CreateWindowExW` on this thread and
            // is destroyed once, here.
            unsafe {
                DestroyWindow(self.0);
            }
        }
    }

    unsafe extern "system" fn first_visible(window: HWND, found: LPARAM) -> BOOL {
        // SAFETY: `found` is the address of the `HWND` slot `box_of` passed.
        let slot = unsafe { &mut *(found as *mut HWND) };
        // SAFETY: `window` is a handle Windows is enumerating.
        if unsafe { IsWindowVisible(window) } != 0 {
            *slot = window;
            0
        } else {
            1
        }
    }

    /// The first visible top-level window of `thread`, or null.
    fn box_of(thread: u32) -> HWND {
        let mut found: HWND = std::ptr::null_mut();
        // SAFETY: the callback only writes the slot whose address is passed,
        // which outlives the call.
        unsafe {
            EnumThreadWindows(
                thread,
                Some(first_visible),
                &mut found as *mut HWND as LPARAM,
            );
        }
        found
    }

    /// Try to make `window` the foreground window; if Windows refuses, flash
    /// its taskbar button until it is.
    fn raise(window: HWND) {
        // SAFETY: every handle is one just read from Windows; the input
        // attachment is undone before returning.
        unsafe {
            let foreground = GetForegroundWindow();
            if foreground == window {
                return;
            }
            let me = GetCurrentThreadId();
            let other = if foreground.is_null() {
                0
            } else {
                GetWindowThreadProcessId(foreground, std::ptr::null_mut())
            };
            let attached = other != 0 && other != me && AttachThreadInput(me, other, 1) != 0;
            SetForegroundWindow(window);
            BringWindowToTop(window);
            if attached {
                AttachThreadInput(me, other, 0);
            }
            if GetForegroundWindow() != window {
                let flash = FLASHWINFO {
                    cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
                    hwnd: window,
                    dwFlags: FLASHW_ALL | FLASHW_TIMERNOFG,
                    uCount: 0,
                    dwTimeout: 0,
                };
                FlashWindowEx(&flash);
            }
        }
    }

    fn raise_once_shown(owner: u32, stop: &AtomicBool) {
        // About ten seconds to appear: the box has long appeared by then.
        for _ in 0..100 {
            if stop.load(Ordering::SeqCst) {
                return;
            }
            let window = box_of(owner);
            if !window.is_null() {
                raise(window);
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// What the hook has seen, for this process's one box (0 nothing, 1 fits,
    /// 2 does not fit).
    static SEEN: AtomicU8 = AtomicU8::new(0);

    pub(super) fn seen() -> Seen {
        match SEEN.load(Ordering::SeqCst) {
            1 => Seen::Fits,
            2 => Seen::DoesNotFit,
            _ => Seen::Nothing,
        }
    }

    fn remember(seen: Seen) {
        SEEN.store(
            match seen {
                Seen::Nothing => 0,
                Seen::Fits => 1,
                Seen::DoesNotFit => 2,
            },
            Ordering::SeqCst,
        );
    }

    /// The hook, removed when it is dropped.
    pub(super) struct Hook(HHOOK);

    impl Drop for Hook {
        fn drop(&mut self) {
            // SAFETY: the handle came from `SetWindowsHookExW` and is removed
            // once, here.
            unsafe {
                UnhookWindowsHookEx(self.0);
            }
        }
    }

    /// Install a computer-based-training hook on this thread, the one that is
    /// about to show the box. `None` if Windows refuses.
    pub(super) fn install() -> Option<Hook> {
        remember(Seen::Nothing);
        // SAFETY: `on_activate` is an `extern "system"` function with the
        // hook procedure's signature; a null module handle with this thread's
        // id hooks this thread only.
        let handle = unsafe {
            SetWindowsHookExW(
                WH_CBT,
                Some(on_activate),
                std::ptr::null_mut(),
                GetCurrentThreadId(),
            )
        };
        (!handle.is_null()).then_some(Hook(handle))
    }

    /// The rectangle of `window` and the work area of the monitor nearest to
    /// it, in the same unit. `None` if either cannot be read.
    fn measure(window: HWND) -> Option<(Rect, Rect)> {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        // SAFETY: `window` is the window being activated and `rect` is a
        // live out-parameter of the right type.
        if unsafe { GetWindowRect(window, &mut rect) } == 0 {
            return None;
        }
        // SAFETY: as above.
        let monitor = unsafe { MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST) };
        if monitor.is_null() {
            return None;
        }
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            rcWork: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            dwFlags: 0,
        };
        // SAFETY: `monitor` is a handle just returned for this window and
        // `info` has its `cbSize` set, as the call requires.
        if unsafe { GetMonitorInfoW(monitor, &mut info) } == 0 {
            return None;
        }
        let rect = Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        };
        let work = Rect {
            left: info.rcWork.left,
            top: info.rcWork.top,
            right: info.rcWork.right,
            bottom: info.rcWork.bottom,
        };
        Some((rect, work))
    }

    /// Whether `window` lies wholly inside the work area of the monitor
    /// nearest to it. Any failure to read either is "does not fit". One line
    /// of the numbers goes to the process's own error output (the parent
    /// discards it; a person running the dialog by hand can read it).
    fn fits(window: HWND) -> bool {
        let measured = measure(window);
        let fitted = measured.is_some_and(|(rect, work)| rect.is_wholly_inside(work));
        let _ = match measured {
            Some((r, w)) => writeln!(
                std::io::stderr(),
                "box-fit: box=({},{},{},{}) work=({},{},{},{}) fits={fitted}",
                r.left,
                r.top,
                r.right,
                r.bottom,
                w.left,
                w.top,
                w.right,
                w.bottom
            ),
            None => writeln!(std::io::stderr(), "box-fit: not readable fits=false"),
        };
        fitted
    }

    /// The hook procedure: when the box is about to be activated, measure it;
    /// if it does not fit, post it a Cancel and remember that it did not.
    unsafe extern "system" fn on_activate(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code == HCBT_ACTIVATE as i32 {
            let window = wparam as HWND;
            let fitted = fits(window);
            remember(seen().after(fitted));
            if !fitted {
                // SAFETY: `window` is the box being activated; Cancel is a
                // button every box this module guards has.
                unsafe {
                    PostMessageW(window, WM_COMMAND, IDCANCEL as WPARAM, 0);
                }
            }
        }
        // SAFETY: passing the hook's own arguments on, with no hook handle,
        // is what the documentation prescribes.
        unsafe { CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORK: Rect = Rect {
        left: 0,
        top: 0,
        right: 1920,
        bottom: 1030,
    };

    fn rect(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }

    /// One pixel each side of the work area's edge: a box on the edge fits, a
    /// box one pixel past any edge does not.
    #[test]
    fn a_box_fits_only_wholly_inside_the_work_area() {
        assert!(rect(0, 0, 1920, 1030).is_wholly_inside(WORK));
        assert!(rect(704, 54, 1238, 1025).is_wholly_inside(WORK));
        assert!(!rect(-1, 0, 1920, 1030).is_wholly_inside(WORK));
        assert!(!rect(0, -1, 1920, 1030).is_wholly_inside(WORK));
        assert!(!rect(0, 0, 1921, 1030).is_wholly_inside(WORK));
        assert!(!rect(0, 0, 1920, 1031).is_wholly_inside(WORK));
        // The lab's 60-line box: top pinned at 0, bottom well below the area.
        assert!(!rect(588, 0, 1353, 1134).is_wholly_inside(WORK));
        // A work area that does not start at the origin (a taskbar on top, a
        // second monitor to the left).
        let offset = rect(-1920, 40, 0, 1080);
        assert!(rect(-1500, 100, -900, 900).is_wholly_inside(offset));
        assert!(!rect(-1500, 39, -900, 900).is_wholly_inside(offset));
    }

    /// A rectangle that was not read properly is never a fit, on either side.
    #[test]
    fn an_empty_box_or_area_is_never_a_fit() {
        assert!(!rect(10, 10, 10, 20).is_wholly_inside(WORK));
        assert!(!rect(10, 10, 20, 10).is_wholly_inside(WORK));
        assert!(!rect(20, 20, 10, 10).is_wholly_inside(WORK));
        assert!(!rect(10, 10, 20, 20).is_wholly_inside(rect(0, 0, 0, 0)));
    }

    /// A box once seen not to fit stays not fitting, in any order.
    #[test]
    fn what_the_hook_saw_never_improves() {
        assert_eq!(Seen::Nothing.after(true), Seen::Fits);
        assert_eq!(Seen::Nothing.after(false), Seen::DoesNotFit);
        assert_eq!(Seen::Fits.after(true), Seen::Fits);
        assert_eq!(Seen::Fits.after(false), Seen::DoesNotFit);
        assert_eq!(Seen::DoesNotFit.after(true), Seen::DoesNotFit);
        assert_eq!(Seen::DoesNotFit.after(false), Seen::DoesNotFit);
    }

    /// The result of a box is an answer only when the hook saw it fit. Every
    /// result of a box that did not fit, a Yes (6) included, is a refusal; a
    /// box never measured, or one that could not be shown, is unavailable.
    #[test]
    fn no_result_can_approve_a_box_that_did_not_fit() {
        const IDCANCEL: i32 = 2;
        const IDYES: i32 = 6;
        const IDNO: i32 = 7;
        for result in [0, IDCANCEL, IDYES, IDNO, 1, 99, -1] {
            assert_eq!(
                outcome(result, Seen::DoesNotFit),
                Outcome::DoesNotFit,
                "{result}"
            );
            assert_eq!(
                outcome(result, Seen::Nothing),
                Outcome::Unavailable,
                "{result}"
            );
        }
        assert_eq!(outcome(0, Seen::Fits), Outcome::Unavailable);
        for result in [IDCANCEL, IDYES, IDNO] {
            assert_eq!(outcome(result, Seen::Fits), Outcome::Answered(result));
        }
    }

    /// The exit status the parent reads must not be one a decline, an
    /// approval, an unavailable dialog, a C `abort` or a Rust panic uses.
    #[test]
    fn the_does_not_fit_status_is_its_own() {
        assert!(![0, 1, 2, 3, 101].contains(&DOES_NOT_FIT_EXIT));
    }
    /// The first live run (125 percent scale): the 44-line box, measured
    /// physically at 588,116,1353,990 inside a work area 1920x1030. In one
    /// unit it fits, in physical pixels or divided by the scale; at 100
    /// percent the same shape fits a 1040-high area and the 60-line box does
    /// not. A rectangle in one unit against an area in the other gives either
    /// a refusal of a fitting box (physical box, scaled area) or, worse, the
    /// approval of a box that is too tall (scaled box, physical area): which
    /// is why the module reads both in one unit and shows no box if it cannot.
    #[test]
    fn a_box_is_compared_in_one_unit_at_100_and_125_percent() {
        // 125 percent, physical.
        let work = rect(0, 0, 1920, 1030);
        assert!(rect(588, 116, 1353, 990).is_wholly_inside(work));
        assert!(rect(588, 84, 1353, 1023).is_wholly_inside(work));
        assert!(!rect(588, 0, 1353, 1134).is_wholly_inside(work));
        // 125 percent, the same boxes and area divided by the scale.
        let scaled = rect(0, 0, 1536, 824);
        assert!(rect(470, 93, 1082, 792).is_wholly_inside(scaled));
        assert!(rect(470, 67, 1082, 818).is_wholly_inside(scaled));
        assert!(!rect(470, 0, 1082, 907).is_wholly_inside(scaled));
        // 100 percent.
        let plain = rect(0, 0, 1920, 1040);
        assert!(rect(577, 83, 1342, 957).is_wholly_inside(plain));
        assert!(!rect(577, 0, 1342, 1134).is_wholly_inside(plain));
        // Mixed units: what the module must never compare.
        assert!(!rect(588, 116, 1353, 990).is_wholly_inside(scaled));
        assert!(rect(470, 0, 1082, 907).is_wholly_inside(work));
    }

    /// The box's owner (#1780) is a hidden topmost tool window that exists until
    /// it is dropped. No box is shown.
    #[cfg(windows)]
    #[test]
    fn the_box_s_owner_is_a_hidden_topmost_tool_window_until_it_is_dropped() {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            GetWindowLongW, IsWindow, IsWindowVisible, GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
        };
        let owner = super::windows_hook::TopmostOwner::create().expect("an owner window");
        let window = owner.handle();
        // SAFETY: `window` is a handle this thread just made; each call only reads it.
        let (exists, visible, extended) = unsafe {
            (
                IsWindow(window),
                IsWindowVisible(window),
                GetWindowLongW(window, GWL_EXSTYLE) as u32,
            )
        };
        assert_ne!(exists, 0);
        assert_eq!(visible, 0);
        assert_ne!(extended & WS_EX_TOPMOST, 0);
        assert_ne!(extended & WS_EX_TOOLWINDOW, 0);
        drop(owner);
        // SAFETY: as above; a destroyed handle only reads as no window.
        assert_eq!(unsafe { IsWindow(window) }, 0);
    }
}
