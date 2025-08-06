import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";

let client: lc.LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    vscode.window.showErrorMessage("hello");

    client = new lc.LanguageClient(
        "deval-lsp",
        {
            command: "/root/oss/deval/target/debug/deval-cli",
            args: ["lsp"],
        },
        {
            documentSelector: [
                { scheme: 'file', language: 'plaintext' },
                { scheme: 'file', language: 'toml' },
            ],
        }
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}