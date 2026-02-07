use crate::aln::fs_vfs::PetabyteVfs;
use crate::aln::quantum_image_term::QuantumImageTerminal;
use crate::aln::amazon_vm::AmazonVmBridge;
use crate::aln::merchant_terminal::MerchantTerminal;
use crate::aln::connection_framework::ConnectionFramework;
use tracing::{info, warn};
use uuid::Uuid;

pub struct SyncController {
    sync_id: String,
    vfs: PetabyteVfs,
    terminal: QuantumImageTerminal,
    vm_bridge: AmazonVmBridge,
    merchant: MerchantTerminal,
    framework: ConnectionFramework,
}

impl SyncController {
    pub fn new(
        sync_id: String,
        vfs: PetabyteVfs,
        terminal: QuantumImageTerminal,
        vm_bridge: AmazonVmBridge,
        merchant: MerchantTerminal,
        framework: ConnectionFramework,
    ) -> Self {
        Self {
            sync_id,
            vfs,
            terminal,
            vm_bridge,
            merchant,
            framework,
        }
    }

    pub fn remove_sims_and_barriers(&mut self) {
        info!("aln.cmd.actions: remove.sims & sim.barriers");
        self.vfs.set_sims_enabled(false);
        self.vfs.set_sim_barriers_enabled(false);
    }

    pub fn load_dependencies(&mut self, branch: &str) {
        info!("~aln; load-dependencies {}", branch);
        self.vm_bridge.load_branch(branch);
    }

    pub async fn bootstrap(&mut self) -> Result<(), String> {
        info!("bootstrap: sync_id={} session={}", self.sync_id, Uuid::new_v4());

        self.vfs.mount_root().map_err(|e| format!("vfs mount: {e}"))?;
        self.terminal.deploy_image().map_err(|e| format!("image deploy: {e}"))?;

        self.vm_bridge.ensure_static_vm().await.map_err(|e| format!("vm bridge: {e}"))?;

        let merchant_id = self.merchant.resolve_id().await
            .map_err(|e| format!("merchant id: {e}"))?;
        info!("merchant terminal resolved: {}", merchant_id);

        let conn = self.framework.stabilize_connection().await
            .map_err(|e| format!("framework connect: {e}"))?;

        if !conn.stable {
            warn!("connection not fully stable after retries");
        }

        Ok(())
    }
}
