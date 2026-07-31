use zed_extension_api::{self as zed, LanguageServerId, Result};

const PACKAGE: &str = "@damirmanyapov/routeros-lsp";
const BINARY: &str = "routeros-lsp";
const ENTRY: &str = "node_modules/@damirmanyapov/routeros-lsp/dist/server.js";

struct RouterOsExtension {
    installed_version: Option<String>,
}

impl zed::Extension for RouterOsExtension {
    fn new() -> Self {
        Self {
            installed_version: None,
        }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Prefer an existing install so contributors can point the extension at
        // a local checkout without touching the manifest.
        if let Some(path) = worktree.which(BINARY) {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".into()],
                env: vec![],
            });
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let latest = zed::npm_package_latest_version(PACKAGE)?;

        if self.installed_version.as_deref() != Some(&latest)
            || zed::npm_package_installed_version(PACKAGE)?.as_deref() != Some(&latest)
        {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::npm_install_package(PACKAGE, &latest)?;
            self.installed_version = Some(latest);
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![ENTRY.into(), "--stdio".into()],
            env: vec![],
        })
    }
}

zed::register_extension!(RouterOsExtension);
