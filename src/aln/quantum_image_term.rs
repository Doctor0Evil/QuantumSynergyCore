use tracing::info;

pub struct QuantumImageTerminal {
    os: String,
    destination: String,
}

impl QuantumImageTerminal {
    pub fn new(os: String, destination: String) -> Self {
        Self { os, destination }
    }

    pub fn deploy_image(&self) -> Result<(), String> {
        info!(
            "term.aln.cmd.cfgsys type=image os={} dest={}",
            self.os, self.destination
        );
        Ok(())
    }
}
