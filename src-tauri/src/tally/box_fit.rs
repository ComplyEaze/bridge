// SPDX-License-Identifier: Apache-2.0
//! Whether the native approval box lies wholly inside the work area of the
//! monitor it opened on (#1832, #1780).
//!
//! A Windows message box cannot scroll. Measured in the lab (1920x1080 at 125
//! percent, work area 1030 high): a box of 48 lines fitted (bottom edge 1023),
//! one of 60 lines was 1,134 high with its top pinned at 0 and its buttons off
//! the screen, and Windows neither clipped nor shrank it. A person who can see
//! only the top of such a box could answer it with `Y` having read none of the
//! lines below the screen's edge. The caps on a text's lines are a guess about
//! the screen; this measures the real box when it opens, in the dialog's own
//! process, and refuses when it is not wholly on the screen.
//!
//! The process draws no DPI-aware window (the manifest declares no DPI
//! awareness; measured by the lab, W1), so Windows virtualises both numbers
//! below by the same factor: a box's rectangle and its monitor's work area are
//! read in the same units, and the comparison needs no scale. A rectangle is
//! therefore compared as it is given, with no slack either way.
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
#[cfg(windows)]
pub(crate) fn guarded(show: impl FnOnce() -> i32) -> i32 {
    let Some(hook) = windows_hook::install() else {
        // No way to measure the box, so none is shown.
        std::process::exit(0);
    };
    let result = show();
    let seen = windows_hook::seen();
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
    use std::sync::atomic::{AtomicU8, Ordering};
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows_sys::Win32::System::Threading::GetCurrentThreadId;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, GetWindowRect, PostMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
        HCBT_ACTIVATE, HHOOK, IDCANCEL, WH_CBT, WM_COMMAND,
    };

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

    /// Whether `window` lies wholly inside the work area of the monitor
    /// nearest to it. Any failure to read either is "does not fit".
    fn fits(window: HWND) -> bool {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        // SAFETY: `window` is the window being activated and `rect` is a
        // live out-parameter of the right type.
        if unsafe { GetWindowRect(window, &mut rect) } == 0 {
            return false;
        }
        // SAFETY: as above.
        let monitor = unsafe { MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST) };
        if monitor.is_null() {
            return false;
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
            return false;
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
        rect.is_wholly_inside(work)
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
}
