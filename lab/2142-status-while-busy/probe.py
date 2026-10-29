# Lab helper for the #2142 row (not product code): send the one read, or none, and time GET /status
# beside it, one probe at a time. No shell: every curl is a list of arguments.
#
# Usage: python -I probe.py --block NAME --out FOLDER [--read FILE] [--cut-after 20]
#   --block      the step's name (A1, B1 ...), used in file names
#   --out        a folder that exists; the log, headers and answer are written there
#   --read       the request file to send by C3's line; left out, only probes are sent (a block at rest)
#   --cut-after  give the read curl's --max-time in seconds (a block that cuts the read); left out, none
#
# Probes: GET /status by C2 c's line with time_total added to -w, no --max-time. The first starts
# 2 s after the read starts (or at once at rest); each next one starts 2 s after the previous ended,
# so at most one probe is in flight. Probing ends when the read curl has exited and 3 probes in a row
# sent after it exited answered 200 in under 1.000 s. At rest it ends after 10 probes.
# The block STOPS (exit 2, logged) on: a probe exit that is not 0 or a status that is not 200; a
# probe without an answer in 600 s (that curl is left running); 200 probes; 900 s in all.
# The log is JSON lines: one per event, with the UTC time, the seconds since the block started,
# and curl's exit code and -w fields as printed.
import argparse
import datetime
import json
import os
import subprocess
import sys
import time

PORT = 'http://127.0.0.1:9001'
QUICK_SECONDS = 1.0
QUICK_IN_A_ROW = 3
GAP_SECONDS = 2.0
REST_PROBES = 10
MAX_PROBES = 200
MAX_BLOCK_SECONDS = 900
PROBE_WAIT_SECONDS = 600


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='milliseconds')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--block', required=True)
    parser.add_argument('--out', required=True)
    parser.add_argument('--read')
    parser.add_argument('--cut-after', type=int)
    args = parser.parse_args()
    if not os.path.isdir(args.out):
        sys.exit('probe: --out must be a folder that exists')
    if args.cut_after is not None and not args.read:
        sys.exit('probe: --cut-after needs --read')
    log_path = os.path.join(args.out, f'{args.block}-log.jsonl')
    if os.path.exists(log_path):
        sys.exit(f'probe: {log_path} exists; a block is never run twice into one log')
    log = open(log_path, 'w', encoding='utf-8', newline='\n')
    start = time.monotonic()

    def record(event, **fields):
        line = {'utc': utc(), 't': round(time.monotonic() - start, 3), 'event': event, **fields}
        log.write(json.dumps(line) + '\n')
        log.flush()
        print(json.dumps(line))

    read = None
    if args.read:
        command = ['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}',
                   '--data-binary', '@' + args.read,
                   '-H', 'Content-Type: text/xml; charset=utf-16',
                   '-D', os.path.join(args.out, f'{args.block}-read-headers.txt'),
                   PORT, '-o', os.path.join(args.out, f'{args.block}-read-answer.xml')]
        if args.cut_after is not None:
            command[1:1] = ['--max-time', str(args.cut_after)]
        record('read_sent', command=command)
        read = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        time.sleep(GAP_SECONDS)
    read_done = read is None
    quick_after_read = 0
    probes = 0
    while True:
        if read is not None and not read_done and read.poll() is not None:
            out, err = read.communicate()
            record('read_ended', exit=read.returncode, printed=out.strip(), stderr=err.strip())
            read_done = True
        if read is None and probes >= REST_PROBES:
            record('end', reason='rest_probes_done')
            return 0
        if read_done and read is not None and quick_after_read >= QUICK_IN_A_ROW:
            record('end', reason='quick_in_a_row_after_read')
            return 0
        if probes >= MAX_PROBES or time.monotonic() - start > MAX_BLOCK_SECONDS:
            record('stop', reason='probe_or_time_cap')
            return 2
        sent_after_read = read_done
        probes += 1
        status_file = os.path.join(args.out, f'{args.block}-probe-{probes:03d}.body')
        command = ['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}',
                   '-o', status_file, PORT + '/status']
        record('probe_sent', n=probes, after_read=sent_after_read)
        probe = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            out, err = probe.communicate(timeout=PROBE_WAIT_SECONDS)
        except subprocess.TimeoutExpired:
            # C3: a request without an answer is left running and never sent again. The curl
            # is not killed; the block stops and sends nothing more.
            record('stop', reason='probe_unanswered', n=probes, left_running_pid=probe.pid)
            return 2
        printed = out.strip()
        fields = printed.split()
        record('probe_ended', n=probes, exit=probe.returncode, printed=printed,
               stderr=err.strip(), after_read=sent_after_read)
        if probe.returncode != 0 or not fields or fields[0] != '200':
            record('stop', reason='probe_failed', n=probes)
            return 2
        seconds = float(fields[2]) if len(fields) > 2 else float('inf')
        if sent_after_read:
            quick_after_read = quick_after_read + 1 if seconds < QUICK_SECONDS else 0
        time.sleep(GAP_SECONDS)


if __name__ == '__main__':
    sys.exit(main())
