import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";

import {
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
    path.normalize("/workspace/fklang/target/debug/fkl_lsp"),
  );
});

test("serverExecutableName includes exe suffix on Windows", () => {
  assert.equal(serverExecutableName("win32"), "fkl_lsp.exe");
  assert.equal(serverExecutableName("linux"), "fkl_lsp");
});
