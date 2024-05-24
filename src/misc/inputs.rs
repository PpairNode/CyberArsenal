use anyhow::Result;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;


pub fn write_co_clipboard(command: &str) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| anyhow::anyhow!("Cannot create context for clipboard, error={}", e))?;
    ctx.set_contents(command.to_string())
        .map_err(|e| anyhow::anyhow!("Cannot set content of clipboard, error={}", e))?;

    Ok(())
}

pub fn replace_first_last_quote(text: &str) -> String {
    let mut text = text.to_string();
    if text.starts_with("\"") {
        text.remove(0);
    }
    if text.ends_with("\"") {
        text.pop();
    }
    text
}