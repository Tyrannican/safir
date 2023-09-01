mod cli;

use cli::*;
use safir_core::{utils, Safir, SafirEngineType};

use anyhow::Result;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut safir_mem = Safir::new(SafirEngineType::Memcache).await?;

    match &cli.command {
        Commands::Add(args) => {
            safir_mem
                .add_entry(args.key.to_owned(), args.value.to_owned())
                .await?
        }
        Commands::Get(args) => {
            if let Some(key) = &args.key {
                safir_mem.get_entry(key.to_string()).await?;
            } else {
                utils::print_header();
                utils::print_output("A key is required for memcache GET command!");
            }
        }
        Commands::Rm(args) => {
            safir_mem.remove_entry(args.key.clone()).await?;
        }
        Commands::Alias(args) => {
            safir_mem.set_commands("alias", &args.keys).await;
        }
        Commands::Export(args) => {
            safir_mem.set_commands("export", &args.keys).await;
        }
        Commands::Clear => {
            safir_mem.clear_entries().await?;
        }
        Commands::Start => {
            if !utils::check_rubin_installed() {
                eprintln!(
                    "The Rubin binary must be installed to use this feature, please install it via cargo using `cargo install rubin-cli`"
                );
                return Ok(());
            }

            let config = &mut safir_mem.config;
            if let Some(pid) = config.memcache_pid {
                println!(
                    "Safir memcache service is already running on 127.0.0.1:9876 - PID {}",
                    pid
                );

                return Ok(());
            }

            let child = Command::new("rubin")
                .args(["server"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .stdin(Stdio::null())
                .spawn()
                .expect("unable to spawn child process");

            let pid = child.id();
            config.memcache_pid = Some(pid);
            config.write().await?;
            println!(
                "Safir memcache service started at 127.0.0.1:9876 - PID {}",
                pid
            );
        }
        Commands::Stop => {
            if !utils::check_rubin_installed() {
                eprintln!("The Rubin binary must be installed to use this feature, please install it via cargo using `cargo install rubin-cli`");
                return Ok(());
            }

            let config = &mut safir_mem.config;
            let pid = match config.memcache_pid {
                Some(pid) => pid,
                None => {
                    println!("Safir memcache service does not seem to be running.");
                    return Ok(());
                }
            };

            if let Err(err) = utils::kill_process(pid).await {
                eprintln!(
                    "Safir memcache service failed to stop, manual removal may be necessary - {}",
                    err
                );
            } else {
                config.memcache_pid = None;
                config.write().await?;
                println!("Stopping Safir memcache service!");
            }
        }
        Commands::Dump(args) => {
            let inner = safir_mem.as_safir_memcache();
            if let Err(e) = inner.dump_store(&args.path).await {
                eprintln!("unable to dump Safir memcache service: {}", e);
            }
        }
    }

    Ok(())
}
