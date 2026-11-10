#!/usr/bin/env python3
"""Run every test of one unittest file, split across N processes that run side by side.

    python3 scripts/shard-unittest.py TEST_FILE COUNT

CI runs the merge gate's test this way (several minutes in one process). The parent lists the file's tests and
hands process i the tests at positions i, i+COUNT, i+2*COUNT, ... of that list (a class's tests are spread over
the processes, in the file's order inside each, so a class's fixtures are built once per process that holds any
of its tests). A test file is loaded by path, because its name may hold a hyphen.

The exit status is 0 only when, for every process: it exited 0, it wrote its result file, the test ids it reports are
exactly the ids the parent assigned to it, it ran as many tests as that, none was skipped, and it was assigned at least one. A process that crashed before
it ran anything, a test that leaves the process, a skipped test or a mismatch in which test ran is a failure, and
the failing process's output is printed. Test output cannot alter a result: each process reports in a file.
"""
from __future__ import annotations

import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


def load(path: Path):
    spec = importlib.util.spec_from_file_location("sharded_test_module", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def flatten(suite):
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            yield from flatten(item)
        else:
            yield item


def all_tests(module) -> list:
    # loadTestsFromModule orders classes and tests the way a plain run of the file does; a test is keyed by its id.
    tests = list(flatten(unittest.defaultTestLoader.loadTestsFromModule(module)))
    ids = [test.id() for test in tests]
    if len(set(ids)) != len(ids):
        raise SystemExit("shard-unittest: duplicate test ids, so a test could run twice or not at all")
    return tests


def run_shard(path: Path, count: int, index: int, result_file: Path) -> int:
    chosen = all_tests(load(path))[index::count]
    if not chosen:
        print(f"shard {index} of {count} holds no test", file=sys.stderr)
        return 1
    outcome = unittest.TextTestRunner(verbosity=1).run(unittest.TestSuite(chosen))
    ran = sorted(test.id() for test in chosen)
    result_file.write_text(json.dumps({"ids": ran, "ran": outcome.testsRun, "skipped": len(outcome.skipped)}), encoding="utf-8")
    return 0 if outcome.wasSuccessful() else 1


def check_shard(index: int, expected_ids: list, code: int, result_file: Path) -> str:
    """Why this process does not count as a pass, or '' when it does."""
    if code != 0:
        return f"exit {code}"
    try:
        result = json.loads(result_file.read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return "no readable result file"
    if not isinstance(result, dict):
        return "its result file is not a record"
    if result.get("ids") != sorted(expected_ids):
        return "the tests it ran are not the tests assigned to it"
    if result.get("ran") != len(expected_ids) or not expected_ids:
        return f"ran {result.get('ran')} of {len(expected_ids)} assigned tests"
    if result.get("skipped"):
        return f"{result['skipped']} test(s) skipped; a skipped test is not a test that ran"
    return ""


def main(argv: list) -> int:
    sys.stdout.reconfigure(errors="replace")
    if len(argv) == 5 and argv[0] == "--shard":
        return run_shard(Path(argv[3]), int(argv[2]), int(argv[1]), Path(argv[4]))
    if len(argv) != 2 or not argv[1].isdigit() or int(argv[1]) < 1:
        print(__doc__, file=sys.stderr)
        return 2
    path, count = Path(argv[0]).resolve(), int(argv[1])
    ids = [test.id() for test in all_tests(load(path))]
    if len(ids) < count:
        print(f"shard-unittest: {len(ids)} tests cannot fill {count} shards", file=sys.stderr)
        return 1
    failed = []
    with tempfile.TemporaryDirectory(prefix="shard-unittest-") as folder:
        runs = []
        for index in range(count):
            log = open(Path(folder) / f"shard-{index}.log", "w+", encoding="utf-8", errors="replace")
            result_file = Path(folder) / f"shard-{index}.json"
            process = subprocess.Popen([sys.executable, __file__, "--shard", str(index), str(count), str(path), str(result_file)],
                                       stdout=log, stderr=subprocess.STDOUT)
            runs.append((index, process, log, result_file))
        for index, process, log, result_file in runs:
            code = process.wait()
            log.seek(0)
            text = log.read()
            log.close()
            reason = check_shard(index, ids[index::count], code, result_file)
            if reason:
                failed.append(index + 1)
                print(f"::error::shard {index + 1} of {count} failed: {reason}")
                print(text)
            else:
                print(f"::group::shard {index + 1} of {count} passed")
                print(text)
                print("::endgroup::")
    if failed:
        print(f"shard-unittest: shard(s) {failed} failed", file=sys.stderr)
        return 1
    print(f"shard-unittest: all {len(ids)} tests ran once, in {count} shards")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
