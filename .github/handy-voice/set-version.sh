#!/usr/bin/env bash
# Handy Voice: sets the version of a fork build. The handy-voice workflow runs
# it in the commit it builds, which only the tag voice-v<version> points to;
# the branch itself keeps Handy's own version, so rebases never meet it.
#
# Usage: bash .github/handy-voice/set-version.sh <Handy tag vX.Y.Z> <build N>
# Prints the version X.Y.(Z*100+N): the updater only offers a strictly higher
# version and Windows installers need numeric parts, so Handy 0.10.0 build 2
# is 0.10.2 and Handy 0.10.1 build 0 is 0.10.100.
set -euo pipefail

base=${1:?Handy tag, e.g. v0.10.0}
build=${2:?fork build number}

if ! [[ $base =~ ^v([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
  echo "set-version: Handy tag '$base' is not vX.Y.Z" >&2
  exit 1
fi
major=$((10#${BASH_REMATCH[1]}))
minor=$((10#${BASH_REMATCH[2]}))
patch=$((10#${BASH_REMATCH[3]}))
if ! [[ $build =~ ^[0-9]+$ ]] || ((10#$build > 99)); then
  echo "set-version: build number '$build' is not 0-99" >&2
  exit 1
fi
fork_patch=$((patch * 100 + 10#$build))
# MSI versions allow at most 255.255.65535.
if ((major > 255 || minor > 255 || fork_patch > 65535)); then
  echo "set-version: $major.$minor.$fork_patch does not fit a Windows installer version" >&2
  exit 1
fi
version="$major.$minor.$fork_patch"

cd "$(git rev-parse --show-toplevel)"

# Replaces the first line matching the pattern; fails when there is none.
first_line() {
  local file=$1 pattern=$2 line=$3
  if ! awk -v re="$pattern" -v line="$line" \
    '!done && $0 ~ re { print line; done = 1; next } { print } END { exit !done }' \
    "$file" >"$file.tmp"; then
    rm -f "$file.tmp"
    echo "set-version: no line matching '$pattern' in $file" >&2
    exit 1
  fi
  mv "$file.tmp" "$file"
}

first_line src-tauri/tauri.conf.json '^  "version": "[^"]*",$' "  \"version\": \"$version\","
first_line package.json '^  "version": "[^"]*",$' "  \"version\": \"$version\","
first_line src-tauri/Cargo.toml '^version = "[^"]*"$' "version = \"$version\""
first_line src/components/settings/voice/forkBuild.json '^  "numbered": false$' '  "numbered": true'

# Cargo.lock: the version line right after name = "handy".
if ! awk -v line="version = \"$version\"" '
  prev == "name = \"handy\"" && /^version = / { print line; n++; prev = ""; next }
  { print; prev = $0 }
  END { exit n != 1 }' src-tauri/Cargo.lock >src-tauri/Cargo.lock.tmp; then
  rm -f src-tauri/Cargo.lock.tmp
  echo "set-version: no version for the handy package in src-tauri/Cargo.lock" >&2
  exit 1
fi
mv src-tauri/Cargo.lock.tmp src-tauri/Cargo.lock

echo "$version"
