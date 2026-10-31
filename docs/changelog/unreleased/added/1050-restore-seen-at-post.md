- `post_import` now sees a restore before it sends (part of #1050). When the company's voucher mark its last read before
  the send gives is below what an earlier native post of the company had proved, the book was rolled back after that
  post, and the restore is recorded before this post is sent, as `verify_import` already records one it sees: the
  earlier posts it covers no longer read `posted_verified`, and the answer names them (`rollback_recorded_at_post`) to
  verify again. Before, a restore followed by new posts went unseen once those posts raised the mark again. The record
  names one of the posts it covers, so the journal does not say which post saw the restore. A record that cannot be
  written refuses the post as `post_rollback_unrecorded`, with nothing sent. No new kind of journal record is written:
  a build that reads the records `verify_import` writes since this restore check began reads these too.
