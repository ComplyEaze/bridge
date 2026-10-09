// A Node test reporter for the Windows run of the Node suite (.github/workflows/node-windows.yml).
// It writes one JSON line per finished test: the test file, the names from the file down to the
// test, and pass, fail or skip. Suites are dropped, and so is a test that failed only because a
// subtest failed, so each failure is counted once, at the test that threw.
// Use: node --test --test-reporter=./scripts/node-test-outcomes.mjs --test-reporter-destination=FILE ...
// scripts/<name> whatever the host's separator or drive prefix is.
export function repoFile(file) {
  return `scripts/${String(file).split(/[\\/]/).pop()}`;
}

export default async function* nodeTestOutcomes(source) {
  const stacks = new Map();
  for await (const event of source) {
    const data = event.data;
    if (event.type === "test:start") {
      const stack = stacks.get(data.file) ?? [];
      stack[data.nesting] = data.name;
      stack.length = data.nesting + 1;
      stacks.set(data.file, stack);
    } else if (event.type === "test:pass" || event.type === "test:fail") {
      if (data.details?.type === "suite") continue;
      if (event.type === "test:fail" && data.details?.error?.failureType === "subtestsFailed") continue;
      const names = [...(stacks.get(data.file) ?? [])].slice(0, data.nesting);
      names.push(data.name);
      const outcome = event.type === "test:fail" ? "fail" : data.skip || data.todo ? "skip" : "pass";
      const line = { file: repoFile(data.file), names, outcome };
      if (outcome === "fail") line.error = String(data.details?.error?.message ?? "").split("\n")[0].slice(0, 200);
      yield `${JSON.stringify(line)}\n`;
    }
  }
}
