import path from "node:path";

export function serverExecutableName(platform: NodeJS.Platform | string = process.platform): string {
  return platform === "win32" ? "fkl_lsp.exe" : "fkl_lsp";
}

export function resolveServerCommand(
  configuredPath: string | undefined,
  extensionPath: string,
  platform: NodeJS.Platform | string = process.platform,
): string {
  const configured = configuredPath?.trim();
  if (configured) {
    return path.normalize(configured);
  }

  return path.resolve(
    extensionPath,
    "..",
    "..",
    "target",
    "debug",
    serverExecutableName(platform),
  );
}
