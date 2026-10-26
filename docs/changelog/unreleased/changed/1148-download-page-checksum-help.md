- The Download page, the install guide and the top of each release now help a person check a download. The Download
  page gives the two commands that print a file's SHA-256 code (`certutil -hashfile "<file>" SHA256` on Windows,
  `shasum -a 256 "<file>"` on a Mac) with the file name in double quotes, says to run them in the Downloads folder,
  shows beside each download the code as GitHub reports it, and says where a newer version appears. A GitHub release
  now opens with a short header (which file to take, the install guide, the checksum step) before the change list.
  What an older release needs moved to the last section of the Download page, below the steps for a new install, and
  the README gained an "At a glance" block with what can be asked and what cannot yet (part of #1148, #1046, #1785,
  #1786, #1788, #1790, #1837, #1850, #1882).
