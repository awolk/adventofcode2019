fn fuel_for_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn part1(input: &str) {
    let fuel_required_sum: i32 =
        input
            .lines()
            .map(|line| line.parse().expect("expected integer"))
            .map(fuel_for_mass)
            .sum();

    println!("Fuel required: {}", fuel_required_sum);
}

fn total_fuel_for_mass(mass: i32) -> i32 {
    let fuel_for_mass = fuel_for_mass(mass);
    if fuel_for_mass < 0 {
        0
    } else {
        fuel_for_mass + total_fuel_for_mass(fuel_for_mass)
    }
}

fn part2(input: &str) {
    let fuel_required_sum: i32 =
        input
            .lines()
            .map(|line| line.parse().expect("expected integer"))
            .map(total_fuel_for_mass)
            .sum();

    println!("Total fuel required: {}", fuel_required_sum);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input)
}
