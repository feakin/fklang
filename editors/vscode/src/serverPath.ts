import path from "node:path";

export function serverExecutableName(platform: NodeJS.Platform | string = process.platform): string {
  return platform === "win32" ? "fkl_lsp.exe" : "fkl_lsp";
}

export function debugAdapterExecutableName(platform: NodeJS.Platform | string = process.platform): string {
  return platform === "win32" ? "fkl.exe" : "fkl";
}

export function resolveServerCommand(
  configuredPath: string | undefined,
  extensionPath: string,
  platform: NodeJS.Platform | string = process.platform,
): string {
  const configured = configuredPath?.trim();
  if (configured) {
    return configured;
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

export function resolveDebugAdapterCommand(
  configuredPath: string | undefined,
  extensionPath: string,
  platform: NodeJS.Platform | string = process.platform,
): string {
  const configured = configuredPath?.trim();
  if (configured) {
    return configured;
  }

  return path.resolve(
    extensionPath,
    "..",
    "..",
    "target",
    "debug",
    debugAdapterExecutableName(platform),
  );
}

export function debugAdapterArgs(main: string | undefined): string[] {
  const mainPath = main?.trim();
  if (!mainPath) {
    return ["dap"];
  }

  return ["dap", "--main", mainPath];
}
