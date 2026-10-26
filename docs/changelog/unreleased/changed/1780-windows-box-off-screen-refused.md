- On Windows, the approval box for a post, a review and a stop release is now measured when it opens. A box that is not
  wholly inside the screen it opened on is cancelled and never answered, whatever key was pressed, and the call is
  refused as `import_review_too_large` (the post) or `ack_review_too_large` (the review and the release), with
  nothing sent to Tally. Before, Windows left a box taller than the screen with its buttons below the edge, where
  a person could press `Y` on lines they could not see. Measured on one Windows PC (1920x1080 at 125 percent):
  boxes of 40, 44 and 48 lines fitted and one of 60 lines did not. A batch of about ten payments can therefore be
  refused on a smaller or more scaled screen, and has to be posted in parts. Other scales, a second monitor and
  1366x768 are not measured, and whether a cancelled box shows for an instant is not known. The macOS alert is not
  covered by this change (part of #1780, #1879).
