use uom::si::{
    pressure::psi, f64::*
}

use crate::{
    simulation::{InitContext, SimulationElement, SimulatorWriter, UpdateContext},
}

pub struct YawDampnerComputer {
    identifier: String,
    state: bool,
    rudder_angle: Angle,
    max_authority: Angle,
}
impl YawDampnerComputer