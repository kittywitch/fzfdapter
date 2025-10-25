use {
    crate::{
        cache::AdapterCache,
        config::AdapterConfig,
        handlers::{handle_terminal, handle_xdg, WhatDo},
        store::AdapterStore,
    },
    anyhow::anyhow,
    clap::{Parser, ValueEnum},
    freedesktop_desktop_entry::{default_paths, get_languages_from_env, DesktopEntry, Iter},
    indexmap::IndexMap,
    is_executable::IsExecutable,
    rmp_serde::Serializer,
    serde::{Deserialize, Serialize},
    std::{
        collections::HashSet,
        env,
        fs::{read_to_string, File},
        io::{pipe, BufReader, Write},
        mem,
        os::unix::ffi::OsStrExt,
        path::Path,
        process::{Command, Stdio},
        sync::Arc,
    },
};

mod cache;
mod config;
mod handlers;
mod store;

const CFG_DIR: &str = "fzfdapter";

#[derive(Clone, PartialEq, ValueEnum)]
enum Mode {
    All,
    Desktop,
    Path,
}

#[derive(Parser, Clone)]
#[command(name = "fzfdapter")]
#[command(about = "A PATH and desktop file executor that uses fzf/skim/...", long_about = None)]
#[command(arg_required_else_help = true)]
struct Args {
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',', num_args = 1.., help = "How to source programs")]
    mode: Vec<Mode>,
}

fn main() -> anyhow::Result<()> {
    let mut aca = AdapterCache::load()?;
    let aco = AdapterConfig::load()?;

    let args = Args::parse();
    let mut store = AdapterStore::new();
    if args.mode.contains(&Mode::All) || args.mode.contains(&Mode::Desktop) {
        store.load_desktop();
    }
    if args.mode.contains(&Mode::All) || args.mode.contains(&Mode::Path) {
        store.load_path()?;
    }
    store.configure(&aca);

    if !args.mode.is_empty() {
        let (reader, mut writer) = pipe()?;

        let fuzz_command = aco.fuzzy_bin();
        let fuzz_args = aco.fuzzy_args();
        let fuzzy = Command::new(fuzz_command)
            .args(&fuzz_args)
            .stdin(reader)
            .stdout(Stdio::piped())
            .spawn()?;

        let fzf_input = store.input();
        writer.write_all(fzf_input.as_bytes())?;
        writer.flush()?;
        drop(writer);

        let fuzz = fuzzy.wait_with_output()?;

        let fuzz = String::from_utf8(fuzz.stdout)?;
        let fuzz = fuzz.strip_suffix("\n").unwrap_or(&fuzz);

        if let Some(whatdo) = store.storage.get(fuzz) {
            aca.add(fuzz)?;
            match whatdo {
                WhatDo::XdgApplication(exec) => {
                    handle_xdg(exec.clone())?;
                }
                WhatDo::XdgTerminal(exec) => {
                    let args: Vec<_> = exec.iter().map(|x| x.as_str()).collect();
                    handle_terminal(&aco, &args)?;
                }
                WhatDo::PathExec(path) => {
                    let path_arg = match path.to_path_buf().into_os_string().into_string() {
                        Ok(path) => path,
                        Err(os) => os.to_string_lossy().to_string(),
                    };
                    let args = [path_arg.as_str()];
                    handle_terminal(&aco, &args)?;
                }
            }
        }
    }

    Ok(())
}
