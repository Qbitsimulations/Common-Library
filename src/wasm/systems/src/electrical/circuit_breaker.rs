use uom::si::{
    electric_charge::ampere_hour, electric_current::ampere, electric_potential::volt,
    electrical_resistance::ohm, f64::*, time::second,
};

pub struct CircuitBreaker {
    identifier: String,
    state: bool,
    current_in: ElectricCurrent,
    current_out: ElectricCurrent,
}
impl CircuitBreaker{
    fn pub fn new(identifier: String, state: bool, current_in: ElectricCurrent, current_out: ElectricCurrent) -> Self {
        CircuitBreaker {
            identifier: identifier,
            state: state,
            current_in: current_in,
            current_out: current_out,
        }
    }
    fn pub fn get_state(&self) -> bool {
        self.state
    }
    fn pub fn update_state(&mut self, state: bool) {
        self.state = state;
        if(self.state == true){
            self.current_out = self.current_in;
        }
        else{
            self.current_out = ElectricCurrent::new::<ampere>(0.0);
        }
    }
}