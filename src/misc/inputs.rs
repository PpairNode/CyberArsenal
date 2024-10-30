use anyhow::Result;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;


pub struct IntelligentStringBuilder {
    s: String,
    delete_first_quote: bool,
    delete_last_quote: bool,
    replace_backslash_quote_with_quote: bool
}

impl IntelligentStringBuilder {
    pub fn new(s: String) -> Self {
        IntelligentStringBuilder { s, delete_first_quote: false, delete_last_quote: false, replace_backslash_quote_with_quote: false }
    }

    pub fn delete_first_quote(&mut self) -> &mut Self {
        self.delete_first_quote = true;
        self
    }

    pub fn delete_last_quote(&mut self) -> &mut Self {
        self.delete_last_quote = true;
        self
    }

    pub fn replace_backslash_quote_with_quote(&mut self) -> &mut Self {
        self.replace_backslash_quote_with_quote = true;
        self
    }

    pub fn build(&mut self) -> String {
        if self.delete_first_quote && self.s.starts_with("\"") {
            self.s.remove(0);
        }

        if self.delete_last_quote && self.s.ends_with("\"") {
            self.s.pop();
        }

        if self.replace_backslash_quote_with_quote {
            self.s = self.s.replace("\\\"", "\"");
        }

        self.s.clone()
    }
}

pub fn write_co_clipboard(command: &str) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| anyhow::anyhow!("Cannot create context for clipboard, error={}", e))?;
    ctx.set_contents(command.to_string())
        .map_err(|e| anyhow::anyhow!("Cannot set content of clipboard, error={}", e))?;

    Ok(())
}
