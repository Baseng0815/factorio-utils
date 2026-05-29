use prototypes::{MachineId, MachineKind};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no satisfying layout within {width}x{height}")]
    Unsat { width: u32, height: u32 },
    #[error("solver timed out or returned unknown")]
    Timeout,
    #[error("unknown machine `{0}` (not in database)")]
    UnknownMachine(MachineId),
    #[error("unsupported machine kind `{kind}` for machine `{machine}`")]
    UnsupportedMachineKind { kind: MachineKind, machine: MachineId },
    #[error("z3 model evaluation produced no value (internal)")]
    ModelEval,
}

pub type Result<T> = std::result::Result<T, Error>;
