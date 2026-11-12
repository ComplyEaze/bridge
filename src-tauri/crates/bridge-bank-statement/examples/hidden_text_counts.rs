//! Count a statement PDF's characters by class, counts only.
//!
//! For the operator's own machine, on the statements it holds: it prints how
//! many characters of the document were drawn, were in render mode 3, and fell
//! into each of the classes counted besides (#1656), and nothing else: never a
//! character, a word, a name, an amount or the account number. Its output is
//! safe to paste into an issue. The numbers decide where a threshold may sit
//! before any statement is refused for one of these classes.
//!
//! ```text
//! cargo run --example hidden_text_counts -- \
//!     --pdfium /path/to/libpdfium.dylib --pdf statement.pdf --password-file statement.password
//! ```
//!
//! The password is read only from a file that, on Unix, must not be readable by group or
//! others, and no argument is echoed.

use bridge_bank_statement::pdf::{engine, extract_pages};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;

const OPTIONS: [&str; 3] = ["pdfium", "pdf", "password-file"];

fn arguments() -> Result<(PathBuf, PathBuf, PathBuf), String> {
    let mut named: BTreeMap<String, String> = BTreeMap::new();
    let mut args = std::env::args().skip(1).enumerate();
    while let Some((position, key)) = args.next() {
        let Some(name) = key.strip_prefix("--").filter(|name| OPTIONS.contains(name)) else {
            return Err(format!(
                "argument {} is not one of --{}",
                position + 1,
                OPTIONS.join(", --")
            ));
        };
        let (_, value) = args
            .next()
            .ok_or_else(|| format!("argument {} needs a value", position + 1))?;
        named.insert(name.to_string(), value);
    }
    let mut take = |name: &str| {
        named
            .remove(name)
            .map(PathBuf::from)
            .ok_or_else(|| format!("--{name} is required"))
    };
    Ok((take("pdfium")?, take("pdf")?, take("password-file")?))
}

#[cfg(unix)]
fn private_to_owner(path: &std::path::Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mode = std::fs::metadata(path)
        .map_err(|_| "the password file could not be read".to_string())?
        .permissions()
        .mode();
    if mode & 0o077 == 0 {
        Ok(())
    } else {
        Err("the password file must not be readable by group or others".into())
    }
}

#[cfg(not(unix))]
fn private_to_owner(_path: &std::path::Path) -> Result<(), String> {
    Ok(())
}

fn run() -> Result<(), String> {
    let (library, pdf, password_file) = arguments()?;
    private_to_owner(&password_file)?;
    let password = std::fs::read_to_string(&password_file)
        .map_err(|_| "the password file could not be read".to_string())?;
    let bytes = std::fs::read(&pdf).map_err(|_| "the statement could not be read".to_string())?;
    let engine = engine(&library).map_err(|refusal| refusal.category.to_string())?;
    let text = extract_pages(engine, &bytes, password.trim_end_matches(['\r', '\n']))
        .map_err(|refusal| format!("refused: {}", refusal.category))?;
    let counts = text.class_counts();
    println!("drawn            {}", counts.drawn);
    println!("invisible_mode   {}", counts.invisible_mode);
    println!("clip_only_mode   {}", counts.clip_only_mode);
    println!("tiny             {}", counts.tiny);
    println!("off_page         {}", counts.off_page);
    println!("white_fill       {}", counts.white_fill);
    println!("transparent_fill {}", counts.transparent_fill);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
