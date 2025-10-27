use {
    crate::{cache::AdapterCache, handlers::WhatDo},
    freedesktop_desktop_entry::{default_paths, get_languages_from_env, DesktopEntry, Iter},
    indexmap::IndexMap,
    is_executable::IsExecutable,
    std::{collections::HashSet, env, sync::Arc},
};

#[derive(Clone, Default)]
pub(crate) struct AdapterStore {
    pub storage: IndexMap<String, WhatDo>,
    locales: Vec<String>,
    entries: Vec<DesktopEntry>,
}

impl AdapterStore {
    pub fn new() -> Self {
        let storage = Default::default();
        let locales = get_languages_from_env();
        let entries = Iter::new(default_paths())
            .entries(Some(&locales))
            .collect::<Vec<_>>();
        AdapterStore {
            storage,
            locales,
            entries,
        }
    }
    pub fn load_desktop(&mut self) {
        for entry in &self.entries {
            let name = entry.name(&self.locales).unwrap_or_default();
            let selectable = format!("{} ({})", name, entry.id());
            let entry_type = entry.type_();
            let type_check = entry_type.is_none() || entry_type == Some("Application");
            if !entry.hidden()
                && type_check
                && !entry.no_display()
                && let Ok(entry_whatdo_inner) = entry.parse_exec()
            {
                let entry_whatdo = if entry.terminal() {
                    WhatDo::XdgTerminal(entry_whatdo_inner)
                } else {
                    WhatDo::XdgApplication(entry_whatdo_inner)
                };
                self.storage.insert(selectable, entry_whatdo);
            }
        }
    }
    pub fn load_path(&mut self) -> anyhow::Result<()> {
        let mut dedup = HashSet::new();
        let path_var = env::var("PATH").unwrap_or_default();
        let paths = env::split_paths(&path_var);
        for path in paths {
            if let Ok(mut dir_entries) = path.read_dir() {
                while let Some(Ok(entry)) = dir_entries.next() {
                    let path = entry.path();
                    let filename = entry.file_name();
                    if path.is_file() && path.is_executable() {
                        let filename_string = match filename.into_string() {
                            Ok(filename) => filename,
                            Err(os) => os.to_string_lossy().to_string(),
                        };
                        let path_string = path.clone().into_os_string().into_string();
                        if let Ok(path_string) = path_string {
                            let full_string =
                                format!("{} (Path: {})", filename_string, path_string);
                            if !dedup.contains(&filename_string) {
                                dedup.insert(filename_string.clone());
                                let entry_path = Arc::new(path.clone());
                                let entry_whatdo = WhatDo::PathExec(entry_path.as_path().into());
                                self.storage.insert(full_string, entry_whatdo);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn configure(&mut self, config: &AdapterCache) {
        config.transfer(&mut self.storage);
    }
    pub fn keys(&self) -> Vec<String> {
        self.storage.keys().cloned().collect()
    }
    pub fn input(&self) -> String {
        self.keys().join("\n")
    }
}
