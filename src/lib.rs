use zed_extension_api::{self as zed, LanguageServerId, Result};

const REPO: &str = "DamirManyapov/routeros-lsp";
const BINARY: &str = "routeros-lsp";
/// Path to the server entry point inside an extracted release tarball.
const ENTRY: &str = "routeros-lsp/dist/server.js";

struct RouterOsExtension {
    /// Release tag of the server currently on disk, if any.
    installed: Option<String>,
}

impl RouterOsExtension {
    /// Downloads the server from GitHub Releases, skipping the work when the
    /// wanted version is already unpacked.
    fn install(&mut self, id: &LanguageServerId) -> Result<String> {
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let name = format!("routeros-lsp-{}.tar.gz", release.version.trim_start_matches('v'));
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == name)
            .ok_or_else(|| format!("release {} has no asset named {name}", release.version))?;

        let directory = format!("routeros-lsp-{}", release.version);

        if self.installed.as_deref() != Some(&release.version)
            || !std::path::Path::new(&format!("{directory}/{ENTRY}")).exists()
        {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &asset.download_url,
                &directory,
                zed::DownloadedFileType::GzipTar,
            )?;

            // Older versions are useless once a new one is unpacked.
            if let Ok(entries) = std::fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if name.starts_with("routeros-lsp-") && name != directory {
                        std::fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }

            self.installed = Some(release.version.clone());
        }

        Ok(format!("{directory}/{ENTRY}"))
    }
}

impl zed::Extension for RouterOsExtension {
    fn new() -> Self {
        Self { installed: None }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // A server already on PATH wins, so a contributor can point the
        // extension at a local build without touching the manifest.
        if let Some(path) = worktree.which(BINARY) {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".into()],
                env: vec![],
            });
        }

        let entry = self.install(id)?;

        // Zed runs the command from the worktree, not from the extension's own
        // directory, so a relative path would resolve against the user's
        // project and fail.
        let absolute = std::env::current_dir()
            .map_err(|e| format!("cannot resolve the extension directory: {e}"))?
            .join(&entry);

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![absolute.to_string_lossy().into_owned(), "--stdio".into()],
            env: vec![],
        })
    }
}

zed::register_extension!(RouterOsExtension);
