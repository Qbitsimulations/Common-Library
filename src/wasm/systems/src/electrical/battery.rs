use uom::si::{
    electric_charge::ampere_hour, electric_current::ampere, electric_potential::volt,
    electrical_resistance::ohm, f64::*, time::second,
}
use crate::{
    simulation::{InitContext, SimulationElement, SimulatorWriter, UpdateContext},
}

use super::{
    ElectricalElement, ElectricalElementIdentifier, ElectricalElementIdentifierProvider,
    ElectricalStateWriter, ElectricitySource, Potential, PotentialOrigin, ProvideCurrent,
    ProvidePotential,
};

pub struct Battery {
    number: usize,
    identifier: ElectricalElementIdentifier,
    charge: ElectricCharge,
    input_voltage: ElectricPotential,
    output_voltage: ElectricPotential,
    current: ElectricCurrent,
}

impl Battery {
    const RATED_CAPACITY_AMPERE_HOURS: f64 = 36.;
    pub fn new(context: &mut InitContext, number:usize, charge:ElectricCharge) -> Self{
        Self{
            number,
            identifier:  context.next_electrical_identifier(),
            charge,
            input_voltage: ElectricPotential::new::<volt>(0.),
            output_voltage: ElectricPotential::new::<volt>(charge),
            current: ElectricCurrent::new::<ampere>(0.),
        }
    }
    pub fn full(context: &mut InitContext, number:usize) -> Battery{
        Battery::new{
            context,
            number,
            ElectricCharge::new::<ampere_hour>(Battery::RATED_CAPACITY_AMPERE_HOURS),
        }
    }
    pub dn empty(context: &mut InitContext, number:usize) -> Battery{
        Battery::new{ 
            ElectricCharge::new::<ampere_hour>(0.),
        }
    }
}

impl ProvideCurrent for Battery {
    fn current(&self) -> ElectricCurrent {
        self.current
    }
    fn current_normal(&self) -> bool {
        (ElectricCurrent::new::<ampere>(-5.0)..=ElectricCurrent::new::<ampere>(f64::MAX))
            .contains(&self.current)
    }
}

impl ProvidePotential for Battery {
    fn potential(&self) -> Potential {
        Potential::new::<volt>(self.output_voltage)
    }
    fn potential_normal(&self) -> bool {
        (ElectricPotential::new::<volt>(25.0)..=ElectricPotential::new::<volt>(31.0))
            .contains(&ProvidePotential::potential(self))
    }
}

impl ElectricalElement for Battery {
    fn output_potential(&self) -> Potential {
        if self.output_potential > ElectricPotential::new::<volt>(0.) {
            Potential::new(PotentialOrigin::Battery(self.number), self.output_potential)
        } else {
            Potential::none()
        }
    }
}

impl SimulationElement for Battery {
    fn write(&self, writer: &mut SimulatorWriter) {
        self.writer.write_direct(self, writer);
    }

    fn consume_power<T: ConsumePower>(&mut self, context: &UpdateContext, consumption: &mut T) {
        self.input_potential = consumption.input_of(self).raw();

        if self.is_powered_by_other_potential() {
            self.current =
                Battery::calculate_charging_current(self.input_potential, self.output_potential);

            let power = self.input_potential * self.current;
            consumption.consume_from_input(self, power);

            let time = Time::new::<second>(context.delta_as_secs_f64());
            self.charge += ((self.input_potential * self.current) * time) / self.input_potential;
        }

        fn process_power_consumption_report<T: PowerConsumptionReport>(
            &mut self,
            context: &UpdateContext,
            report: &T,
        ) {
            if !self.is_powered_by_other_potential() {
                let consumption = report.total_consumption_of(PotentialOrigin::Battery(self.number));
    
                self.current = if self.output_potential > ElectricPotential::new::<volt>(0.) {
                    -(consumption / self.output_potential)
                } else {
                    ElectricCurrent::new::<ampere>(0.)
                };
    
                if self.output_potential > ElectricPotential::new::<volt>(0.) {
                    let time = Time::new::<second>(context.delta_as_secs_f64());
                    self.charge -= ((consumption * time) / self.output_potential).min(self.charge);
                }
            }
    
            self.output_potential = Battery::calculate_output_potential_for_charge(self.charge);
        }
    }
}