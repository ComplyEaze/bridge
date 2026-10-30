// Prints the frame of every on-screen window owned by UserNotificationCenter,
// the process that draws rfd's parentless macOS alert, and every screen's
// frame and visible frame, all in points with the origin at the top left of
// the main screen (Core Graphics' convention). Reads only: no window, no key.
// Exit 0: one or more panels. Exit 2: the read worked and found no panel.
// Exit 3: the window list could not be read.
import AppKit
import CoreGraphics

guard let list = CGWindowListCopyWindowInfo([.optionOnScreenOnly], kCGNullWindowID) as? [[String: Any]] else {
    print("error: window list unreadable")
    exit(3)
}
let mainHeight = NSScreen.screens.first?.frame.height ?? 0
for screen in NSScreen.screens {
    let f = screen.frame, v = screen.visibleFrame
    print(String(format: "screen frame=%.0f,%.0f,%.0f,%.0f visible_top_left=%.0f,%.0f,%.0f,%.0f scale=%.1f",
                 f.minX, mainHeight - f.maxY, f.width, f.height,
                 v.minX, mainHeight - v.maxY, v.width, v.height, screen.backingScaleFactor))
}
var panels = 0
for window in list where (window[kCGWindowOwnerName as String] as? String) == "UserNotificationCenter" {
    let b = window[kCGWindowBounds as String] as? [String: Double] ?? [:]
    let layer = window[kCGWindowLayer as String] as? Int ?? -1
    print(String(format: "panel x=%.0f y=%.0f w=%.0f h=%.0f layer=%d",
                 b["X"] ?? -1, b["Y"] ?? -1, b["Width"] ?? -1, b["Height"] ?? -1, layer))
    panels += 1
}
print("panels=\(panels)")
exit(panels > 0 ? 0 : 2)
