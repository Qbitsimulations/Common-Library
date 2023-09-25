use std::time::Duration

use uom::si::{
    f64::*,
    thermodynamic_temperature::{degree_celsius, kelvin},
    time::second,
};

use super::{Turbine, TurbineState, TurbineSignal, ApuState};
//==============================================================================
// Allied Signal 131-9
//==============================================================================

//==============================================================================
// States Declaration
//==============================================================================

struct Stop{
    egt: ThermodynamicTemperature,
    OAT: ThermodynamicTemperature,
    ApuState: ApuState,
}

impl Stop{
    fn pub fn new() -> Self {
        Stop {
            OAT : ThermodynamicTemperature::new::<degree_celsius>(0.),
            egt: ThermodynamicTemperature::new::<degree_celsius>(OAT),
        }
    }
}

impl Turbine for Stop{
    fn update( mut self: Box<Self>, context: &UpdateContext, state: ApuState) -> Box<dyn Turbine> {
        self.egt = self.OAT;
        match state {
            Off => self,
            Start => Box::new(Starting::new(self.egt, self.OAT)),
        }
    }
}

struct Starting{
    since: Duration,
    egt: ThermodynamicTemperature,
    OAT: ThermodynamicTemperature,

}

impl Starting{
    fn pub fn new(egt: ThermodynamicTemperature, OAT: ThermodynamicTemperature) -> Self {
        Starting {
            since: Duration::from_secs(0),
            OAT : ThermodynamicTemperature::new::<degree_celsius>(0.),
            egt: ThermodynamicTemperature::new::<degree_celsius>(OAT),
        }
    }

    fn calculate_egt(&mut self, context: &UpdateContext) -> ThermodynamicTemperature {
        //Bare aproximation
        self.egt = egt;
        let egt = self.OAT
        if (self.since.as_secs() <= 10 && egt =! 100) {
            egt = self.OAT + (self.since.as_secs() as f64 * 0.5);
        } else if (egt =! 60) {
            egt = egt - 0.5;
        } else {
            egt = 60;
        }
        return egt;
    }
}

impl Turbine for Starting {
    fn update(mut self: Box<Self>, context: &UpdateContext, state: ApuState) -> Box<dyn Turbine> {
        self.since += context.delta();
        self.egt = self.calculate_egt(context);
        match state {
            Stop => Box::new(Stop::new(self.egt, self.OAT)),
            Start | Running => 
                if {self.egt.get<ThermodynamicTemperature>() >= 100 } => {
                    Box::new(Running::new(self.egt, self.OAT))
                } else => self,
        }
    }
    fn egt(&self) -> ThermodynamicTemperature {
        return self.egt;
    }
    fn state(&self) -> TurbineState {
        return TurbineState::Starting;
    }
}
struct Running{
    egt: ThermodynamicTemperature,
    OAT: ThermodynamicTemperature,
    N1: Ratio,
    N2: Ratio,
    FF: MassFlowRate,
    ITT: ThermodynamicTemperature,
    OilP: Pressure,
    OilT: ThermodynamicTe
}
//==============================================================================
// Had to stop here as an electrical grid should be implemented first
//==============================================================================
impl Running for Running {

}
struct ShutDown{
    since: Duration,
    egt: ThermodynamicTemperature,
    OAT: ThermodynamicTemperature,
}