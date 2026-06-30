import * as assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

import * as vscode from "vscode";

type TraceBody = {
  trace?: {
    frames?: Array<{
      implementation?: string;
      operation?: string;
      source?: {
        path?: string;
      };
    }>;
  };
};

type StackTraceBody = {
  stackFrames?: Array<{
    name?: string;
    source?: {
      path?: string;
    };
  }>;
};

type SeekBody = {
  frame?: number;
};

export async function run(): Promise<void> {
  const repoRoot = requireRepoRoot();
  const sampleFile = path.join(repoRoot, "docs", "samples", "impl.fkl");
  const fklCli = path.join(
    repoRoot,
    "target",
    "debug",
    process.platform === "win32" ? "fkl.exe" : "fkl",
  );

  assert.ok(fs.existsSync(sampleFile), `missing sample fixture: ${sampleFile}`);
  assert.ok(fs.existsSync(fklCli), `missing fkl debug adapter binary: ${fklCli}`);

  const document = await vscode.workspace.openTextDocument(vscode.Uri.file(sampleFile));
  await vscode.window.showTextDocument(document);
  assert.equal(document.languageId, "fkl");

  const extension = vscode.extensions.getExtension("feakin.fkl-vscode");
  assert.ok(extension, "FKL extension is not installed in the development host");
  if (!extension.isActive) {
    await extension.activate();
  }

  let session: vscode.DebugSession | undefined;
  try {
    const started = waitForDebugSessionStart("fkl");
    const accepted = await vscode.debug.startDebugging(vscode.workspace.workspaceFolders?.[0], {
      type: "fkl",
      request: "launch",
      name: "Debug FKL Time Travel",
      main: "${file}",
    });

    assert.equal(accepted, true);
    session = await started;
    assert.equal(session.type, "fkl");
    assert.equal(vscode.debug.activeDebugSession?.id, session.id);

    const traceBody = await waitForTimeTravelTrace(session);
    const frames = traceBody.trace?.frames ?? [];
    assert.ok(frames.length >= 2, "expected the sample flow to produce replayable frames");
    assert.equal(frames[0].implementation, "UserCreated");
    assert.equal(frames[0].operation, "UserRepository.getUserById");
    assert.equal(path.normalize(frames[0].source?.path ?? ""), path.normalize(sampleFile));

    const stackTrace = await session.customRequest("stackTrace", {
      threadId: 1,
      startFrame: 0,
      levels: 1,
    }) as StackTraceBody;
    assert.equal(stackTrace.stackFrames?.[0]?.name, "UserRepository.getUserById");
    assert.equal(
      path.normalize(stackTrace.stackFrames?.[0]?.source?.path ?? ""),
      path.normalize(sampleFile),
    );

    const seek = await session.customRequest("timeTravelSeek", { frame: 1 }) as SeekBody;
    assert.equal(seek.frame, 1);
  } finally {
    if (session) {
      await vscode.debug.stopDebugging(session);
    }
  }
}

function requireRepoRoot(): string {
  const repoRoot = process.env.FKL_REPO_ROOT;
  assert.ok(repoRoot, "FKL_REPO_ROOT must point at the repository root");
  return repoRoot;
}

function waitForDebugSessionStart(type: string): Promise<vscode.DebugSession> {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      subscription.dispose();
      reject(new Error(`Timed out waiting for ${type} debug session`));
    }, 10_000);

    const subscription = vscode.debug.onDidStartDebugSession((session) => {
      if (session.type !== type) {
        return;
      }

      clearTimeout(timeout);
      subscription.dispose();
      resolve(session);
    });
  });
}

async function waitForTimeTravelTrace(session: vscode.DebugSession): Promise<TraceBody> {
  let lastError: unknown;
  for (let attempt = 0; attempt < 20; attempt += 1) {
    try {
      const body = await session.customRequest("timeTravelTrace") as TraceBody;
      if ((body.trace?.frames?.length ?? 0) > 0) {
        return body;
      }
    } catch (error: unknown) {
      lastError = error;
    }

    await sleep(250);
  }

  throw new Error(`Timed out waiting for timeTravelTrace response: ${String(lastError)}`);
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
