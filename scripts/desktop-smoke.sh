#!/usr/bin/env bash
# Starts the desktop app under a virtual display and checks that it is still
# running after a few seconds, i.e. it did not crash on start-up.
set -euo pipefail

binary="${1:?usage: desktop-smoke.sh <path-to-desktop-binary> [seconds]}"
seconds="${2:-10}"

set +e
timeout --preserve-status --signal=TERM "${seconds}" xvfb-run --auto-servernum "${binary}"
status=$?
set -e

# 143 = terminated by our SIGTERM, so the app was still running.
if [[ ${status} -eq 143 ]]; then
  echo "Desktop app ran for ${seconds}s without crashing."
  exit 0
fi
echo "Desktop app exited early with status ${status}." >&2
exit 1
