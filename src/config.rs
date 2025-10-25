use {
    crate::CFG_DIR,
    anyhow::anyhow,
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

fn fuzzy_exec() -> String {
    "fzf".to_string()
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct AdapterConfig {
    terminal_exec: Option<String>,
    #[serde(default = "fuzzy_exec")]
    fuzzy_exec: String,
}

impl AdapterConfig {
    const FNA: &str = "config.toml";
    pub fn load() -> anyhow::Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(CFG_DIR);
        match xdg_dirs.find_config_file(Self::FNA) {
            Some(p) => {
                let file = read_to_string(p)?;
                let ac: Self = toml::from_str(&file)?;
                Ok(ac)
            }
            None => {
                let ac: Self = Default::default();
                let p = xdg_dirs.place_config_file(Self::FNA)?;

                let mut f = File::create_new(p)?;
                let self_string = toml::to_string_pretty(&ac)?;
                f.write_all(self_string.as_bytes())?;
                Ok(ac)
            }
        }
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(CFG_DIR);
        let p = xdg_dirs.place_config_file(Self::FNA)?;

        let mut f = File::create(p)?;
        let self_string = toml::to_string_pretty(self)?;
        f.write_all(self_string.as_bytes())?;
        Ok(())
    }

    pub fn terminal_bin(&self) -> Option<String> {
        if let Some(exec) = &self.terminal_exec {
            exec.split_once(" ").map(|(e, _)| e.to_string())
        } else {
            env::var("TERMINAL").ok()
        }
    }

    pub fn terminal_args(&self) -> Vec<String> {
        if let Some(exec) = &self.terminal_exec {
            let mut ret = exec
                .split_whitespace()
                .map(|v| v.to_string())
                .collect::<Vec<_>>();
            ret.remove(0);
            ret
        } else {
            Vec::new()
        }
    }

    pub fn fuzzy_bin(&self) -> String {
        self.fuzzy_exec.split(" ").collect::<Vec<_>>()[0].to_string()
    }

    pub fn fuzzy_args(&self) -> Vec<String> {
        let mut ret = self
            .fuzzy_exec
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();
        ret.remove(0);
        ret
    }
}
