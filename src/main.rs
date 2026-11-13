use std::marker::PhantomData;

use crate::secrecy::*;

mod process;
mod secrecy;

pub struct Height<L: Level2>(usize, PhantomData<L>);

pub struct Mass<L: Level1>(usize, PhantomData<L>);

pub struct HasDiabetes<L: Level0>(bool, PhantomData<L>);

fn bmi<L: Level1 + Level2>(w: Mass<L>, h: Height<L>, your_level: L) -> usize {
    w.0 * h.0
}

fn main() {
    let my_level = S0 {};
    let mass = Mass(50, PhantomData);
    let height = Height(150, PhantomData);

    println!("bmi {}", bmi(mass, height, my_level));
}
