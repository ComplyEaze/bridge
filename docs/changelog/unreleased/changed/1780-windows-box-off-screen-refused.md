- On Windows, the approval box for a post, a review and a stop release is now measured when it opens. A box that is not
  wholly inside the screen it opened on is cancelled and never answered, whatever key was pressed, and the call is
  refused as `import_review_too_large` (the post) or `ack_review_too_large` (the review and the release), with
  nothing sent to Tally. Before, Windows left a box taller than the screen with its buttons below the edge, where
  a person could press `Y` on lines they could not see. A Windows that cannot be made per-monitor DPI aware shows no
  box at all, and the call is refused as `import_approval_unavailable` (`ack_review_unavailable` for the review and
  the release). Measured on one Windows PC with a work area 1,030 pixels high (1920x1080 at 125 percent), in the
  program as shipped: boxes of 40 and 44 lines (935 and 1,011 pixels high) fitted, and boxes of 48 and 60 lines
  (1,087 and 1,315) did not and were cancelled. Scales of 100, 150 and 175 percent, 1366x768, a second monitor and
  the Windows text-size setting are not measured, and whether a cancelled box shows for an instant is not known. The
  macOS alert is not covered by this change (part of #1780, #1879).
