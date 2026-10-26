- When the local mirror refuses to commit a snapshot run because of something in the run's own saved batch, the Proof of
  Sync screen now says so and offers no resume. Before, it showed the same "mirror failed" title as a locked or full
  database and offered a resume, which could never succeed, because the same commit is refused every time. The run
  stays listed and inspectable: its status line reads "Commit refused: a resume cannot succeed. Start a new read.", the
  recent-runs table reads "Commit refused", and nothing is lost from the last verified baseline. Other mirror
  failures keep their title and their resume. After the app is restarted, a refused run shows again as one that can be
  resumed until the resume is tried, and is then refused again with the new text. Only unit and component tests ran; no
  run was refused this way by a real mirror, since no release writes such a batch (fixes #1677).
