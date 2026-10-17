#!/usr/bin/env bash
# Installs the debug APK on the running emulator, launches it and checks that
# the app process is still alive after a few seconds.
set -euo pipefail

app_id="${1:?usage: android-smoke.sh <application-id> [seconds]}"
seconds="${2:-15}"

apk=$(find target/dx -path '*android*' -name '*.apk' | head -n 1)
if [[ -z $apk ]]; then
  echo "No APK found under target/dx." >&2
  exit 1
fi

adb install -r "$apk"
adb logcat -c
adb shell monkey -p "$app_id" -c android.intent.category.LAUNCHER 1
sleep "$seconds"

if adb shell pidof "$app_id" >/dev/null; then
  echo "Android app ran for ${seconds}s without crashing."
  exit 0
fi
echo "Android app is not running after ${seconds}s. Crash log:" >&2
adb logcat -d -b crash >&2 || true
exit 1
