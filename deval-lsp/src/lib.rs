use std::sync::Arc;

use dashmap::DashMap;
use deval_data_model::{Format, SemanticType};
use deval_validator::Validator;
use line_index::{LineCol, TextSize};
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

mod document;

use document::Document;

struct Backend {
    client: Client,
    documents: DashMap<Uri, Document>,
    format: Arc<dyn Format>,
    schema: Arc<dyn Validator>,
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Deval LSP".to_string(),
                version: Some("0.1".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: Some(false),
                            },
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::new("namespace"),
                                    SemanticTokenType::new("type"),
                                    SemanticTokenType::new("class"),
                                    SemanticTokenType::new("enum"),
                                    SemanticTokenType::new("interface"),
                                    SemanticTokenType::new("struct"),
                                    SemanticTokenType::new("typeParameter"),
                                    SemanticTokenType::new("parameter"),
                                    SemanticTokenType::new("variable"), // 8
                                    SemanticTokenType::new("property"),
                                    SemanticTokenType::new("enumMember"),
                                    SemanticTokenType::new("event"),
                                    SemanticTokenType::new("function"),
                                    SemanticTokenType::new("method"),
                                    SemanticTokenType::new("macro"),
                                    SemanticTokenType::new("keyword"),
                                    SemanticTokenType::new("modifier"),
                                    SemanticTokenType::new("comment"),
                                    SemanticTokenType::new("string"), // 18
                                    SemanticTokenType::new("number"), // 19
                                    SemanticTokenType::new("regexp"),
                                    SemanticTokenType::new("operator"),
                                    SemanticTokenType::new("decorator"),
                                ],
                                token_modifiers: vec![],
                            },
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        },
                    ),
                ),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                // hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "did open!")
            .await;

        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents.insert(
            uri,
            Document::new(&text, self.format.clone(), self.schema.clone()),
        );
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.content_changes[0].text.clone();

        if let Some(mut doc) = self.documents.get_mut(&uri) {
            doc.update_text(&text);
        } else {
            self.client
                .log_message(MessageType::ERROR, "did change for non-existing file!")
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "did close!")
            .await;

        let uri = params.text_document.uri;
        self.documents.remove(&uri);
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        self.client.log_message(MessageType::INFO, "full!").await;

        let Some(doc) = self.documents.get(&params.text_document.uri) else {
            self.client
                .log_message(MessageType::ERROR, "doc was missing!")
                .await;
            return Ok(None);
        };
        let Some(data) = doc.annotated.as_ref() else {
            self.client
                .log_message(MessageType::ERROR, "parse was failing!")
                .await;
            return Ok(None);
        };
        let mut result = vec![];
        let mut prev_line = 0;
        let mut prev_col = 0;

        data.value.walk(&mut |annotation| {
            let span = annotation.span.primary();
            let token_type = match annotation.semantic_type {
                Some(SemanticType::Number) => 19,
                Some(SemanticType::String) => 18,
                Some(SemanticType::Variable) => 8,
                None => return,
            };
            let l = doc
                .line_index
                .line_col(TextSize::try_from(span.start).unwrap());
            if l.line != prev_line {
                prev_col = 0;
            }
            result.push(SemanticToken {
                delta_line: l.line - prev_line,
                delta_start: l.col - prev_col,
                length: (span.end - span.start) as u32,
                token_type,
                token_modifiers_bitset: 0,
            });
            prev_col = l.col;
            prev_line = l.line;
        });
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: result,
        })))
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        self.client.log_message(MessageType::INFO, "range!").await;

        let Some(doc) = self.documents.get(&params.text_document.uri) else {
            self.client
                .log_message(MessageType::ERROR, "doc was missing!")
                .await;
            return Ok(None);
        };

        // Convert LSP range to byte offsets
        let start_offset: usize = doc
            .line_index
            .offset(LineCol {
                line: params.range.start.line,
                col: params.range.start.character,
            })
            .unwrap()
            .into();
        let end_offset: usize = doc
            .line_index
            .offset(LineCol {
                line: params.range.end.line,
                col: params.range.end.character,
            })
            .unwrap()
            .into();

        // Get tokens in range from our token store
        let tokens = doc.token_store.tokens_in_range(start_offset, end_offset);

        let mut result = vec![];
        let mut prev_line = params.range.start.line;
        let mut prev_col = 0;

        for token in tokens {
            let l = doc
                .line_index
                .line_col(TextSize::try_from(token.start).unwrap());
            
            // Convert our internal semantic type to LSP token type
            let token_type = match token.token_type {
                SemanticType::Number => 19,
                SemanticType::String => 18,
                SemanticType::Variable => 8,
            };

            if l.line != prev_line {
                prev_col = 0;
            }

            // Only include tokens that are within the requested range
            if l.line >= params.range.start.line && l.line <= params.range.end.line {
                result.push(SemanticToken {
                    delta_line: l.line - prev_line,
                    delta_start: l.col - prev_col,
                    length: (token.end - token.start) as u32,
                    token_type,
                    token_modifiers_bitset: 0,
                });
                prev_col = l.col;
                prev_line = l.line;
            }
        }

        Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
            result_id: None,
            data: result,
        })))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let Some(doc) = self.documents.get(&params.text_document_position_params.text_document.uri) else {
            return Ok(None);
        };

        // Convert LSP position to byte offset
        let offset: usize = doc
            .line_index
            .offset(LineCol {
                line: params.text_document_position_params.position.line,
                col: params.text_document_position_params.position.character,
            })
            .unwrap()
            .into();

        // Find the smallest token containing this position
        let token = doc.token_store.smallest_token_containing(offset);
        
        if let Some(token) = token {
            // Create hover content based on token type
            let content = match token.token_type {
                SemanticType::Number => "Number literal",
                SemanticType::String => "String literal",
                SemanticType::Variable => "Variable",
            };
            
            return Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(content.to_string())),
                range: None,
            }));
        }

        Ok(None)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub async fn start_server(format: Arc<dyn Format>, schema: Arc<dyn Validator>) {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: DashMap::new(),
        format,
        schema,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
