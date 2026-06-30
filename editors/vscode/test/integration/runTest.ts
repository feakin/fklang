import fs from "node:fs";
import path from "node:path";

import { runTests } from "@vscode/test-electron";

async function main(): Promise<void> {
  const extensionDevelopmentPath = path.resolve(__dirname, "..", "..", "..");
  const repoRoot = path.resolve(extensionDevelopmentPath, "..", "..");
  const extensionTestsPath = path.resolve(__dirname, "suite");

  await runTests({
    vscodeExecutablePath: localVSCodeExecutablePath(),
    extensionDevelopmentPath,
    extensionTestsPath,
    extensionTestsEnv: {
      FKL_REPO_ROOT: repoRoot,
    },
    launchArgs: [
      repoRoot,
      "--disable-extensions",
      "--disable-workspace-trust",
      "--skip-welcome",
      "--skip-release-notes",
    ],
  });
}

function localVSCodeExecutablePath(): string | undefined {
  const configured = process.env.FKL_VSCODE_EXECUTABLE?.trim();
  if (configured) {
    return configured;
  }

  if (process.platform === "darwin") {
    const stable = "/Applications/Visual Studio Code.app/Contents/MacOS/Electron";
    if (fs.existsSync(stable)) {
      return stable;
    }
  }

  return undefined;
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
