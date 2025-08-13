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

    // Register the restart command
    const restartCommand = vscode.commands.registerCommand('deval.restartServer', async () => {
        vscode.window.showInformationMessage('Restarting Deval language server...');
        
        if (client) {
            await client.stop();
            await client.start();
            
            vscode.window.showInformationMessage('Deval language server restarted successfully');
        } else {
            vscode.window.showErrorMessage('Deval language server is not running');
        }
    });

    context.subscriptions.push(restartCommand);
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}