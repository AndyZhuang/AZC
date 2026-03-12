import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { execSync, spawn } from 'child_process';

let diagnosticCollection: vscode.DiagnosticCollection;
let outputChannel: vscode.OutputChannel;

export function activate(context: vscode.ExtensionContext) {
    console.log('AZC extension is now active');

    // Create output channel
    outputChannel = vscode.window.createOutputChannel('AZC');
    context.subscriptions.push(outputChannel);

    // Create diagnostic collection
    diagnosticCollection = vscode.languages.createDiagnosticCollection('azc');
    context.subscriptions.push(diagnosticCollection);

    // Register commands
    registerCommands(context);

    // Register document listeners
    registerDocumentListeners(context);

    // Check if AZC compiler is available
    checkCompiler();

    // Show welcome message
    showWelcomeMessage(context);
}

function registerCommands(context: vscode.ExtensionContext) {
    // Compile command
    const compileCmd = vscode.commands.registerCommand('azc.compile', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'azc') {
            compileFile(editor.document);
        } else {
            vscode.window.showWarningMessage('No AZC file is currently open');
        }
    });
    context.subscriptions.push(compileCmd);

    // Run command
    const runCmd = vscode.commands.registerCommand('azc.run', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'azc') {
            runFile(editor.document);
        } else {
            vscode.window.showWarningMessage('No AZC file is currently open');
        }
    });
    context.subscriptions.push(runCmd);

    // New project command
    const newProjectCmd = vscode.commands.registerCommand('azc.newProject', async () => {
        const name = await vscode.window.showInputBox({
            prompt: 'Enter project name',
            placeHolder: 'my-project'
        });
        if (name) {
            createNewProject(name);
        }
    });
    context.subscriptions.push(newProjectCmd);

    // Show safety report command
    const safetyReportCmd = vscode.commands.registerCommand('azc.showSafetyReport', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'azc') {
            showSafetyReport(editor.document);
        } else {
            vscode.window.showWarningMessage('No AZC file is currently open');
        }
    });
    context.subscriptions.push(safetyReportCmd);
}

function registerDocumentListeners(context: vscode.ExtensionContext) {
    // Save document listener
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(document => {
            if (document.languageId === 'azc') {
                const config = vscode.workspace.getConfiguration('azc');
                
                if (config.get<boolean>('formatOnSave')) {
                    // Format document
                }
                
                if (config.get<boolean>('lintOnSave')) {
                    lintDocument(document);
                }
                
                if (config.get<boolean>('safetyCheckOnSave')) {
                    checkSafety(document);
                }
            }
        })
    );

    // Change document listener for diagnostics
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            if (event.document.languageId === 'azc') {
                updateDiagnostics(event.document);
            }
        })
    );
}

function getCompilerPath(): string {
    const config = vscode.workspace.getConfiguration('azc');
    return config.get<string>('compilerPath') || 'azc';
}

function checkCompiler() {
    const compilerPath = getCompilerPath();
    try {
        const version = execSync(`${compilerPath} --version`, { encoding: 'utf-8' });
        outputChannel.appendLine(`AZC Compiler found: ${version.trim()}`);
    } catch (error) {
        outputChannel.appendLine('AZC Compiler not found. Please install it or configure the path in settings.');
        vscode.window.showWarningMessage(
            'AZC compiler not found. Please install it or configure the path in settings.',
            'Open Settings'
        ).then(selection => {
            if (selection === 'Open Settings') {
                vscode.commands.executeCommand('workbench.action.openSettings', 'azc.compilerPath');
            }
        });
    }
}

function compileFile(document: vscode.TextDocument) {
    const compilerPath = getCompilerPath();
    const filePath = document.uri.fsPath;
    
    outputChannel.show(true);
    outputChannel.appendLine(`Compiling: ${filePath}`);
    
    try {
        const output = execSync(`${compilerPath} ${filePath}`, {
            encoding: 'utf-8',
            cwd: path.dirname(filePath)
        });
        
        outputChannel.appendLine(output);
        outputChannel.appendLine('Compilation successful!');
        vscode.window.showInformationMessage('AZC: Compilation successful!');
        
        // Clear diagnostics
        diagnosticCollection.set(document.uri, []);
        
    } catch (error: any) {
        outputChannel.appendLine('Compilation failed:');
        outputChannel.appendLine(error.stderr || error.message);
        
        // Parse errors and add diagnostics
        const diagnostics = parseErrors(error.stderr || error.message, document);
        diagnosticCollection.set(document.uri, diagnostics);
        
        vscode.window.showErrorMessage('AZC: Compilation failed. See output for details.');
    }
}

function runFile(document: vscode.TextDocument) {
    const compilerPath = getCompilerPath();
    const filePath = document.uri.fsPath;
    
    outputChannel.show(true);
    outputChannel.appendLine(`Running: ${filePath}`);
    
    const process = spawn(compilerPath, ['run', filePath], {
        cwd: path.dirname(filePath)
    });
    
    process.stdout.on('data', (data) => {
        outputChannel.append(data.toString());
    });
    
    process.stderr.on('data', (data) => {
        outputChannel.append(data.toString());
    });
    
    process.on('close', (code) => {
        outputChannel.appendLine(`Process exited with code ${code}`);
    });
}

async function createNewProject(name: string) {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders) {
        vscode.window.showErrorMessage('Please open a workspace first');
        return;
    }
    
    const projectPath = path.join(workspaceFolders[0].uri.fsPath, name);
    
    try {
        // Create project directory
        fs.mkdirSync(projectPath, { recursive: true });
        fs.mkdirSync(path.join(projectPath, 'src'), { recursive: true });
        fs.mkdirSync(path.join(projectPath, 'tests'), { recursive: true });
        
        // Create azc.toml
        const azcToml = `[package]
name = "${name}"
version = "0.1.0"
edition = "2024"

[dependencies]
`;
        fs.writeFileSync(path.join(projectPath, 'azc.toml'), azcToml);
        
        // Create main.azc
        const mainAzc = `# ${name} - AZC Project

def main()
    puts "Hello, ${name}!"
end
`;
        fs.writeFileSync(path.join(projectPath, 'src', 'main.azc'), mainAzc);
        
        // Create README
        const readme = `# ${name}

An AZC project.

## Building

\`\`\`bash
azc build
\`\`\`

## Running

\`\`\`bash
azc run
\`\`\`
`;
        fs.writeFileSync(path.join(projectPath, 'README.md'), readme);
        
        vscode.window.showInformationMessage(`Project "${name}" created successfully!`);
        
        // Open main.azc
        const mainUri = vscode.Uri.file(path.join(projectPath, 'src', 'main.azc'));
        vscode.window.showTextDocument(mainUri);
        
    } catch (error: any) {
        vscode.window.showErrorMessage(`Failed to create project: ${error.message}`);
    }
}

function showSafetyReport(document: vscode.TextDocument) {
    const compilerPath = getCompilerPath();
    const filePath = document.uri.fsPath;
    
    outputChannel.show(true);
    outputChannel.appendLine(`Safety Report for: ${filePath}`);
    
    try {
        const output = execSync(`${compilerPath} safety ${filePath}`, {
            encoding: 'utf-8',
            cwd: path.dirname(filePath)
        });
        
        outputChannel.appendLine(output);
        
        // Create and show webview with safety report
        const panel = vscode.window.createWebviewPanel(
            'azcSafetyReport',
            'AZC Safety Report',
            vscode.ViewColumn.Beside,
            {}
        );
        
        panel.webview.html = generateSafetyReportHtml(output);
        
    } catch (error: any) {
        outputChannel.appendLine('Failed to generate safety report:');
        outputChannel.appendLine(error.message);
        vscode.window.showErrorMessage('Failed to generate safety report');
    }
}

function generateSafetyReportHtml(report: string): string {
    return `<!DOCTYPE html>
<html>
<head>
    <title>AZC Safety Report</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 20px; }
        h1 { color: #333; }
        .score { font-size: 48px; font-weight: bold; color: #4CAF50; }
        .check { margin: 10px 0; padding: 10px; border-radius: 4px; }
        .pass { background: #e8f5e9; border-left: 4px solid #4CAF50; }
        .warn { background: #fff3e0; border-left: 4px solid #ff9800; }
        .fail { background: #ffebee; border-left: 4px solid #f44336; }
        pre { background: #f5f5f5; padding: 10px; border-radius: 4px; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>🛡️ Safety Report</h1>
    <pre>${report}</pre>
</body>
</html>`;
}

function lintDocument(document: vscode.TextDocument) {
    // Placeholder for linting logic
    updateDiagnostics(document);
}

function checkSafety(document: vscode.TextDocument) {
    // Placeholder for safety checking
}

function updateDiagnostics(document: vscode.TextDocument) {
    // Simple syntax validation
    const diagnostics: vscode.Diagnostic[] = [];
    const text = document.getText();
    const lines = text.split('\n');
    
    // Check for unmatched 'end' keywords
    let depth = 0;
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();
        
        // Count block openers
        if (/^(def|if|while|for|class|struct|enum|impl|trait|async|unsafe|macro|extern|match)\b/.test(trimmed)) {
            depth++;
        }
        
        // Count 'end' closers
        if (trimmed === 'end') {
            depth--;
            if (depth < 0) {
                const diagnostic = new vscode.Diagnostic(
                    new vscode.Range(i, 0, i, 3),
                    "Unexpected 'end' keyword - no matching block opener",
                    vscode.DiagnosticSeverity.Error
                );
                diagnostics.push(diagnostic);
            }
        }
    }
    
    // Check for unclosed blocks
    if (depth > 0) {
        const lastLine = lines.length - 1;
        const diagnostic = new vscode.Diagnostic(
            new vscode.Range(lastLine, 0, lastLine, lines[lastLine].length),
            `Missing 'end' keyword - ${depth} unclosed block(s)`,
            vscode.DiagnosticSeverity.Error
        );
        diagnostics.push(diagnostic);
    }
    
    diagnosticCollection.set(document.uri, diagnostics);
}

function parseErrors(errorText: string, document: vscode.TextDocument): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];
    const errorRegex = /^(.+):(\d+):(\d+):\s+(error|warning):\s+(.+)$/gm;
    
    let match;
    while ((match = errorRegex.exec(errorText)) !== null) {
        const [, file, line, column, severity, message] = match;
        const lineNum = parseInt(line, 10) - 1;
        const colNum = parseInt(column, 10) - 1;
        
        const diagnostic = new vscode.Diagnostic(
            new vscode.Range(lineNum, colNum, lineNum, colNum + 10),
            message,
            severity === 'error' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning
        );
        diagnostics.push(diagnostic);
    }
    
    return diagnostics;
}

function showWelcomeMessage(context: vscode.ExtensionContext) {
    const showWelcome = context.globalState.get<boolean>('showWelcome', true);
    
    if (showWelcome) {
        vscode.window.showInformationMessage(
            'Welcome to AZC! A safe programming language for industrial control systems.',
            'Get Started',
            'View Docs',
            "Don't Show Again"
        ).then(selection => {
            if (selection === 'Get Started') {
                vscode.commands.executeCommand('azc.newProject');
            } else if (selection === 'View Docs') {
                vscode.env.openExternal(vscode.Uri.parse('https://azc.dev/docs'));
            } else if (selection === "Don't Show Again") {
                context.globalState.update('showWelcome', false);
            }
        });
    }
}

export function deactivate() {
    console.log('AZC extension deactivated');
}