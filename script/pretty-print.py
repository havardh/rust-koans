import sys


failed = True
lines = sys.stdin.readlines()

for line in lines:
    if not (line.endswith("... ok\n") or line == "\n"):
        print line

    if line.startswith("test result: ok. "):
        failed = False


if failed:
    exit(1)
