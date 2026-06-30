import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";

import {
  debugAdapterArgs,
  debugAdapterExecutableName,
  resolveDebugAdapterCommand,
  resolveServerCommand,
  serverExecutableName,
} from "../src/serverPath";

test("resolveServerCommand uses configured server path when present", () => {
  assert.equal(
    resolveServerCommand("/opt/fkl/fkl_lsp", "/workspace/fklang/editors/vscode"),
    "/opt/fkl/fkl_lsp",
  );
});

test("resolveServerCommand falls back to workspace debug binary", () => {
  assert.equal(
    resolveServerCommand("", "/workspace/fklang/editors/vscode", "darwin"),
    path.resolve(
      "/workspace/fklang/editors/vscode",
      "..",
      "..",
      "target",
      "debug",
      "fkl_lsp",
    ),
  );
});

test("serverExecutableName includes exe suffix on Windows", () => {
  assert.equal(serverExecutableName("win32"), "fkl_lsp.exe");
  assert.equal(serverExecutableName("linux"), "fkl_lsp");
});

test("resolveDebugAdapterCommand falls back to fkl cli debug binary", () => {
  assert.equal(
    resolveDebugAdapterCommand("", "/workspace/fklang/editors/vscode", "darwin"),
    path.resolve(
      "/workspace/fklang/editors/vscode",
      "..",
      "..",
      "target",
      "debug",
      "fkl",
    ),
  );
});

test("debugAdapterExecutableName includes exe suffix on Windows", () => {
  assert.equal(debugAdapterExecutableName("win32"), "fkl.exe");
  assert.equal(debugAdapterExecutableName("linux"), "fkl");
});

test("debugAdapterArgs launches dap with optional main file", () => {
  assert.deepEqual(debugAdapterArgs("/workspace/main.fkl"), [
    "dap",
    "--main",
    "/workspace/main.fkl",
  ]);
  assert.deepEqual(debugAdapterArgs(""), ["dap"]);
});
