use std::io::{self, prelude::*};

fn fuel_required(mass: i32) -> i32 {
    (mass / 3) - 2
}

#[allow(dead_code)]
fn part1() {
    let fuel_required_sum: i32 =
        io::stdin()
            .lock()
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.parse().expect("expected integer"))
            .map(fuel_required)
            .sum();

    println!("Fuel required: {}", fuel_required_sum);
}

fn total_fuel_required(mass: i32) -> i32 {
    let fuel_for_mass = fuel_required(mass);
    if fuel_for_mass < 0 {
        0
    } else {
        return fuel_for_mass + total_fuel_required(fuel_for_mass)
    }
}

fn part2() {
    let fuel_required_sum: i32 =
        io::stdin()
            .lock()
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.parse().expect("expected integer"))
            .map(total_fuel_required)
            .sum();

    println!("Total fuel required: {}", fuel_required_sum);
}

fn main() {
    part2()
}
