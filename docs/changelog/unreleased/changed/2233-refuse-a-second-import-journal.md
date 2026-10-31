- On Windows, `build_import_xml` and `post_import` now refuse with `import_journal_split` when a Microsoft Store app's
  copy of the data folder (`%LOCALAPPDATA%\Packages\<package>\LocalCache\Local\Bridge\agent`) holds an import journal,
  other than the one this process uses, with at least one batch. On a Store install of Claude Desktop the extension
  inside it and a ComplyEaze Bridge started outside it each keep their own journal, and neither journal's checks see
  the other's batches, so a row could be posted from both. The refusal writes and sends nothing, and moves, merges or
  deletes neither journal; its `cause` says what was found, and a copy that cannot be read or listed is refused the same
  way. From inside Claude Desktop a journal in `%LOCALAPPDATA%\Bridge\agent` cannot be seen past the package's own copy,
  so there only another package's copy is found (#2233).
