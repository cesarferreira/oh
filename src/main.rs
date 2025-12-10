use std::{fs, path::Path, process::Command};

use anyhow::{Context, Result, anyhow, bail};
use dialoguer::{FuzzySelect, theme::ColorfulTheme};

fn main() -> Result<()> {
    let apps = build_app_list()?;
    if apps.is_empty() {
        bail!("No applications found in /Applications");
    }

    match prompt_user(&apps)? {
        Some(app) => {
            println!();
            println!("==> Opening {app}");
            open_app(&app)?;
        }
        None => println!("No app selected, aborting."),
    }

    Ok(())
}

fn build_app_list() -> Result<Vec<String>> {
    let mut apps = Vec::new();

    apps.extend(list_apps_in("/Applications")?);
    apps.push("finder".to_string());

    apps.sort_unstable();
    apps.dedup();

    Ok(apps)
}

fn list_apps_in(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let mut apps = Vec::new();
    let base = path.as_ref();

    let entries = fs::read_dir(base)
        .with_context(|| format!("reading applications from {}", base.display()))?;

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name();
            let name = name
                .to_str()
                .ok_or_else(|| anyhow!("Unexpected non-UTF-8 filename"))?;

            // Skip hidden apps (starting with ".")
            if name.starts_with('.') {
                continue;
            }

            let name = name.strip_suffix(".app").unwrap_or(name).to_string();
            apps.push(name);
        }
    }

    Ok(apps)
}

fn prompt_user(apps: &[String]) -> Result<Option<String>> {
    let theme = ColorfulTheme::default();
    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt("What app do you want to Open Here?")
        .max_length(10)
        .items(apps)
        .interact_opt()?;

    Ok(selection.map(|idx| apps[idx].clone()))
}

fn open_app(app: &str) -> Result<()> {
    let status = Command::new("open")
        .arg("-a")
        .arg(app)
        .arg(".")
        .status()
        .with_context(|| format!("launching {app}"))?;

    if !status.success() {
        bail!("Command 'open' returned non-zero status");
    }

    Ok(())
}
