use anyhow::Result;

pub fn echo(text: &[String], omit_newline: bool) -> Result<()> {
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    Ok(())
}
