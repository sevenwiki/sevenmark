import * as path from "path";
import * as vscode from "vscode";

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

function getServerPath(context: vscode.ExtensionContext): string {
  if (process.env.SERVER_PATH) {
    return process.env.SERVER_PATH;
  }

  const config = vscode.workspace.getConfiguration("sevenmark");
  const configPath = config.get<string>("server.path", "");
  if (configPath) {
    return configPath;
  }

  const ext = process.platform === "win32" ? ".exe" : "";
  const bundled = path.join(
    context.extensionPath,
    "server",
    `sevenmark_language_server${ext}`
  );
  try {
    require("fs").accessSync(bundled, require("fs").constants.X_OK);
    return bundled;
  } catch {
    // not bundled
  }

  return "sevenmark_language_server";
}

export async function activate(context: vscode.ExtensionContext) {
  const outputChannel = vscode.window.createOutputChannel(
    "SevenMark Language Server"
  );

  const command = getServerPath(context);
  outputChannel.appendLine(`Server path: ${command}`);

  const run: Executable = {
    command,
  };

  const serverOptions: ServerOptions = {
    run,
    debug: run,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "sevenmark" }],
    traceOutputChannel: outputChannel,
    outputChannel,
  };

  client = new LanguageClient(
    "sevenmark",
    "SevenMark Language Server",
    serverOptions,
    clientOptions
  );

  await client.start();

  const caps = client.initializeResult?.capabilities;
  outputChannel.appendLine(
    `semanticTokensProvider: ${JSON.stringify(caps?.semanticTokensProvider)}`
  );
  outputChannel.appendLine("Server started successfully.");

  context.subscriptions.push(client);
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
