use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "clipboard-cleaner")]
#[command(about = "Cross-platform clipboard cleaner that removes trailing suffixes and whitespace")]
#[command(version = "0.1.0")]
pub struct Config {
    #[arg(long, help = "Show what would be cleaned without modifying clipboard")]
    pub dry_run: bool,

    #[arg(
        short,
        long,
        help = "Enable verbose logging to see cleaning operations"
    )]
    pub verbose: bool,

    #[arg(
        long,
        help = "Custom regex pattern for cleaning (advanced users)",
        value_name = "PATTERN"
    )]
    pub pattern: Option<String>,

    #[arg(
        long,
        help = "Also remove trailing empty lines",
        default_value = "true"
    )]
    pub remove_empty_lines: bool,
}
