import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

import {
  debugAdapterArgs,
  resolveDebugAdapterCommand,
  resolveServerCommand,
} from "./serverPath";

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  context.subscriptions.push(
    vscode.commands.registerCommand("fkl.restartLanguageServer", async () => {
      await restartLanguageServer(context);
    }),
  );
  context.subscriptions.push(
    vscode.debug.registerDebugConfigurationProvider(
      "fkl",
      new FklDebugConfigurationProvider(),
    ),
  );
  context.subscriptions.push(
    vscode.debug.registerDebugAdapterDescriptorFactory(
      "fkl",
      new FklDebugAdapterDescriptorFactory(context),
    ),
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

function configuredDebugAdapterPath(): string {
  return vscode.workspace.getConfiguration("fkl").get("debug.adapterPath", "");
}

function workspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

class FklDebugConfigurationProvider implements vscode.DebugConfigurationProvider {
  resolveDebugConfiguration(
    _folder: vscode.WorkspaceFolder | undefined,
    config: vscode.DebugConfiguration,
  ): vscode.ProviderResult<vscode.DebugConfiguration> {
    const activeFklFile = activeFklFilePath();
    return {
      ...config,
      type: config.type ?? "fkl",
      name: config.name ?? "Debug FKL Time Travel",
      request: config.request ?? "launch",
      main: config.main ?? activeFklFile,
    };
  }
}

class FklDebugAdapterDescriptorFactory implements vscode.DebugAdapterDescriptorFactory {
  constructor(private readonly context: vscode.ExtensionContext) {}

  createDebugAdapterDescriptor(
    session: vscode.DebugSession,
  ): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
    const command = resolveDebugAdapterCommand(
      configuredDebugAdapterPath(),
      this.context.extensionPath,
    );
    const main = typeof session.configuration.main === "string"
      ? session.configuration.main
      : "";

    return new vscode.DebugAdapterExecutable(command, debugAdapterArgs(main), {
      cwd: workspaceRoot() ?? this.context.extensionPath,
    });
  }
}

function activeFklFilePath(): string | undefined {
  const editor = vscode.window.activeTextEditor;
  if (editor?.document.languageId !== "fkl") {
    return undefined;
  }

  return editor.document.uri.fsPath;
}
