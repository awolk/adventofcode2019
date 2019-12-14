#![allow(incomplete_features)]
#![feature(const_generics)]
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

fn gcd(a: usize, b: usize) -> usize {
    // Euclid's algorithm
    if b == 0 {
        a
    } else {
        gcd(b, a - b * (a / b))
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm3(a: usize, b: usize, c: usize) -> usize {
    lcm(lcm(a, b), c)
}

#[derive(Clone)]
struct Axis<const N: usize> {
    positions: [i64; N],
    velocities: [i64; N],
}

impl<const N: usize> PartialEq for Axis<{ N }> {
    fn eq(&self, other: &Self) -> bool {
        (0..N).all(|i| {
            self.positions[i] == other.positions[i] && self.velocities[i] == other.velocities[i]
        })
    }
}

impl<const N: usize> Eq for Axis<{ N }> {}

impl<const N: usize> Hash for Axis<{ N }> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.positions.hash(state);
        self.velocities.hash(state);
    }
}

impl<const N: usize> Axis<{ N }> {
    fn step(&mut self) {
        // apply gravity to accelerate
        for i in 0..4 {
            for j in (i + 1)..4 {
                // apply gravity between points i and j
                let diff = self.positions[i] - self.positions[j];
                let diff = if diff == 0 { 0 } else { diff / diff.abs() }; // acceleration is either 1, -1, or 0
                self.velocities[i] -= diff;
                self.velocities[j] += diff;
            }
        }
        // move moons
        for i in 0..4 {
            self.positions[i] += self.velocities[i];
        }
    }

    fn find_cycle(&mut self) -> usize {
        let mut states = HashSet::new();
        states.insert(self.clone());
        let mut step = 0;
        loop {
            step += 1;
            self.step();
            let is_new = states.insert(self.clone());
            if !is_new {
                return step;
            }
        }
    }
}

#[derive(Clone)]
struct System<const N: usize> {
    x_axis: Axis<{ N }>,
    y_axis: Axis<{ N }>,
    z_axis: Axis<{ N }>,
}

fn zeros<const N: usize>() -> [i64; N] {
    unsafe { MaybeUninit::zeroed().assume_init() }
}

impl<const N: usize> System<{ N }> {
    fn step(&mut self) {
        self.x_axis.step();
        self.y_axis.step();
        self.z_axis.step();
    }

    fn moon_energy(&self, n: usize) -> i64 {
        let (x, y, z) = (
            self.x_axis.positions[n],
            self.y_axis.positions[n],
            self.z_axis.positions[n],
        );
        let (vx, vy, vz) = (
            self.x_axis.velocities[n],
            self.y_axis.velocities[n],
            self.z_axis.velocities[n],
        );
        let potential_energy = x.abs() + y.abs() + z.abs();
        let kinetic_energy = vx.abs() + vy.abs() + vz.abs();
        potential_energy * kinetic_energy
    }

    fn total_energy(&self) -> i64 {
        (0..N).map(|i| self.moon_energy(i)).sum()
    }

    fn new(x: [i64; N], y: [i64; N], z: [i64; N]) -> Self {
        let x_axis = Axis {
            positions: x,
            //            velocities: zeros(),
            velocities: zeros(),
        };
        let y_axis = Axis {
            positions: y,
            velocities: zeros(),
        };
        let z_axis = Axis {
            positions: z,
            velocities: zeros(),
        };
        System {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    fn find_cycle(&mut self) -> usize {
        let x_cycle = self.x_axis.find_cycle();
        let y_cycle = self.y_axis.find_cycle();
        let z_cycle = self.z_axis.find_cycle();

        lcm3(x_cycle, y_cycle, z_cycle)
    }
}

fn main() {
    let mut system1: System<4> = System::new([3, 4, -10, -3], [3, -16, -6, 0], [0, 2, 5, -13]);
    let mut system2 = system1.clone();

    for _ in 0..1000 {
        system1.step();
    }
    println!("Part 1: total energy = {}", system1.total_energy());

    println!("Part 2: cycle length = {}", system2.find_cycle());
}
