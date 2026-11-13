use crate::secrecy::Level3;

pub struct Patient<L: Level3> {
    secrecy_level: L,
    health_information: usize,
}

impl<L: Level3> Patient<L> {
    pub fn get_health(&self, secrecy_level: L) -> usize {
        self.health_information
    }
}

pub fn patient_data<L: Level3>(patient: Patient<L>) {}
