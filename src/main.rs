use clap::{Parser, Subcommand};
use cmd_lib::{run_cmd, run_fun};
use dirs::home_dir;
use std::{io::Error, path::PathBuf};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Previous,
    Next,
    Select { theme: String },
    List,
}

#[cmd_lib::main]
fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let root_dir = home_dir().unwrap().join(".theme");
    match &cli.command {
        Commands::Previous => apply_theme_at_offset(&root_dir, -1),
        Commands::Next => apply_theme_at_offset(&root_dir, 1),
        Commands::Select { theme } => apply_theme(&root_dir, theme),
        Commands::List => print_themes(&root_dir),
    }?;
    Ok(())
}

fn get_all_themes(root_dir: &PathBuf) -> Result<Vec<String>, Error> {
    let output = std::fs::read_dir(root_dir.join("options"))?
        .filter_map(|e| Some(e.ok()?.file_name().to_str()?.to_string()))
        .collect();
    Ok(output)
}

fn print_themes(root_dir: &PathBuf) -> Result<(), Error> {
    println!(
        "{}",
        get_all_themes(root_dir)?
            .iter()
            .map(|s| s.replace("\n", "\\n"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}

fn get_current_theme(root_dir: &PathBuf) -> Result<String, Error> {
    let current_dir = root_dir.join("current");
    run_fun!(getfattr --only-values --name user.theme $current_dir 2>/dev/null)
}

fn apply_theme(root_dir: &PathBuf, theme: &str) -> Result<(), Error> {
    let source = root_dir.join("options").join(theme);
    let target = root_dir.join("current");
    run_cmd!(
        cp --recursive --no-target-directory --no-preserve="mode" --dereference $source $target;
        setfattr --name user.theme --value $theme $target;
    )?;
    println!("{}", theme);
    Ok(())
}

fn apply_theme_at_offset(root_dir: &PathBuf, offset: isize) -> Result<(), Error> {
    let current = get_current_theme(&root_dir).unwrap_or(String::from(""));
    let all_themes = get_all_themes(&root_dir)?;
    let position = all_themes.iter().position(|s| s == &current).unwrap_or(0);
    let positive_offset = offset as usize % all_themes.len();
    let new_position = (position + positive_offset) % all_themes.len();
    apply_theme(&root_dir, &all_themes[new_position])?;
    Ok(())
}
