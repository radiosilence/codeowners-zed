use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::RwLock;

use codeowners::Owners;
use serde::Deserialize;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

macro_rules! log_stderr {
    ($($arg:tt)*) => {
        eprintln!("[codeowners-lsp] {}", format!($($arg)*));
    };
}

fn log_to_file(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/codeowners-lsp.log")
    {
        let _ = writeln!(file, "{}", msg);
    }
}

#[derive(Debug, Default, Deserialize)]
struct Settings {
    /// Custom path to CODEOWNERS file (relative to workspace root)
    path: Option<String>,
}

struct Backend {
    client: Client,
    workspace_root: RwLock<Option<PathBuf>>,
    codeowners: RwLock<Option<Owners>>,
    codeowners_path: RwLock<Option<PathBuf>>,
    settings: RwLock<Settings>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            workspace_root: RwLock::new(None),
            codeowners: RwLock::new(None),
            codeowners_path: RwLock::new(None),
            settings: RwLock::new(Settings::default()),
        }
    }

    async fn log(&self, msg: &str) {
        self.client.log_message(MessageType::INFO, msg).await;
    }

    fn load_codeowners(&self) -> Option<PathBuf> {
        let root = self.workspace_root.read().unwrap();
        let Some(root) = root.as_ref() else {
            return None;
        };

        let settings = self.settings.read().unwrap();

        // If custom path is set, use it
        if let Some(custom_path) = &settings.path {
            let path = root.join(custom_path);
            if path.exists() {
                let owners = codeowners::from_path(&path);
                *self.codeowners.write().unwrap() = Some(owners);
                *self.codeowners_path.write().unwrap() = Some(path.clone());
                return Some(path);
            }
        }

        // Otherwise use the crate's locate function
        if let Some(path) = codeowners::locate(root) {
            let owners = codeowners::from_path(&path);
            *self.codeowners.write().unwrap() = Some(owners);
            *self.codeowners_path.write().unwrap() = Some(path.clone());
            return Some(path);
        }

        *self.codeowners.write().unwrap() = None;
        *self.codeowners_path.write().unwrap() = None;
        None
    }

    fn get_owners_for_file(&self, uri: &Url) -> Option<String> {
        let root = self.workspace_root.read().unwrap();
        let root = root.as_ref()?;

        let file_path = uri.to_file_path().ok()?;
        let relative_path = file_path.strip_prefix(root).ok()?;

        let codeowners = self.codeowners.read().unwrap();
        let codeowners = codeowners.as_ref()?;

        let owners = codeowners.of(relative_path)?;
        let owner_strs: Vec<String> = owners.iter().map(|o| o.to_string()).collect();

        Some(owner_strs.join(", "))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log_stderr!("initialize called");
        log_to_file("initialize called");
        self.log("codeowners-lsp: initialize called").await;

        if let Some(root_uri) = &params.root_uri {
            self.log(&format!("codeowners-lsp: root_uri = {}", root_uri))
                .await;
            if let Ok(path) = root_uri.to_file_path() {
                *self.workspace_root.write().unwrap() = Some(path.clone());
                self.log(&format!("codeowners-lsp: workspace_root = {:?}", path))
                    .await;
            }
        } else {
            self.log("codeowners-lsp: no root_uri provided").await;
        }

        // Parse initialization options for settings
        if let Some(opts) = &params.initialization_options {
            self.log(&format!("codeowners-lsp: init_options = {:?}", opts))
                .await;
            if let Ok(settings) = serde_json::from_value::<Settings>(opts.clone()) {
                *self.settings.write().unwrap() = settings;
            }
        }

        if let Some(path) = self.load_codeowners() {
            self.log(&format!(
                "codeowners-lsp: loaded CODEOWNERS from {:?}",
                path
            ))
            .await;
        } else {
            self.log("codeowners-lsp: no CODEOWNERS file found").await;
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::NONE,
                )),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["codeowners.showOwner".to_string()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "codeowners-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.log("codeowners-lsp: initialized notification received")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.log("codeowners-lsp: shutdown").await;
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        log_stderr!("hover request for {}", uri);
        log_to_file(&format!("hover request for {}", uri));
        self.log(&format!("codeowners-lsp: hover request for {}", uri))
            .await;

        let owners = self.get_owners_for_file(uri);
        self.log(&format!("codeowners-lsp: owners = {:?}", owners))
            .await;

        let Some(owners) = owners else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("**CODEOWNERS:** {}", owners),
            }),
            range: None,
        }))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = &params.text_document.uri;
        log_stderr!("inlay_hint request for {}", uri);
        log_to_file(&format!("inlay_hint request for {}", uri));
        self.log(&format!("codeowners-lsp: inlay_hint request for {}", uri))
            .await;

        let owners = self.get_owners_for_file(uri);
        self.log(&format!("codeowners-lsp: inlay owners = {:?}", owners))
            .await;

        let Some(owners) = owners else {
            return Ok(None);
        };

        Ok(Some(vec![InlayHint {
            position: Position {
                line: 0,
                character: 0,
            },
            label: InlayHintLabel::String(format!("Owned by: {}", owners)),
            kind: None,
            text_edits: None,
            tooltip: Some(InlayHintTooltip::String(
                "File ownership from CODEOWNERS".to_string(),
            )),
            padding_left: Some(false),
            padding_right: Some(true),
            data: None,
        }]))
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        self.log(&format!(
            "codeowners-lsp: execute_command: {}",
            params.command
        ))
        .await;

        if params.command == "codeowners.showOwner" {
            if let Some(uri_value) = params.arguments.first() {
                if let Ok(uri) = serde_json::from_value::<String>(uri_value.clone()) {
                    if let Ok(uri) = Url::parse(&uri) {
                        if let Some(owners) = self.get_owners_for_file(&uri) {
                            self.client
                                .show_message(MessageType::INFO, format!("Owners: {}", owners))
                                .await;
                        } else {
                            self.client
                                .show_message(
                                    MessageType::INFO,
                                    "No CODEOWNERS rule matches this file",
                                )
                                .await;
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    async fn did_change_watched_files(&self, _params: DidChangeWatchedFilesParams) {
        self.log("codeowners-lsp: did_change_watched_files").await;
        self.load_codeowners();
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        self.log(&format!(
            "codeowners-lsp: did_change_configuration: {:?}",
            params.settings
        ))
        .await;
        if let Ok(settings) = serde_json::from_value::<Settings>(params.settings) {
            *self.settings.write().unwrap() = settings;
            self.load_codeowners();
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
