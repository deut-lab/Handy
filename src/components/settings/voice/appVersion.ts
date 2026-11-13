// Handy Voice: the name and version shown in About and in the footer, both
// taken from the build (tauri.conf.json), never typed in by hand.
import { getName, getVersion } from "@tauri-apps/api/app";
import i18n from "@/i18n";
import forkBuild from "./forkBuild.json";

/**
 * Builds made by the handy-voice GitHub workflow are numbered X.Y.(Z*100+N):
 * Handy X.Y.Z, fork build N. So 0.10.2 is Handy 0.10.0 build 2 and 0.10.100
 * is Handy 0.10.1 build 0, and the updater always sees a higher version.
 * The workflow sets `numbered` in forkBuild.json for these builds only; a
 * local build keeps Handy's own version and is shown as it is.
 */
export const splitForkVersion = (
  version: string,
): { handy: string; build: number } | null => {
  const match = version.match(/^(\d+)\.(\d+)\.(\d+)$/);
  if (!match) return null;
  const patch = Number(match[3]);
  return {
    handy: `${match[1]}.${match[2]}.${Math.floor(patch / 100)}`,
    build: patch % 100,
  };
};

/**
 * "0.10.0", "0.10.0 · build 2" for a workflow build; a build number in a
 * local version ("0.10.0+1") is shown as "0.10.0.1".
 */
export const shownVersion = (version: string): string => {
  const fork = forkBuild.numbered ? splitForkVersion(version) : null;
  if (!fork) return version.replace("+", ".");
  if (fork.build === 0) return fork.handy;
  return `${fork.handy} · ${i18n.t("voice.appVersion.build", { n: fork.build })}`;
};

/** "Handy Voice v0.10.0", "Handy Voice v0.10.0 · build 2". */
export const getAppLabel = async (): Promise<string> => {
  const [name, version] = await Promise.all([getName(), getVersion()]);
  return `${name} v${shownVersion(version)}`;
};
