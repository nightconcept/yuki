use tokio::process::Command;

pub trait CommandExt {
    fn log_command(&self) -> &Self;
}

impl CommandExt for Command {
    fn log_command(&self) -> &Self {
        tracing::debug!("Executing command: {:?}", self);
        self
    }
}