// Prints the height AppKit lays out a request's text at, in the alert's column
// (13-point system font, 406 points wide), as the dialog child measures it.
// Reads only: no window. Usage: swift measure.swift <request file>
import AppKit
let args = CommandLine.arguments
guard args.count == 2, let body = try? String(contentsOfFile: args[1], encoding: .utf8) else {
    print("error: usage: swift measure.swift <request file>"); exit(3)
}
let parts = body.split(separator: "\n", maxSplits: 2, omittingEmptySubsequences: false)
guard parts.count == 3 else { print("error: not a request"); exit(3) }
var text = String(parts[2]); if text.hasSuffix("\n") { text.removeLast() }
let rect = (text as NSString).boundingRect(
    with: NSSize(width: 406, height: 1.0e7),
    options: [.usesLineFragmentOrigin, .usesFontLeading],
    attributes: [.font: NSFont.systemFont(ofSize: 13)])
print(String(format: "measured_height=%.0f lines_at_16=%.2f", ceil(rect.height), ceil(rect.height) / 16))
