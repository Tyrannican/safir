mod cfg;
mod cli;
mod safir;
mod utils;

use cli::*;

use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let store_dir = utils::create_safir_directory().await?;
    let safir_cfg = &store_dir.join("safir.cfg");
    let mut cfg = utils::load_safir_config(&safir_cfg).await?;

    let safir_state = utils::is_safir_running(cfg.memcache_pid);
    let safir_mem = safir::SafirMemcache::new(safir_state);

    match &cli.command {
        Commands::Add(args) => safir_mem.add_entry(&args.key, &args.value).await?,
        Commands::Get(args) => {
            if let Some(key) = &args.key {
                safir_mem.get_string(key).await?;
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

            if let Some(pid) = cfg.memcache_pid {
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
            cfg.memcache_pid = Some(pid);
            cfg.write(&safir_cfg).await?;
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

            let pid = match cfg.memcache_pid {
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
                cfg.memcache_pid = None;
                cfg.write(&safir_cfg).await?;
                println!("Stopping Safir memcache service!");
            }
        }
        Commands::Dump(args) => {
            if let Err(e) = safir_mem.dump_store(&args.path).await {
                eprintln!("unable to dump Safir memcache service: {}", e);
            }
        }
    }

    Ok(())
}
