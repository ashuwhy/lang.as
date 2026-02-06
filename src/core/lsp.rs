use lsp_server::{Connection, Message, Notification};
use serde_json::Value;
use aslang::parser::Parser;
use std::error::Error;

pub fn start_lsp() -> Result<(), Box<dyn Error + Send + Sync>> {
    eprintln!("Starting ASLang LSP server...");
    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&serde_json::json!({
        "capabilities": {
            "textDocumentSync": 1 // Full synchronization
        }
    })).unwrap();
    
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;
    Ok(())
}

fn main_loop(connection: Connection, _params: Value) -> Result<(), Box<dyn Error + Send + Sync>> {
    eprintln!("LSP Loop Started");
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
            }
            Message::Response(_) => {}
            Message::Notification(not) => {
                if not.method == "textDocument/didOpen" {
                    if let Ok(params) = serde_json::from_value::<Value>(not.params) {
                        if let (Some(uri), Some(text)) = (
                            params["textDocument"]["uri"].as_str(),
                            params["textDocument"]["text"].as_str()
                        ) {
                            validate_document(&connection, uri, text);
                        }
                    }
                } else if not.method == "textDocument/didChange" {
                    if let Ok(params) = serde_json::from_value::<Value>(not.params) {
                        if let (Some(uri), Some(changes)) = (
                            params["textDocument"]["uri"].as_str(),
                            params["contentChanges"].as_array()
                        ) {
                            if let Some(change) = changes.first() {
                                if let Some(text) = change["text"].as_str() {
                                    validate_document(&connection, uri, text);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_document(connection: &Connection, uri: &str, text: &str) {
    let diagnostics = match Parser::parse(text) {
        Ok(_) => vec![],
        Err(e) => {
            // Map ASError to Diagnostic
            let line = if e.location.line > 0 { e.location.line - 1 } else { 0 };
            let col = if e.location.column > 0 { e.location.column - 1 } else { 0 };
            
            vec![serde_json::json!({
                "range": {
                    "start": { "line": line, "character": col },
                    "end": { "line": line, "character": col + 1 }
                },
                "severity": 1, // Error
                "message": e.message,
                "source": "aslang"
            })]
        }
    };

    let params = serde_json::json!({
        "uri": uri,
        "diagnostics": diagnostics
    });
    
    let not = Notification::new("textDocument/publishDiagnostics".to_string(), params);
    let _ = connection.sender.send(Message::Notification(not));
}
