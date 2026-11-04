# check-taskbar.ps1 (row 2161-C v3) - the known-answer check for requests/taskbar-uia.ps1, run on its own at a break
# between two rows, before v3 goes back into the queue: Tally open on the lab PC with no company loaded, and no box open.
# It runs only taskbar-uia.ps1, which is read-only (it changes no window, sends no key or click and sends nothing to
# Tally), reads back the file that script wrote, prints every line of it, then one K line:
#   K buttons=<n> b_lines=<n> in_task_list=<n> tally_in_task_list=<n> verdict=MET|NOT MET
# MET needs exactly one E line and one Z line, every B line in v2's form, one B line for each button the Z line counts,
# in_task_list above 0, and at least one B line in the task list (parent_class=MSTaskListWClass) whose name contains
# "Tally" in any case. A throw prints a K line with threw= and verdict=NOT MET. Exit 0 on MET, else 1.
# The lab also writes down the task buttons it sees on the taskbar; the K line does not judge that count.
# Written on a Mac and NOT RUN anywhere.
param([Parameter(Mandatory = $true)][string]$Out)
$read = Join-Path $PSScriptRoot 'requests\taskbar-uia.ps1'
try {
    & $read -Out $Out | Out-Null
} catch {
    "K threw=$($_.Exception.GetType().FullName): $($_.Exception.Message -replace '\r?\n', ' ') verdict=NOT MET"
    exit 1
}
$lines = [System.IO.File]::ReadAllLines($Out)
$lines
$e = 0; $z = 0; $b = 0; $bad = 0; $inList = 0; $tally = 0; $buttons = -1; $zInList = -1
foreach ($l in $lines) {
    if ($l -match '^E ') { $e++ }
    elseif ($l -match '^Z \S+ buttons=(\d+) in_task_list=(\d+)$') { $z++; $buttons = [int]$Matches[1]; $zInList = [int]$Matches[2] }
    elseif ($l -match '^B \S+ n=\d+ parent_class=(\S*) class=\S* offscreen=[01] rect=\S+ name=(.*)$') {
        $b++
        if ($Matches[1] -ceq 'MSTaskListWClass') {
            $inList++
            if ($Matches[2] -like '*tally*') { $tally++ }
        }
    }
    else { $bad++ }
}
$met = ($e -eq 1) -and ($z -eq 1) -and ($bad -eq 0) -and ($b -eq $buttons) -and ($inList -eq $zInList) -and
       ($inList -gt 0) -and ($tally -gt 0)
$verdict = if ($met) { 'MET' } else { 'NOT MET' }
"K buttons=$buttons b_lines=$b in_task_list=$inList tally_in_task_list=$tally verdict=$verdict"
if ($met) { exit 0 } else { exit 1 }
