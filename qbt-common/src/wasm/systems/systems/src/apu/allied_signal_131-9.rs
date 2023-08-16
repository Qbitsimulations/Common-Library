use std::time::Duration

use uom::si::{
    f64::*,
    thermodynamic_temperature::{degree_celsius, kelvin},
    time::second,
};

use super::{Turbine, TurbineState};
struct Starting{
    since: Duration,
    egt: ThermodynamicTemperature,
    OAT: ThermodynamicTemperature,
}
impl Starting {
    fn new(egt: ThermodynamicTemperature) -> Starting{
        Starting {
            since: Duration::from_secs(0),
            egt,
        }
    }

    fn  
}
impl Turbine 