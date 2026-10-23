use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree, settings::LspSettings};

const NOT_FOUND: &str = "Statac isn't installed, or Zed can't find it. Install it as \
                         statac.dev/install says, or give its path as `lsp.statac.binary.path` \
                         in Zed's settings.";

struct Statac;

impl zed::Extension for Statac {
    fn new() -> Self {
        Statac
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.binary);
        let given = binary.as_ref().and_then(|binary| binary.path.clone());
        let command = given
            .or_else(|| worktree.which("statac"))
            .ok_or(NOT_FOUND)?;
        let args = (binary.as_ref())
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_else(|| vec!["lsp".to_owned()]);
        Ok(zed::Command {
            command,
            args,
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(Statac);
