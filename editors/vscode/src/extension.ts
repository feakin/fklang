import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

import { resolveServerCommand } from "./serverPath";

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  context.subscriptions.push(
    vscode.commands.registerCommand("fkl.restartLanguageServer", async () => {
      await restartLanguageServer(context);
    }),
  );

  client = createLanguageClient(context);
  await client.start();
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
    client = undefined;
  }
}

async function restartLanguageServer(context: vscode.ExtensionContext): Promise<void> {
  if (client) {
    await client.stop();
  }

  client = createLanguageClient(context);
  await client.start();
}

function createLanguageClient(context: vscode.ExtensionContext): LanguageClient {
  const serverCommand = resolveServerCommand(
    configuredServerPath(),
    context.extensionPath,
  );

  const serverOptions: ServerOptions = {
    command: serverCommand,
    args: [],
    options: {
      cwd: workspaceRoot() ?? context.extensionPath,
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      {
        scheme: "file",
        language: "fkl",
      },
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.fkl"),
    },
  };

  return new LanguageClient(
    "fklLanguageServer",
    "FKL Language Server",
    serverOptions,
    clientOptions,
  );
}

function configuredServerPath(): string {
  return vscode.workspace.getConfiguration("fkl").get("lsp.serverPath", "");
}

function workspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}
