use tracing::info;

#[derive(Clone)]
pub struct PetabyteVfs {
    root: String,
    units: u64,
    sims_enabled: bool,
    sim_barriers_enabled: bool,
}

impl PetabyteVfs {
    pub fn new(root: String, units: u64) -> Self {
        Self {
            root,
            units,
            sims_enabled: true,
            sim_barriers_enabled: true,
        }
    }

    pub fn mount_root(&self) -> Result<(), String> {
        info!(
            "hs.aln.exe.util.stg.fs.vfs: mount root={} units={} scale=Petabyte",
            self.root, self.units
        );
        Ok(())
    }

    pub fn set_sims_enabled(&mut self, enabled: bool) {
        self.sims_enabled = enabled;
        info!("fs.vfs: sims_enabled={}", enabled);
    }

    pub fn set_sim_barriers_enabled(&mut self, enabled: bool) {
        self.sim_barriers_enabled = enabled;
        info!("fs.vfs: sim_barriers_enabled={}", enabled);
    }
}
