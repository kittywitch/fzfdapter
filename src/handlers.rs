use {
    crate::config::AdapterConfig, anyhow::anyhow, fork::{daemon, Fork}, std::{
        mem,
        path::Path,
        process::{Command, Stdio},
        sync::Arc,
    }
};

#[derive(Clone, Debug)]
pub(crate) enum WhatDo {
    XdgApplication(Vec<String>),
    XdgTerminal(Vec<String>),
    PathExec(Arc<Path>),
}

pub(crate) fn handle_xdg(exec: Vec<String>) -> anyhow::Result<()> {
    let args = exec.get(1..).unwrap_or_default();
    if let Ok(Fork::Child) = daemon(false, false) {
        let exec_run = Command::new(exec.first().ok_or(anyhow!(
            "Command not provided within the XDG desktop file correctly?"
        ))?)
        .args(args)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
        mem::forget(exec_run);
    }
    Ok(())
}

pub(crate) fn handle_terminal(config: &AdapterConfig, args: &[&str]) -> anyhow::Result<()> {
    let mut in_args = args.iter().map(|x| x.to_string()).collect();
    let mut term_args = config.terminal_args();
    term_args.append(&mut in_args);
    if let Ok(Fork::Child) = daemon(false, false) {
        let term_run = Command::new(
            config
                .terminal_bin()
                .ok_or(anyhow!("No defined or available terminal"))?,
        )
        .args(term_args)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
        mem::forget(term_run);
    }
    Ok(())
}
