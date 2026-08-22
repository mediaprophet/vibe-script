const vscode = require("vscode");
const { LanguageClient, TransportKind } = require("vscode-languageclient/node");

let client;

function activate(context) {
  const command = vscode.workspace.getConfiguration("vibe").get("lsp.path") || "vibe-lsp";
  client = new LanguageClient(
    "vibe",
    "VibeScript",
    { command, transport: TransportKind.stdio },
    { documentSelector: [{ scheme: "file", language: "vibe" }] }
  );
  client.start();
  context.subscriptions.push({ dispose: () => client && client.stop() });
}

function deactivate() {
  return client ? client.stop() : undefined;
}

module.exports = { activate, deactivate };
