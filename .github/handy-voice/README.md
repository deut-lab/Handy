# Handy Voice automation

Handy Voice is Handy (cjpais/Handy) plus the fork's own commits (`voice: …`).
The workflow `.github/workflows/handy-voice.yml` keeps the fork on Handy's
latest release and builds it for Windows and macOS into a **draft** release.
A person publishes the draft; installed copies then update themselves through
the Tauri updater (`releases/latest/download/latest.json` of this repository).

## How it works

Every Monday (05:23 UTC) and on **Run workflow**:

1. **check** (ubuntu, a few seconds): reads `.github/handy-voice/base` (the
   Handy tag the fork is built on, e.g. `v0.10.0`) and Handy's latest
   non-prerelease release. Nothing newer and no `rebuild`: the run ends here.
2. **follow** (only when Handy has a newer release):
   - rebases the fork commits: `git rebase --onto <new> <base>`;
   - on a conflict it aborts, opens an issue (in Russian: which Handy version,
     which files) and fails. The branch is not touched;
   - otherwise writes the new tag into `base`, regenerates
     `handy-voice-build.yml`, commits `voice: base Handy <new>`, keeps the old
     branch tip as the tag `handy-voice-before-<new>`, force-pushes
     `handy-voice` (with lease) and starts a second run with `rebuild`. The
     build runs separately so it uses the build steps of the new Handy.
3. **release** (`rebuild` runs only): works out the fork version, commits it on
   top of the branch tip (that commit is only on the tag `voice-v<version>`,
   so the branch keeps Handy's own version and rebases never meet it), pushes
   the tag and creates a draft release with Russian notes: the Handy version,
   the fork build number and the fork's commit subjects.
4. **build**: Windows x86_64, macOS Apple Silicon and macOS Intel, through
   `handy-voice-build.yml`, which uploads the installers, update archives and
   `.sig` files into the draft. No Apple or Azure signing.
5. **finalize**: writes `latest.json` into the draft once, from the uploaded
   files.

`handy-voice-build.yml` is **generated** from Handy's `build.yml` by
`make-build-workflow.sh` with three changes: the updater signing key is passed
without Apple/Azure signing (upstream passes it only together with them, and
Tauri does not build updater artifacts without it), tauri-action does not
write `latest.json` (parallel builds would overwrite each other's), and its own
name. Never edit it by hand; run the script. Handy's own workflow files are
not changed.

Pushes go only to `handy-voice` and to tags, so none of Handy's workflows run
by themselves: those with a `push` trigger listen to `main` only, and the rest
are `pull_request` or manual. Pushes made with the default token start no
workflows at all.

## Versions

The updater offers only a strictly higher version, and Windows installers need
numeric parts, so a fork build is numbered `X.Y.(Z*100+N)`: Handy `X.Y.Z`,
fork build `N` (0 for the first build on a Handy version, then +1 per
`rebuild`, taken from the existing `voice-v*` tags).

| Handy  | Build | Version  | Tag               | Shown in the app              |
| ------ | ----- | -------- | ----------------- | ----------------------------- |
| 0.10.0 | 0     | 0.10.0   | `voice-v0.10.0`   | Handy Voice v0.10.0           |
| 0.10.0 | 2     | 0.10.2   | `voice-v0.10.2`   | Handy Voice v0.10.0 · build 2 |
| 0.10.1 | 0     | 0.10.100 | `voice-v0.10.100` | Handy Voice v0.10.1           |

The release commit sets the version in `src-tauri/tauri.conf.json`,
`src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `package.json` and marks the
build in `src/components/settings/voice/forkBuild.json` (`set-version.sh`), so
About and the footer show Handy's version and the build number. A local build
keeps Handy's own version and shows it as it is. A failed build leaves its tag,
and its number is skipped next time. Only Handy tags of the form `vX.Y.Z` are
followed; another form opens an issue.

## One-time setup

1. Secrets (Settings → Secrets and variables → Actions → Repository secrets):
   - `TAURI_SIGNING_PRIVATE_KEY`: the private updater key that matches the
     `pubkey` in `src-tauri/tauri.conf.json` (the whole content of the key file).
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: its password. If the key has none,
     leave this secret out.
   - `HANDY_VOICE_TOKEN`: a fine-grained personal access token, repository
     access only `deut-lab/Handy`, permissions **Contents: Read and write** and
     **Workflows: Read and write**. Moving the branch onto a new Handy also
     moves Handy's own workflow files, and GitHub refuses that push with the
     default token. Without this secret, following works only while Handy
     leaves its workflows alone. When it expires, the follow run fails and its
     issue says so; make a new one.
2. Make `handy-voice` the default branch (Settings → General). Scheduled
   workflows run only from the default branch.
3. Optional: disable Handy's own workflows in the Actions tab (Release, Build
   Test, Main Branch Build, PR Test Build, code quality, nix build check,
   Playwright, test). They never start by themselves here, but a manual run of
   Release would build seven platforms and fail on Apple signing.
4. Start the first build: Actions → Handy Voice → Run workflow, branch
   `handy-voice`, tick **rebuild**.

**The updater needs public release files.** It downloads `latest.json` and
the installers without logging in, so this repository is public. The fork's
history before Handy 0.10 lives in the private, archived `deut-lab/Handy-old`.

The updater key is the fork's original one (its public half is the `pubkey`
above), so copies of Handy Voice 0.8.x update to these releases by themselves.
It has no password: `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` is not set.

## Running by hand

Actions → Handy Voice → Run workflow on `handy-voice`:

- without `rebuild`: the weekly check now;
- with `rebuild`: a new fork build of the current branch (for fork-only
  changes). If Handy has a new release at that moment, the run moves the fork
  first and the build follows in a second run.

## Publishing a draft

1. Releases → the draft **Handy Voice <version>**. Check that the run is green
   and the draft has: `…_x64-setup.exe`, `…_x64_en-US.msi`, two `.dmg`, two
   `.app.tar.gz`, a `.sig` for each setup.exe, .msi and .app.tar.gz, and
   `latest.json`.
2. Optionally install the setup.exe on a test machine first.
3. Edit → keep **Set as the latest release** ticked → **Publish release**.
4. Installed copies offer the update at their next check.

To drop a draft: delete it and its tag `voice-v<version>`.

## Fixing a conflict

The issue names the Handy version and the files. Locally, in a Claude session,
by the fork's porting rules (PORTING.md next to the clone: own code in own
files, small hooks in Handy's files, LF line endings, `voice: …` commits):

```bash
git fetch origin
git fetch upstream tag v0.10.1 --no-tags           # the new Handy
git switch -c handy-voice-move origin/handy-voice
git tag handy-voice-before-v0.10.1 origin/handy-voice
git rebase --onto v0.10.1 "$(cat .github/handy-voice/base)"
#   resolve, git add, git rebase --continue, until done
echo v0.10.1 > .github/handy-voice/base
bash .github/handy-voice/make-build-workflow.sh    # adjust it if build.yml changed
git add .github/handy-voice/base .github/workflows/handy-voice-build.yml
git commit -m "voice: base Handy v0.10.1"
#   checks from PORTING.md (cargo check, bun run build)
git push origin handy-voice-before-v0.10.1
git push --force-with-lease=handy-voice:"$(git rev-parse origin/handy-voice)" origin HEAD:handy-voice
```

Then Actions → Handy Voice → Run workflow with **rebuild**, and close the
issue. If GitHub refuses the push because of workflow files, the local login
needs the `workflow` scope: `gh auth refresh -s workflow`.

## When a build fails

The draft stays without `latest.json`, so nobody gets it. Fix the cause, then
either **Re-run failed jobs** in that run (the finalize step follows), or
delete the draft and its tag and run with **rebuild** again.

## Cost (GitHub Actions minutes)

Each job is rounded up to whole minutes; on a private repository Windows counts
2×, macOS 10× (2000 free minutes a month). Public repositories pay nothing.

- Weekly check with nothing new: 1 minute.
- Moving onto a new Handy: about 3 minutes, plus the build.
- Full build: about 300–400 minutes: Windows ~30 min (×2), each Mac
  ~12–18 min (×10), ubuntu jobs ~3 (measured on upstream's release runs, which
  also notarize). Leaving out the Intel Mac saves about 120–170.
- So about 4–6 builds a month fit into the free minutes.
