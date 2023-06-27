mod cfg;
mod cli;
mod safir;
mod utils;

use cfg::SafirConfig;
use cli::*;

use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let store_dir = utils::create_safir_directory().await?;
    let safir_cfg = &store_dir.join("safir.cfg");

    let mut cfg = if utils::path_exists(&safir_cfg).await {
        SafirConfig::load(&safir_cfg).await?
    } else {
        SafirConfig::new()
    };

    // Should probably only initialise when not using memcache but meh...
    // Easier this way
    let mut safir = safir::Safir::init(&store_dir).await?;
    let safir_mem = safir::SafirMemcache::new();

    match &cli.command {
        Commands::Add(args) => {
            if cfg.memcache_flag {
                safir_mem.add_entry(&args.key, &args.value).await?
            } else {
                safir
                    .add_entry(args.key.clone(), args.value.clone())
                    .await?;
            }
        }
        Commands::Get(args) => {
            if cfg.memcache_flag {
                if let Some(key) = &args.key {
                    safir_mem.get_string(&key).await?;
                } else {
                    utils::print_header();
                    utils::print_output("A key is required for memcache GET command!");
                }
            } else {
                if let Some(key) = &args.key {
                    safir.get_entry(key.clone())?;
                } else {
                    safir.display_all();
                }
            }
        }
        Commands::Rm(args) => {
            if cfg.memcache_flag {
                safir_mem.remove_entry(args.key.clone()).await?;
            } else {
                safir.remove_entry(args.key.clone()).await?;
            }
        }
        Commands::Alias(args) => {
            if cfg.memcache_flag {
                safir_mem.set_commands("alias", &args.keys).await;
            } else {
                safir.set_commands("alias", &args.keys);
            }
        }
        Commands::Export(args) => {
            if cfg.memcache_flag {
                safir_mem.set_commands("export", &args.keys).await;
            } else {
                safir.set_commands("export", &args.keys);
            }
        }
        Commands::Clear => {
            if cfg.memcache_flag {
                safir_mem.clear_entries().await?;
            } else {
                safir.clear_entries().await?;
            }
        }
        Commands::Purge => {
            if !cfg.memcache_flag {
                safir.purge();
            }
        }
        Commands::Mem(args) => match args {
            MemArgs::Start => {
                if !utils::check_rubin_installed() {
                    eprintln!(
                        "The Rubin binary must be installed to use this feature, please install it via cargo using `cargo install rubin-cli`"
                    );
                    return Ok(());
                }

                if let Some(pid) = cfg.memcache_pid {
                    println!(
                        "Memcache server is already running on 127.0.0.1:9876 - PID {}",
                        pid
                    );

                    return Ok(());
                }

                let child = Command::new("rubin")
                    .args(&["server"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .stdin(Stdio::null())
                    .spawn()
                    .expect("unable to spawn child process");

                let pid = child.id();
                cfg = cfg.pid(Some(pid)).set_memcache(true);
                cfg.write(&safir_cfg).await?;
                println!("Memcache server started at 127.0.0.1:9876 - PID {}", pid);
            }
            MemArgs::Stop => {
                if !utils::check_rubin_installed() {
                    eprintln!("The Rubin binary must be installed to use this feature, please install it via cargo using `cargo install rubin-cli`");
                    return Ok(());
                }

                let pid = match cfg.memcache_pid {
                    Some(pid) => pid,
                    None => {
                        println!("Memcache does not seem to be running.");
                        return Ok(());
                    }
                };

                if let Err(err) = utils::kill_process(pid).await {
                    eprintln!(
                        "memcache server failed to stop, manual removal may be necessary - {}",
                        err
                    );
                } else {
                    cfg = cfg.pid(None).set_memcache(false);
                    cfg.write(&safir_cfg).await?;
                    println!("Stopping memcache server!");
                }
            }
        },
    }
    Ok(())
}
