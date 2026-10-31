- `bridge_mcp --local-data-report --show-paths` also prints, beside each folder, where its files are as Windows reports
  each file (the files directly in the folder, the first 64 by name, the listing capped at 100,000 entries with
  `listing_capped`), the import journal's own location and, on Windows, each copy of the data folder inside a Store app's
  package that exists. On a Microsoft Store install of Claude Desktop it shows where the files directly in each folder are
  (not those in its subfolders, such as `imports`) and, run outside Claude Desktop, that a second copy of the folder
  exists. A journal file or a lease folder that is itself a link is not followed (`link_not_followed`) (#1459).
