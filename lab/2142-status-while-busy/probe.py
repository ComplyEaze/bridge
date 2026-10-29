# Lab helper for the #2142 row (not product code): send the one read, or none, and time GET /status
# beside it, one probe at a time. No shell: every curl is a list of arguments.
#
# Usage, run from the sheet's folder:
#   python -I probe.py --block NAME --out FOLDER [--read FILE --read-sha256 HEX] [--cut-after 20]
#   --block        the step's name (A1, B1 ...), used in file names
#   --out          a folder that exists; the log, headers, answer and probe bodies are written there
#   --read         the request file to send by C3's line; left out, only probes are sent (a block at rest)
#   --read-sha256  the request file's sha256; another sha256 refuses the block before anything is sent
#   --cut-after    give the read curl's --max-time in seconds (a block that cuts the read); left out, none
#
# The read is reaped by a thread the moment it ends, so `read_ended` is logged when the read ends,
# not when a probe in flight returns. `read_end_t` is `read_sent`'s t plus the read's own time_total,
# the end as curl measured it.
#
# Probes: GET /status by the sheet's line, no --max-time. The first starts 2 s after the read starts
# (or at once at rest); each next one starts 2 s after the previous ended, so at most one probe is in
# flight. A probe counts as answered only with exit 0, status 200 and 51 bytes. Probing ends when the
# read has ended and 3 probes in a row sent after its end answered in under 1.000 s; at rest, after
# 10 probes.
#
# Exit 0: the block ended as designed. Exit 2: a STOP, whose reason is the log's last line. A STOP is:
# a whole read that did not answer 200 with bytes; a cut read that ended with neither exit 28 (cut) nor
# exit 0 (answered); a probe that did not answer as above; a probe without an answer in 600 s (that
# curl is left running); the operator's stop file `STOP-<block>` in the out folder (checked before each
# probe); 200 probes; 900 s in all, checked before each probe. On a STOP no more probes are sent, the
# read curl is never killed, and if it is still running the helper waits up to 600 s for it to end on
# its own so its result is logged. A cut read that answered whole is logged as a result
# (`read_answered_before_cut`), not a STOP. Any other exit code (a Python error exits 1) is a STOP too.
import argparse
import datetime
import hashlib
import json
import os
import subprocess
import sys
import threading
import time

PORT = 'http://127.0.0.1:9001'
STATUS_BYTES = '51'
QUICK_SECONDS = 1.0
QUICK_IN_A_ROW = 3
GAP_SECONDS = 2.0
REST_PROBES = 10
MAX_PROBES = 200
MAX_BLOCK_SECONDS = 900
PROBE_WAIT_SECONDS = 600
CURL_TIMEOUT_EXIT = 28


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='milliseconds')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--block', required=True)
    parser.add_argument('--out', required=True)
    parser.add_argument('--read')
    parser.add_argument('--read-sha256')
    parser.add_argument('--cut-after', type=int)
    args = parser.parse_args()
    if not os.path.isdir(args.out):
        sys.exit('probe: --out must be a folder that exists')
    if bool(args.read) != bool(args.read_sha256):
        sys.exit('probe: --read and --read-sha256 go together')
    if args.cut_after is not None and not args.read:
        sys.exit('probe: --cut-after needs --read')
    if args.read:
        with open(args.read, 'rb') as request:
            found = hashlib.sha256(request.read()).hexdigest()
        if found != args.read_sha256.lower():
            sys.exit(f'probe: {args.read} has sha256 {found}, not the one given; nothing sent')
    log = open(os.path.join(args.out, f'{args.block}-log.jsonl'), 'x', encoding='utf-8', newline='\n')
    lock = threading.Lock()
    start = time.monotonic()

    def record(event, **fields):
        with lock:
            line = {'utc': utc(), 't': round(time.monotonic() - start, 3), 'event': event, **fields}
            log.write(json.dumps(line) + '\n')
            log.flush()
            print(json.dumps(line), flush=True)

    read = None
    read_state = {'ended': False, 'failed': False}
    read_ended = threading.Event()

    def reap(process, sent_t):
        try:
            out, err = process.communicate()
            fields = out.strip().split()
            total = float(fields[2]) if len(fields) > 2 else None
            record('read_ended', exit=process.returncode, printed=out.strip(), stderr=err.strip(),
                   read_end_t=round(sent_t + total, 3) if total is not None else None)
            if args.cut_after is None:
                whole = (process.returncode == 0 and len(fields) > 1 and fields[0] == '200'
                         and fields[1] != '0')
                read_state['failed'] = not whole
            elif process.returncode == 0:
                record('read_answered_before_cut', exit=0)
            elif process.returncode != CURL_TIMEOUT_EXIT:
                read_state['failed'] = True
        except Exception as error:  # the helper must not go on probing blind
            read_state['failed'] = True
            record('reaper_error', error=repr(error))
        finally:
            read_state['ended'] = True
            read_ended.set()

    def stop(reason, **fields):
        running = read is not None and not read_ended.is_set()
        record('stop', reason=reason, read_still_running=running,
               read_pid=read.pid if read is not None else None, **fields)
        if running:
            # Never killed: wait for it to end on its own, so its result is logged.
            read_ended.wait(PROBE_WAIT_SECONDS)
            record('stop_wait_ended', read_ended=read_ended.is_set())
        return 2

    if args.read:
        command = ['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}',
                   '--data-binary', '@' + args.read,
                   '-H', 'Content-Type: text/xml; charset=utf-16',
                   '-D', os.path.join(args.out, f'{args.block}-read-headers.txt'),
                   PORT, '-o', os.path.join(args.out, f'{args.block}-read-answer.xml')]
        if args.cut_after is not None:
            command[1:1] = ['--max-time', str(args.cut_after)]
        sent_t = round(time.monotonic() - start, 3)
        read = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        record('read_sent', command=command, pid=read.pid, read_sent_t=sent_t)
        threading.Thread(target=reap, args=(read, sent_t), daemon=True).start()
        read_ended.wait(GAP_SECONDS)
    quick_after_read = 0
    probes = 0
    while True:
        if read_state['failed']:
            return stop('read_failed')
        if read is None and probes >= REST_PROBES:
            record('end', reason='rest_probes_done')
            return 0
        if read_state['ended'] and quick_after_read >= QUICK_IN_A_ROW:
            record('end', reason='quick_in_a_row_after_read')
            return 0
        if os.path.exists(os.path.join(args.out, f'STOP-{args.block}')):
            return stop('operator_stop')
        if probes >= MAX_PROBES or time.monotonic() - start > MAX_BLOCK_SECONDS:
            return stop('probe_or_time_cap')
        after_read = read_state['ended'] if read is not None else None
        probes += 1
        body = os.path.join(args.out, f'{args.block}-probe-{probes:03d}.body')
        command = ['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}',
                   '-o', body, PORT + '/status']
        record('probe_sent', n=probes, after_read=after_read)
        probe = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            out, err = probe.communicate(timeout=PROBE_WAIT_SECONDS)
        except subprocess.TimeoutExpired:
            # C3: a request without an answer is left running and never sent again.
            return stop('probe_unanswered', n=probes, probe_pid=probe.pid)
        printed = out.strip()
        fields = printed.split()
        record('probe_ended', n=probes, exit=probe.returncode, printed=printed,
               stderr=err.strip(), after_read=after_read)
        if (probe.returncode != 0 or len(fields) < 3 or fields[0] != '200'
                or fields[1] != STATUS_BYTES):
            return stop('probe_failed', n=probes)
        if after_read:
            quick_after_read = quick_after_read + 1 if float(fields[2]) < QUICK_SECONDS else 0
        time.sleep(GAP_SECONDS)


if __name__ == '__main__':
    sys.exit(main())
