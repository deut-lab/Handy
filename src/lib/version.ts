export const getShownVersion = (version: string): string => {
  return version.replace("+", ".");
};
