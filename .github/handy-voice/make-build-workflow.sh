#!/usr/bin/env bash
# Handy Voice: writes .github/workflows/handy-voice-build.yml from upstream's
# .github/workflows/build.yml with three small changes, so the fork's builds
# keep every build step upstream has at the Handy version the fork is based on:
#   1. the updater signing key is passed with sign-binaries: false too
#      (upstream passes it only together with Apple and Azure signing, and
#      Tauri refuses to build updater artifacts without it);
#   2. tauri-action does not upload latest.json (handy-voice.yml writes it
#      once, after all builds, so parallel builds cannot overwrite each other);
#   3. its own name.
# Usage: bash .github/handy-voice/make-build-workflow.sh   (from the repo)
# Fails when upstream's file no longer contains a line it changes.
set -euo pipefail

root=$(git rev-parse --show-toplevel)
src="$root/.github/workflows/build.yml"
out="$root/.github/workflows/handy-voice-build.yml"

text=$(cat "$src")

replace_once() {
  local from=$1 to=$2 count
  count=$(grep -cF -- "$from" <<<"$text" || true)
  if [ "$count" != 1 ]; then
    echo "make-build-workflow: expected one line with '$from' in build.yml, found $count" >&2
    exit 1
  fi
  text=${text/"$from"/"$to"}
}

empty="''"
replace_once \
  "TAURI_SIGNING_PRIVATE_KEY: \${{ inputs.sign-binaries && secrets.TAURI_SIGNING_PRIVATE_KEY || $empty }}" \
  "TAURI_SIGNING_PRIVATE_KEY: \${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}"
replace_once \
  "TAURI_SIGNING_PRIVATE_KEY_PASSWORD: \${{ inputs.sign-binaries && secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD || $empty }}" \
  "TAURI_SIGNING_PRIVATE_KEY_PASSWORD: \${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}"
replace_once \
  "          args: \${{ inputs.build-args }}" \
  "          args: \${{ inputs.build-args }}
          includeUpdaterJson: false"
replace_once 'name: "Build"' 'name: "Handy Voice build"'

{
  echo "# GENERATED from build.yml by .github/handy-voice/make-build-workflow.sh."
  echo "# Do not edit: run the script again after build.yml changes."
  echo
  printf '%s\n' "$text"
} >"$out"
