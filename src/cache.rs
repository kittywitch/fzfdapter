use {
    crate::{handlers::WhatDo, CFG_DIR},
    indexmap::IndexMap,
    rmp_serde::Serializer,
    serde::{Deserialize, Serialize},
    std::{
        fs::File,
        io::{BufReader, Write},
    },
};
// honestly it's probably inefficient to store a usize for how many times a program has been opened
// but maybe you want to open it usize::MAX times???
#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct AdapterCache(IndexMap<String, usize>);

impl AdapterCache {
    const FNA: &str = "cache.msgpack";
    pub fn load() -> anyhow::Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(CFG_DIR);
        match xdg_dirs.find_cache_file(Self::FNA) {
            Some(p) => {
                let f = File::open(p)?;
                let reader = BufReader::new(f);
                let mut ac: Self = rmp_serde::from_read(reader)?;
                ac.sort();
                Ok(ac)
            }
            None => {
                let p = xdg_dirs.place_cache_file(Self::FNA)?;

                let mut f = File::create_new(p)?;
                let ac: Self = Default::default();
                let mut ac_buf = Vec::new();
                ac.serialize(&mut Serializer::new(&mut ac_buf)).unwrap();
                f.write_all(&ac_buf)?;
                Ok(ac)
            }
        }
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(CFG_DIR);
        let p = xdg_dirs.place_cache_file(Self::FNA)?;

        let mut f = File::create(p)?;
        let mut ac_buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut ac_buf)).unwrap();
        f.write_all(&ac_buf)?;
        Ok(())
    }
    pub fn sort(&mut self) {
        self.0.sort_by(|_ak, av, _bk, bv| bv.cmp(av))
    }
    pub fn add(&mut self, name: &str) -> anyhow::Result<()> {
        *self.0.entry(name.to_string()).or_insert(0) += 1;
        self.sort();
        self.save()?;
        Ok(())
    }
    pub fn transfer(&self, recipient: &mut IndexMap<String, WhatDo>) {
        for (idx, (key, _opens)) in self.0.iter().enumerate() {
            if let Some(idx_recipient) = recipient.get_index_of(key) {
                recipient.swap_indices(idx_recipient, idx);
            }
        }
    }
}
