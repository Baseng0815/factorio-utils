use prototypes::{Database, MachineId};

use crate::error::{Error, Result};

pub fn footprint_for(db: &Database, machine: &MachineId) -> Result<(u32, u32)> {
    let m = db
        .machines
        .get(machine)
        .ok_or_else(|| Error::UnknownMachine(machine.clone()))?;
    Ok((m.tile_width, m.tile_height))
}
