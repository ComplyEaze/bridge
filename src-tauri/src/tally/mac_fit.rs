// SPDX-License-Identifier: Apache-2.0
//! The macOS approval alert, measured against the screen it opens on and shown
//! only if it fits (#1832, #1780).
//!
//! An `NSAlert` has no scroll and AppKit neither shrinks nor clamps it. A
//! person who can see only the top of an alert taller than the screen could
//! answer it having read none of the lines below the screen's edge, among them
//! its buttons. Unlike a Windows message box, an alert can be measured before
//! it is shown.
//!
//! This module builds the alert itself, lays it out (`layout()` gives the
//! frame the alert then has; nothing is shown yet), compares its size with the
//! main screen's visible frame, and shows that same object with `runModal`
//! only if it fits. One that does not fit is never shown: the dialog child
//! exits [`DOES_NOT_FIT_EXIT`], which the parent reads as a text too large for
//! the box, as it does on Windows. One that could not be measured is not shown
//! either: exit 0 with no token, which the parent reads as unavailable. The
//! measured alert is the shown alert by construction, so no twin is involved.
//!
//! Before this, the dialogs went through `rfd`, which for a dialog with no
//! parent window shows a system notification panel drawn by another process,
//! not an `NSAlert`; `rfd` remains the answer on other platforms. Which screen
//! the alert opens on is taken to be the main screen at that moment; with
//! several displays only that one is measured. Position is not compared, only
//! size: AppKit centres an alert that fits inside the visible frame.
//!
//! Confidence. VERIFIED here by reading the AppKit bindings: the first button
//! added is the default (Return) and a button titled "Cancel" takes Escape, so
//! both are the first button; the second has no key; only response 1001
//! approves. UNVERIFIED, until the owner runs it at the Mac: that `runModal`
//! shows the frame `layout()` gave (AppKit documents `layout()` only as laying
//! out now instead of just before display), that an alert opened by this
//! background child is in front and visible (it is a modal panel above other
//! applications' windows, but keyboard focus and a visible, un-hidden panel are
//! not measured; failing that it fails closed, timing out after 120 s), that
//! the main screen is the one it opens on, and that a fitting alert is placed
//! wholly inside the visible frame.
#![cfg_attr(all(not(target_os = "macos"), not(test)), allow(dead_code))]

use super::box_fit::DOES_NOT_FIT_EXIT;

/// What the measurement of one alert came to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Measured {
    Fits,
    DoesNotFit,
    Unavailable,
}

/// A size in points, as `NSWindow.frame` and `NSScreen.visibleFrame` give them.
pub(crate) type Size = (f64, f64);

/// Decide from the alert's size and the screen's visible size. Either missing,
/// or not a positive finite number, is unavailable: a size that was not read
/// properly is not a fit, and not a refusal of the text either.
pub(crate) fn measured(alert: Option<Size>, visible: Option<Size>) -> Measured {
    let usable =
        |size: Size| size.0.is_finite() && size.1.is_finite() && size.0 > 0.0 && size.1 > 0.0;
    match (alert, visible) {
        (Some(alert), Some(visible)) if usable(alert) && usable(visible) => {
            if alert.0 <= visible.0 && alert.1 <= visible.1 {
                Measured::Fits
            } else {
                Measured::DoesNotFit
            }
        }
        _ => Measured::Unavailable,
    }
}

/// `NSAlertSecondButtonReturn`: the response of the second button added.
const SECOND_BUTTON: isize = 1001;

/// Whether `response` (an `NSModalResponse`) approves: only the second button
/// does. The first, the window's own close and anything else decline.
pub(crate) fn approves(response: isize) -> bool {
    response == SECOND_BUTTON
}

/// Show the approval alert with `Cancel` first (so Return and Escape cancel)
/// and `positive` second, if it fits the screen, and say whether the person
/// chose `positive`. An alert that does not fit ends the process with the
/// status the parent reads; one that could not be measured ends it with none
/// and no token. One line of the numbers goes to the error output (the parent
/// discards it; a person running the dialog by hand can read it). `None` on
/// a platform where `rfd` shows the dialog.
#[cfg(target_os = "macos")]
pub(crate) fn ask(title: &str, text: &str, positive: &str) -> Option<bool> {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{
        NSAlert, NSAlertStyle, NSApplication, NSApplicationActivationPolicy, NSScreen,
    };
    use objc2_foundation::NSString;
    use std::io::Write;

    // The dialog child runs this on its main thread, before Tokio starts.
    let Some(mtm) = MainThreadMarker::new() else {
        std::process::exit(0);
    };
    let app = NSApplication::sharedApplication(mtm);
    // A process with no Dock presence cannot show an alert in front: as `rfd`
    // does, make it an accessory app for the alert and put the policy back.
    let policy = app.activationPolicy();
    if policy == NSApplicationActivationPolicy::Prohibited {
        app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    }
    let visible = NSScreen::mainScreen(mtm).map(|screen| {
        let frame = screen.visibleFrame();
        (frame.size.width, frame.size.height)
    });
    // The alert this function shows if it fits: measured, then run.
    let alert = NSAlert::new(mtm);
    alert.setAlertStyle(NSAlertStyle::Warning);
    alert.addButtonWithTitle(&NSString::from_str("Cancel"));
    alert.addButtonWithTitle(&NSString::from_str(positive));
    alert.setMessageText(&NSString::from_str(title));
    alert.setInformativeText(&NSString::from_str(text));
    alert.layout();
    let frame = alert.window().frame();
    let size = Some((frame.size.width, frame.size.height));
    let _ = writeln!(
        std::io::stderr(),
        "box-fit: alert={size:?} visible={visible:?}"
    );
    match measured(size, visible) {
        Measured::Fits => {}
        Measured::DoesNotFit => std::process::exit(DOES_NOT_FIT_EXIT),
        Measured::Unavailable => std::process::exit(0),
    }
    let response = alert.runModal();
    if policy == NSApplicationActivationPolicy::Prohibited {
        app.setActivationPolicy(policy);
    }
    Some(approves(response))
}

/// Not macOS: `rfd` shows the dialog.
#[cfg(not(target_os = "macos"))]
pub(crate) fn ask(_title: &str, _text: &str, _positive: &str) -> Option<bool> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const VISIBLE: Size = (1512.0, 865.0);

    /// The lab's own numbers: 27 lines fit the visible frame, 30 and 43 do not.
    #[test]
    fn an_alert_fits_only_inside_the_visible_frame() {
        assert_eq!(
            measured(Some((520.0, 854.0)), Some(VISIBLE)),
            Measured::Fits
        );
        assert_eq!(
            measured(Some((520.0, 865.0)), Some(VISIBLE)),
            Measured::Fits
        );
        assert_eq!(
            measured(Some((520.0, 865.5)), Some(VISIBLE)),
            Measured::DoesNotFit
        );
        assert_eq!(
            measured(Some((520.0, 934.0)), Some(VISIBLE)),
            Measured::DoesNotFit
        );
        assert_eq!(
            measured(Some((520.0, 1302.0)), Some(VISIBLE)),
            Measured::DoesNotFit
        );
        // Too wide is a refusal as well.
        assert_eq!(
            measured(Some((1600.0, 400.0)), Some(VISIBLE)),
            Measured::DoesNotFit
        );
        // The edges are inside: as wide or as tall as the screen fits.
        assert_eq!(
            measured(Some((1512.0, 865.0)), Some(VISIBLE)),
            Measured::Fits
        );
        assert_eq!(
            measured(Some((1512.5, 400.0)), Some(VISIBLE)),
            Measured::DoesNotFit
        );
    }

    /// A size that was not read, or is nonsense, is never a fit and never a
    /// refusal of the text: the alert is not shown and the parent hears none.
    #[test]
    fn a_size_that_was_not_read_is_unavailable() {
        for (alert, visible) in [
            (None, Some(VISIBLE)),
            (Some((520.0, 400.0)), None),
            (None, None),
            (Some((0.0, 400.0)), Some(VISIBLE)),
            (Some((520.0, 0.0)), Some(VISIBLE)),
            (Some((520.0, -1.0)), Some(VISIBLE)),
            (Some((520.0, f64::NAN)), Some(VISIBLE)),
            (Some((520.0, f64::INFINITY)), Some(VISIBLE)),
            (Some((520.0, 400.0)), Some((0.0, 865.0))),
            (Some((520.0, 400.0)), Some((1512.0, f64::NAN))),
            (Some((520.0, 400.0)), Some((f64::INFINITY, 865.0))),
            (Some((f64::INFINITY, 400.0)), Some(VISIBLE)),
        ] {
            assert_eq!(
                measured(alert, visible),
                Measured::Unavailable,
                "{alert:?} {visible:?}"
            );
        }
    }

    /// Only the second button approves; the first, the close box and anything
    /// else decline.
    #[test]
    fn only_the_second_button_approves() {
        assert!(approves(1001));
        for response in [1000, 1002, 0, -1, 1, isize::MAX, isize::MIN] {
            assert!(!approves(response), "{response}");
        }
    }

    /// The constant is AppKit's own.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_second_button_constant_is_appkits() {
        assert_eq!(SECOND_BUTTON, objc2_app_kit::NSAlertSecondButtonReturn);
    }
}
