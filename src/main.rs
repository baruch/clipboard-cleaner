use anyhow::Result;
use clap::Parser;
use clipboard_rs::{
    Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
};
use log::{error, info, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

mod cleaner;
mod config;

use cleaner::clean_content;
use config::Config;

struct ClipboardCleaner {
    ctx: ClipboardContext,
    config: Config,
    last_content: Arc<Mutex<String>>,
    running: Arc<AtomicBool>,
}

impl ClipboardCleaner {
    fn new(config: Config) -> Result<Self> {
        let ctx = ClipboardContext::new()
            .map_err(|e| anyhow::anyhow!("Failed to initialize clipboard context: {}", e))?;
        let last_content = Arc::new(Mutex::new(String::new()));
        let running = Arc::new(AtomicBool::new(true));

        Ok(ClipboardCleaner {
            ctx,
            config,
            last_content,
            running,
        })
    }
}

impl ClipboardHandler for ClipboardCleaner {
    fn on_clipboard_change(&mut self) {
        if !self.running.load(Ordering::Relaxed) {
            return;
        }

        match self.ctx.get_text() {
            Ok(content) => {
                let mut last_content = self.last_content.lock().unwrap();

                if content == *last_content {
                    return;
                }

                let cleaned = clean_content(&content);

                if self.config.verbose {
                    info!("Original: {:?}", content);
                    info!("Cleaned: {:?}", cleaned);
                }

                if cleaned != content {
                    if self.config.dry_run {
                        println!("Would clean: {:?} -> {:?}", content, cleaned);
                    } else {
                        match self.ctx.set_text(cleaned.clone()) {
                            Ok(_) => {
                                *last_content = cleaned.clone();
                                if self.config.verbose {
                                    info!("Clipboard cleaned successfully");
                                }
                            }
                            Err(e) => {
                                error!("Failed to set clipboard: {}", e);
                            }
                        }
                    }
                } else if self.config.verbose {
                    info!("No cleaning needed");
                }

                *last_content = content;
            }
            Err(e) => {
                warn!("Failed to get clipboard content: {}", e);
            }
        }
    }
}

fn main() -> Result<()> {
    let config = Config::parse();

    if config.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Warn)
            .init();
    }

    info!("Starting clipboard cleaner...");
    if config.dry_run {
        println!("Running in dry-run mode - no changes will be made to clipboard");
    }

    let cleaner = ClipboardCleaner::new(config)?;

    let mut watcher = ClipboardWatcherContext::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize clipboard watcher: {}", e))?;
    let _watcher_shutdown = watcher.add_handler(cleaner).get_shutdown_channel();

    info!("Clipboard cleaner is now running. Press Ctrl+C to stop.");
    watcher.start_watch();

    Ok(())
}
