//! AZC Language Server
//!
//! Provides IDE support for AZC via Language Server Protocol.

use anyhow::Result;
use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();

    log::info!("Starting AZC Language Server");

    let (connection, io_threads) = Connection::stdio();

    let server = AzcLanguageServer::new(connection);
    server.run()?;

    io_threads.join()?;

    log::info!("AZC Language Server stopped");

    Ok(())
}

struct AzcLanguageServer {
    connection: Connection,
    documents: HashMap<String, String>,
}

impl AzcLanguageServer {
    fn new(connection: Connection) -> Self {
        AzcLanguageServer {
            connection,
            documents: HashMap::new(),
        }
    }

    fn run(self) -> Result<()> {
        // Initialize
        let initialize_params = self.wait_for_initialize()?;
        self.send_initialize_response(&initialize_params.id)?;

        // Main loop
        for msg in &self.connection.receiver {
            match msg {
                Message::Request(req) => {
                    if self.handle_request(req)? {
                        break;
                    }
                }
                Message::Notification(notif) => {
                    self.handle_notification(notif)?;
                }
                Message::Response(_) => {}
            }
        }

        Ok(())
    }

    fn wait_for_initialize(&self) -> Result<Request> {
        for msg in &self.connection.receiver {
            if let Message::Request(req) = msg {
                if req.method == "initialize" {
                    return Ok(req);
                }
            }
        }
        anyhow::bail!("Expected initialize request")
    }

    fn send_initialize_response(&self, id: &RequestId) -> Result<()> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(vec![".".to_string(), ":" .to_string()]),
                ..Default::default()
            }),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            definition_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            rename_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                DiagnosticOptions {
                    inter_file_dependencies: false,
                    workspace_diagnostics: false,
                    ..Default::default()
                },
            )),
            ..Default::default()
        };

        let result = InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: "azc-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        };

        let response = Response {
            id: id.clone(),
            result: Some(serde_json::to_value(result)?),
            error: None,
        };

        self.connection.sender.send(Message::Response(response))?;

        Ok(())
    }

    fn handle_request(&self, req: Request) -> Result<bool> {
        match req.method.as_str() {
            "shutdown" => {
                self.send_response(req.id, None)?;
                return Ok(true);
            }
            "textDocument/completion" => {
                let params: CompletionParams = serde_json::from_value(req.params.clone())?;
                let result = self.handle_completion(&params)?;
                self.send_response(req.id, Some(serde_json::to_value(result)?))?;
            }
            "textDocument/hover" => {
                let params: HoverParams = serde_json::from_value(req.params.clone())?;
                if let Some(result) = self.handle_hover(&params)? {
                    self.send_response(req.id, Some(serde_json::to_value(result)?))?;
                } else {
                    self.send_response(req.id, None)?;
                }
            }
            "textDocument/definition" => {
                let params: GotoDefinitionParams = serde_json::from_value(req.params.clone())?;
                if let Some(result) = self.handle_definition(&params)? {
                    self.send_response(req.id, Some(serde_json::to_value(result)?))?;
                } else {
                    self.send_response(req.id, None)?;
                }
            }
            "textDocument/formatting" => {
                let params: DocumentFormattingParams = serde_json::from_value(req.params.clone())?;
                let result = self.handle_formatting(&params)?;
                self.send_response(req.id, Some(serde_json::to_value(result)?))?;
            }
            _ => {
                self.send_response(req.id, None)?;
            }
        }
        Ok(false)
    }

    fn handle_notification(&mut self, notif: lsp_server::Notification) -> Result<()> {
        match notif.method.as_str() {
            "textDocument/didOpen" => {
                let params: DidOpenTextDocumentParams =
                    serde_json::from_value(notif.params)?;
                let uri = params.text_document.uri.to_string();
                self.documents.insert(uri, params.text_document.text);
            }
            "textDocument/didChange" => {
                let params: DidChangeTextDocumentParams =
                    serde_json::from_value(notif.params)?;
                let uri = params.text_document.uri.to_string();
                
                if let Some(doc) = self.documents.get_mut(&uri) {
                    for change in params.content_changes {
                        if let Some(range) = change.range {
                            // Apply incremental change
                            *doc = apply_change(doc, range, &change.text);
                        } else {
                            // Full update
                            *doc = change.text;
                        }
                    }
                }
            }
            "textDocument/didClose" => {
                let params: DidCloseTextDocumentParams =
                    serde_json::from_value(notif.params)?;
                self.documents.remove(&params.text_document.uri.to_string());
            }
            _ => {}
        }
        Ok(())
    }

    fn send_response(&self, id: RequestId, result: Option<serde_json::Value>) -> Result<()> {
        let response = Response {
            id,
            result,
            error: None,
        };
        self.connection.sender.send(Message::Response(response))?;
        Ok(())
    }

    fn handle_completion(&self, params: &CompletionParams) -> Result<CompletionResponse> {
        let uri = &params.text_document_position.text_document.uri;
        let position = &params.text_document_position.position;

        let completions = vec![
            // Keywords
            CompletionItem {
                label: "let".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Variable declaration".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "def".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Function definition".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "if".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            CompletionItem {
                label: "else".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            CompletionItem {
                label: "while".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            CompletionItem {
                label: "end".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            CompletionItem {
                label: "return".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            CompletionItem {
                label: "struct".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            CompletionItem {
                label: "enum".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            CompletionItem {
                label: "impl".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            },
            // Built-in functions
            CompletionItem {
                label: "puts".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Print with newline".to_string()),
                insert_text: Some("puts ${1:value}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "print".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Print without newline".to_string()),
                ..Default::default()
            },
        ];

        Ok(CompletionResponse::Array(completions))
    }

    fn handle_hover(&self, params: &HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = &params.text_document_position_params.position;

        // Simplified - would need actual semantic analysis
        if let Some(doc) = self.documents.get(&uri.to_string()) {
            let line = doc.lines().nth(position.line as usize);
            if let Some(line) = line {
                // Check for known symbols
                if line.contains("puts") {
                    return Ok(Some(Hover {
                        contents: HoverContents::Scalar(MarkedString::String(
                            "```azc\nfn puts(value: String) -> Nil\n```\n\nPrints a string with a newline.".to_string(),
                        )),
                        range: None,
                    }));
                }
            }
        }

        Ok(None)
    }

    fn handle_definition(&self, params: &GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // Simplified - would need proper symbol table
        Ok(None)
    }

    fn handle_formatting(&self, params: &DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;
        
        if let Some(doc) = self.documents.get(&uri.to_string()) {
            let formatted = format_document(doc);
            
            let edit = TextEdit {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { 
                        line: doc.lines().count() as u32,
                        character: 0,
                    },
                },
                new_text: formatted,
            };
            
            return Ok(Some(vec![edit]));
        }

        Ok(None)
    }
}

fn apply_change(text: &str, range: Range, new_text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    
    // Simplified implementation
    let mut result = String::new();
    
    for (i, line) in lines.iter().enumerate() {
        if i as u32 == range.start.line {
            let start_char = range.start.character as usize;
            let end_char = if range.end.line == range.start.line {
                range.end.character as usize
            } else {
                line.len()
            };
            
            result.push_str(&line[..start_char]);
            result.push_str(new_text);
            result.push_str(&line[end_char..]);
        } else if i as u32 > range.end.line || i as u32 < range.start.line {
            result.push_str(line);
        }
        
        if i < lines.len() - 1 {
            result.push('\n');
        }
    }
    
    result
}

fn format_document(text: &str) -> String {
    // Simplified formatter
    text.lines()
        .map(|line| {
            let trimmed = line.trim_end();
            trimmed.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}