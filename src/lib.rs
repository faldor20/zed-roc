use std::fs;
use zed_extension_api::{self as zed, Result};

struct RocExtension {}

impl zed::Extension for RocExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which("roc_language_server").ok_or_else(|| {
            "roc_language_server should be installed when you install roc. Check roc is installed correctly".to_string()
        })?;

        Ok(zed::Command {
            command: path,
            args: Vec::new(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(RocExtension);
