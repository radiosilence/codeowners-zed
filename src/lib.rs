use std::fs;
use std::path::PathBuf;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct CodeownersExtension {
    binary_cache: Option<PathBuf>,
}

static LSP_NAME: &str = "codeowners-lsp";

impl CodeownersExtension {
    fn new() -> Self {
        Self { binary_cache: None }
    }

    fn get_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<PathBuf> {
        // Check if user has a custom binary path configured
        if let Ok(lsp_settings) = LspSettings::for_worktree(LSP_NAME, worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    return Ok(PathBuf::from(path));
                }
            }
        }

        // Check if binary is in PATH
        if let Some(path) = worktree.which(LSP_NAME) {
            return Ok(PathBuf::from(path));
        }

        // Check cache
        if let Some(path) = &self.binary_cache {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // Download from GitHub releases
        self.install_binary(language_server_id)
    }

    fn install_binary(&mut self, language_server_id: &zed::LanguageServerId) -> Result<PathBuf> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "radiosilence/codeowners-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("Failed to fetch latest release: {e}"))?;

        let (platform, arch) = zed::current_platform();
        let arch_name = match arch {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x86_64",
            zed::Architecture::X86 => return Err("x86 architecture is not supported".into()),
        };

        let (os_str, file_ext) = match platform {
            zed::Os::Mac => ("apple-darwin", "tar.gz"),
            zed::Os::Linux => ("unknown-linux-gnu", "tar.gz"),
            zed::Os::Windows => ("pc-windows-msvc", "zip"),
        };

        let asset_name = format!("{LSP_NAME}-{arch_name}-{os_str}.{file_ext}");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("No compatible binary found for {arch_name}-{os_str}"))?;

        let version_dir = format!("{LSP_NAME}-{}", release.version);
        let mut binary_path = PathBuf::from(&version_dir).join(LSP_NAME);

        if platform == zed::Os::Windows {
            binary_path.set_extension("exe");
        }

        if !binary_path.exists() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_result = (|| -> Result<()> {
                zed::download_file(
                    &asset.download_url,
                    &version_dir,
                    if platform == zed::Os::Windows {
                        zed::DownloadedFileType::Zip
                    } else {
                        zed::DownloadedFileType::GzipTar
                    },
                )
                .map_err(|e| format!("Failed to download binary: {e}"))?;

                zed::make_file_executable(binary_path.to_str().ok_or("Invalid binary path")?)
                    .map_err(|e| format!("Failed to make binary executable: {e}"))?;

                Ok(())
            })();

            if let Err(e) = download_result {
                fs::remove_dir_all(&version_dir).ok();
                return Err(e);
            }

            // Clean up old versions
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name != version_dir && name.starts_with(LSP_NAME) {
                            fs::remove_dir_all(entry.path()).ok();
                        }
                    }
                }
            }
        }

        self.binary_cache = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for CodeownersExtension {
    fn new() -> Self {
        Self::new()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.get_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary_path
                .to_str()
                .ok_or("Invalid binary path")?
                .to_string(),
            args: vec![],
            env: worktree.shell_env(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone());

        Ok(settings)
    }
}

zed::register_extension!(CodeownersExtension);
