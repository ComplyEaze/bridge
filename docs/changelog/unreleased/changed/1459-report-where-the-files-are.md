- `bridge_mcp --local-data-report --show-paths` also prints, beside each folder, the folders its files are in as Windows
  reports each file, and the import journal's own location, so on a Microsoft Store install of Claude Desktop it shows
  where the files really are, including when they are split between Claude Desktop's own copy of the folder and
  `%LOCALAPPDATA%\Bridge\agent` (#1459).
