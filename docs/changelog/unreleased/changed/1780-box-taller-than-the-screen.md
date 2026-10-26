- On Windows the approval and review boxes cannot scroll, and a box taller than the screen used to push its buttons
  off it. The box is now measured when it opens, against the working area of the monitor nearest it, and one that is
  not wholly on the screen is cancelled and refused as `import_review_too_large` (a post) or `ack_review_too_large`
  (a review, or the release of an invoice stop), with nothing posted or recorded; before this, a person could answer
  with `Y` a box whose lower lines and buttons they could not see (part of #1780, #1832). A long batch or a small or
  highly scaled screen can now be refused at the box where it was shown before; a larger or less scaled screen, or a
  shorter narration, helps, and the `post_import` and `acknowledge_post_review` texts say so. Not covered: macOS,
  whose alert cannot be measured, a screen that changes while the box is open, and a second monitor with another
  scale. Run on one real Windows screen (1920x1080 at 125 percent, one monitor): boxes of 40 and 44 lines were
  shown and answered normally, and boxes of 48, 60, 80 and 120 lines were cancelled before they were seen and refused;
  that screen holds about 44 lines (the 48-line box is 1087 pixels high against a working area of 1030). Other
  screens, other scales and a second monitor have not been measured.
