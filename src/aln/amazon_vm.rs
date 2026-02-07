use serde::Deserialize;
use tracing::{info, warn};
use tokio::time::{sleep, Duration};
use std::fs;

#[derive(Debug, Deserialize)]
struct VmConfig {
    vm_profile: String,
    provider: String,
    s3_bucket: String,
    pak: PakConfig,
    paths: PathsConfig,
    storage: StorageConfig,
    flags: FlagsConfig,
}

#[derive(Debug, Deserialize)]
struct PakConfig {
    install: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PathsConfig {
    homedir: String,
    system_root: String,
}

#[derive(Debug, Deserialize)]
struct StorageConfig {
    mode_units: u64,
    scale: String,
}

#[derive(Debug, Deserialize)]
struct FlagsConfig {
    sims_enabled: bool,
    sim_barriers_enabled: bool,
}

pub struct AmazonVmBridge {
    cfg: VmConfig,
}

impl AmazonVmBridge {
    pub fn from_config(path: &str) -> Result<Self, String> {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("read config {path}: {e}"))?;
        let cfg: VmConfig = serde_yaml::from_str(&raw)
            .map_err(|e| format!("parse config {path}: {e}"))?;
        Ok(Self { cfg })
    }

    pub fn load_branch(&self, branch: &str) {
        info!(
            "aln.cmd.sync: deploy branch={} vm_profile={} provider={}",
            branch, self.cfg.vm_profile, self.cfg.provider
        );
    }

    pub async fn ensure_static_vm(&self) -> Result<(), String> {
        info!(
            "aln_static_deployment: enabled=true framework=aln_framework.v1.0.0.5 homedir={} system_root={}",
            self.cfg.paths.homedir, self.cfg.paths.system_root
        );
        info!(
            "pak.install.aln.ppl.s3amazon.aws: bucket={} packages={:?}",
            self.cfg.s3_bucket, self.cfg.pak.install
        );

        sleep(Duration::from_millis(200)).await;

        if self.cfg.flags.sims_enabled || self.cfg.flags.sim_barriers_enabled {
            warn!("vm flags indicate sims/sim_barriers enabled; ALN controller will override.");
        }

        Ok(())
    }
}
