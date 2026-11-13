// Handy Voice: the name and version shown in About and in the footer, both
// taken from the build (tauri.conf.json), never typed in by hand.
import { getName, getVersion } from "@tauri-apps/api/app";

/** A build number in the version ("0.10.0+1") is shown as "0.10.0.1". */
export const shownVersion = (version: string): string =>
  version.replace("+", ".");

/** "Handy Voice v0.10.0". */
export const getAppLabel = async (): Promise<string> => {
  const [name, version] = await Promise.all([getName(), getVersion()]);
  return `${name} v${shownVersion(version)}`;
};
