"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = require("vscode");
const fs = require("fs");
const path = require("path");
let schema;
function activate(context) {
    // Create output channel
    const output = vscode.window.createOutputChannel("Scenario DSL");
    output.show(true);
    output.appendLine("Activating Scenario DSL extension...");
    try {
        const schemaPath = path.join(context.extensionPath, "src", "schema.json");
        schema = JSON.parse(fs.readFileSync(schemaPath, "utf-8"));
        output.appendLine(`Loaded schema with ${Object.keys(schema.steps).length} steps.`);
    }
    catch (err) {
        output.appendLine("Error loading schema.json:");
        output.appendLine(String(err));
        vscode.window.showErrorMessage("Scenario DSL: Failed to load schema.json. Check Output → Scenario DSL.");
        return; // don’t register provider if schema fails
    }
    const provider = vscode.languages.registerCompletionItemProvider("scenario", {
        provideCompletionItems(document, position) {
            try {
                const linePrefix = document.lineAt(position).text.slice(0, position.character);
                output.appendLine(`Completion requested for line: "${linePrefix}"`);
                // Suggest step names if line is empty or whitespace
                if (/^\s*$/.test(linePrefix)) {
                    const items = Object.keys(schema.steps).map(step => {
                        const item = new vscode.CompletionItem(step, vscode.CompletionItemKind.Function);
                        item.detail = "Scenario step";
                        return item;
                    });
                    output.appendLine(`Suggesting ${items.length} steps.`);
                    return items;
                }
                // Suggest fields after a step
                const stepMatch = linePrefix.match(/^(\w+)\s+/);
                if (stepMatch) {
                    const stepName = stepMatch[1];
                    const stepSchema = schema.steps[stepName];
                    output.appendLine(`Detected step: ${stepName}`);
                    if (stepSchema) {
                        const items = Object.entries(stepSchema.fields).map(([name, field]) => {
                            const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Property);
                            item.detail = field.doc;
                            item.insertText = `${name}=`;
                            return item;
                        });
                        output.appendLine(`Suggesting ${items.length} fields for step ${stepName}.`);
                        return items;
                    }
                    else {
                        output.appendLine(`Step ${stepName} not found in schema.`);
                    }
                }
                return [];
            }
            catch (err) {
                output.appendLine("Error in completion provider:");
                output.appendLine(String(err));
                return [];
            }
        }
    }, " ", "=" // trigger completions on space or '='
    );
    context.subscriptions.push(provider);
    context.subscriptions.push(output);
    output.appendLine("Scenario DSL extension activated.");
}
function deactivate() { }
//# sourceMappingURL=extension.js.map