import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";

interface FieldSchema {
    ty: "String" | "Number";
    optional: boolean;
    doc: string;
}

interface StepSchema {
    fields: Record<string, FieldSchema>;
    doc?: string; // optional doc for the step itself
}

interface ScenarioSchema {
    steps: Record<string, StepSchema>;
}

let schema: ScenarioSchema;

export function activate(context: vscode.ExtensionContext) {
    const output = vscode.window.createOutputChannel("Scenario DSL");
    output.show(true);
    output.appendLine("Activating Scenario DSL extension...");

    try {
        const schemaPath = path.join(context.extensionPath, "src", "schema.json");
        schema = JSON.parse(fs.readFileSync(schemaPath, "utf-8"));
        output.appendLine(`Loaded schema with ${Object.keys(schema.steps).length} steps.`);
    } catch (err) {
        output.appendLine("Error loading schema.json:");
        output.appendLine(String(err));
        vscode.window.showErrorMessage(
            "Scenario DSL: Failed to load schema.json. Check Output → Scenario DSL."
        );
        return;
    }

    const provider = vscode.languages.registerCompletionItemProvider(
        "scenario",
        {
            provideCompletionItems(document, position) {
                try {
                    const linePrefix = document.lineAt(position).text.slice(0, position.character);
                    output.appendLine(`Completion requested for line: "${linePrefix}"`);

                    // Suggest step names if line is empty or whitespace
                    if (/^\s*$/.test(linePrefix)) {
                        const items = Object.entries(schema.steps).map(([stepName, step]) => {
                            const item = new vscode.CompletionItem(stepName, vscode.CompletionItemKind.Function);
                            item.detail = "Scenario step";
                            if (step.doc) item.documentation = new vscode.MarkdownString(step.doc);
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
                            const items = Object.entries(stepSchema.fields).map(
                                ([name, field]: [string, FieldSchema]) => {
                                    const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Property);
                                    item.detail = `${field.ty}${field.optional ? " (optional)" : ""}`;
                                    item.insertText = `${name}=`;
                                    item.documentation = new vscode.MarkdownString(field.doc);
                                    return item;
                                }
                            );
                            output.appendLine(`Suggesting ${items.length} fields for step ${stepName}.`);
                            return items;
                        } else {
                            output.appendLine(`Step ${stepName} not found in schema.`);
                        }
                    }

                    return [];
                } catch (err) {
                    output.appendLine("Error in completion provider:");
                    output.appendLine(String(err));
                    return [];
                }
            }
        },
        " ", "=" // trigger on space or '='
    );

    context.subscriptions.push(provider);
    context.subscriptions.push(output);
    output.appendLine("Scenario DSL extension activated.");
}

export function deactivate() { }