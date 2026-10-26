- A snapshot run whose commit the local mirror refuses for good, on an invariant of the batch, now says "Commit refused: a
  resume cannot succeed. Start a new read." and offers no resume button, and the recent-runs table lists it as "Commit
  refused" instead of "Inspect only"; before this the screen showed "Snapshot mirror failed" beside a resume that was
  refused again each time. After the app restarts, such a run reads back as resumable until that resume is tried
  (fixes #1677, #1900).
