// SPDX-License-Identifier: Apache-2.0
//
// Checks that a Windows executable does not import the Microsoft C runtime (#2390): none of the
// Visual C++ runtime, the C++ standard library, the concurrency, OpenMP, C++/CLI, AMP, MFC and
// ATL runtimes, the old MSVCR family, the Universal CRT forwarders or ucrtbase. A person whose PC lacks the Visual
// C++ Redistributable cannot start an executable that imports them.
//
//   node scripts/check-static-runtime.mjs <file.exe>
//       Exit 0 and a counted list when no such import exists; exit 1 when one does; exit 2
//       when the file could not be read as a 64-bit PE, so "nothing found" is never reported
//       for a file that was not understood (AGENTS.md, P5).
//
// The import directory and the delay-load directory are read as structures, never searched as
// text. A file with no kernel32.dll import is refused as unreadable, because every Windows
// executable of ours imports it and an empty list would otherwise pass.
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";

// Matched on the lowercased DLL name.
export const FORBIDDEN_IMPORTS = [
  /^vcruntime/,
  /^msvcp/,
  /^msvcr\d/,
  /^concrt/,
  /^vcomp/,
  /^vccorlib/,
  /^vcamp/,
  /^mfc\d/,
  /^atl\d/,
  /^api-ms-win-crt-/,
  /^ucrtbase/,
];

export class PeReadError extends Error {}

const IMAGE_DIRECTORY_IMPORT = 1;
const IMAGE_DIRECTORY_DELAY_IMPORT = 13;
const PE32_PLUS_MAGIC = 0x20b;

function need(condition, message) {
  if (!condition) throw new PeReadError(message);
}

// Returns { imports, delayImports }: DLL names exactly as stored, in file order.
export function readPeImports(bytes) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  need(bytes.length >= 0x40 && view.getUint16(0, true) === 0x5a4d, "not a PE file: no MZ header");
  const peOffset = view.getUint32(0x3c, true);
  need(peOffset + 24 <= bytes.length && view.getUint32(peOffset, true) === 0x00004550, "not a PE file: no PE signature");
  const sectionCount = view.getUint16(peOffset + 6, true);
  const optionalSize = view.getUint16(peOffset + 20, true);
  const optional = peOffset + 24;
  need(optionalSize >= 112 && optional + optionalSize <= bytes.length, "truncated or missing optional header");
  need(view.getUint16(optional, true) === PE32_PLUS_MAGIC, "not a 64-bit PE file");
  const directoryCount = view.getUint32(optional + 108, true);
  const directories = optional + 112;
  need(directories + directoryCount * 8 <= optional + optionalSize, "truncated data directories");

  const sectionTable = optional + optionalSize;
  need(sectionTable + sectionCount * 40 <= bytes.length, "truncated section table");
  const sections = [];
  for (let index = 0; index < sectionCount; index += 1) {
    const at = sectionTable + index * 40;
    sections.push({
      virtualAddress: view.getUint32(at + 12, true),
      rawSize: view.getUint32(at + 16, true),
      rawOffset: view.getUint32(at + 20, true),
    });
  }
  function offsetOf(rva) {
    for (const section of sections) {
      // Only bytes the file holds: a name in the part of a section that exists in memory alone has
      // no bytes to read, and reading the next section's bytes instead would give a wrong name.
      if (rva >= section.virtualAddress && rva < section.virtualAddress + section.rawSize) {
        const offset = section.rawOffset + (rva - section.virtualAddress);
        need(offset < bytes.length, `RVA ${rva} points past the end of the file`);
        return offset;
      }
    }
    throw new PeReadError(`RVA ${rva} is in no section`);
  }
  function nameAt(rva) {
    let at = offsetOf(rva);
    let name = "";
    while (true) {
      need(at < bytes.length, "unterminated DLL name");
      const byte = bytes[at];
      if (byte === 0) break;
      need(name.length < 260, "DLL name longer than 260 bytes");
      name += String.fromCharCode(byte);
      at += 1;
    }
    need(name.length > 0, "empty DLL name");
    return name;
  }
  function directory(index) {
    if (index >= directoryCount) return null;
    const rva = view.getUint32(directories + index * 8, true);
    const size = view.getUint32(directories + index * 8 + 4, true);
    return rva === 0 && size === 0 ? null : { rva, size };
  }

  const imports = [];
  const importDirectory = directory(IMAGE_DIRECTORY_IMPORT);
  if (importDirectory) {
    let at = offsetOf(importDirectory.rva);
    while (true) {
      need(at + 20 <= bytes.length, "import directory is not terminated");
      const nameRva = view.getUint32(at + 12, true);
      const thunkRva = view.getUint32(at + 16, true);
      const originalRva = view.getUint32(at, true);
      if (nameRva === 0 && thunkRva === 0 && originalRva === 0) break;
      imports.push(nameAt(nameRva));
      at += 20;
      need(imports.length < 4096, "import directory has more than 4096 entries");
    }
  }
  const delayImports = [];
  const delayDirectory = directory(IMAGE_DIRECTORY_DELAY_IMPORT);
  if (delayDirectory) {
    let at = offsetOf(delayDirectory.rva);
    while (true) {
      need(at + 32 <= bytes.length, "delay-load directory is not terminated");
      const nameRva = view.getUint32(at + 4, true);
      if (nameRva === 0) break;
      delayImports.push(nameAt(nameRva));
      at += 32;
      need(delayImports.length < 4096, "delay-load directory has more than 4096 entries");
    }
  }
  return { imports, delayImports };
}

// Returns { names, forbidden }. Throws PeReadError when the file is not understood.
export function checkStaticRuntime(bytes) {
  let parsed;
  try {
    parsed = readPeImports(bytes);
  } catch (error) {
    // A read past the end of a malformed file is "not understood", never "found".
    if (error instanceof RangeError) throw new PeReadError(`malformed file: ${error.message}`);
    throw error;
  }
  const { imports, delayImports } = parsed;
  const names = [...imports, ...delayImports.map((name) => `delay-load:${name}`)];
  need(
    imports.some((name) => name.toLowerCase() === "kernel32.dll"),
    "the import table has no kernel32.dll, so it was not read correctly",
  );
  const forbidden = [...imports, ...delayImports].filter((name) =>
    FORBIDDEN_IMPORTS.some((pattern) => pattern.test(name.toLowerCase())),
  );
  return { names, forbidden };
}

export function main(args, log = console.log) {
  if (args.length !== 1) {
    log("usage: node scripts/check-static-runtime.mjs <file.exe>");
    return 2;
  }
  let bytes;
  try {
    bytes = readFileSync(args[0]);
  } catch (error) {
    log(`static runtime check: cannot read ${args[0]}: ${error.code ?? error.message}`);
    return 2;
  }
  let result;
  try {
    result = checkStaticRuntime(new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength));
  } catch (error) {
    if (!(error instanceof PeReadError)) throw error;
    log(`static runtime check: ${args[0]} was not understood: ${error.message}`);
    return 2;
  }
  log(`static runtime check: ${args[0]}: ${bytes.length} bytes, ${result.names.length} imported DLLs`);
  for (const name of result.names) log(`  ${name}`);
  if (result.forbidden.length > 0) {
    log(`static runtime check FAILED: ${result.forbidden.length} runtime import(s): ${result.forbidden.join(", ")}`);
    return 1;
  }
  log("static runtime check ok: 0 runtime imports");
  return 0;
}

if (process.argv[1] && existsSync(process.argv[1]) && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  process.exit(main(process.argv.slice(2)));
}
