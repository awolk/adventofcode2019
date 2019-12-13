use std::collections::HashSet;

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

#[derive(Hash, Eq, PartialEq, Clone)]
struct Axis {
    positions: [i64; 4],
    velocities: [i64; 4],
}

impl Axis {
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
struct System {
    x_axis: Axis,
    y_axis: Axis,
    z_axis: Axis,
}

impl System {
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
        (0..4).map(|i| self.moon_energy(i)).sum()
    }

    fn new(moons: [(i64, i64, i64); 4]) -> System {
        let x_axis = Axis {
            positions: [moons[0].0, moons[1].0, moons[2].0, moons[3].0],
            velocities: [0; 4],
        };
        let y_axis = Axis {
            positions: [moons[0].1, moons[1].1, moons[2].1, moons[3].1],
            velocities: [0; 4],
        };
        let z_axis = Axis {
            positions: [moons[0].2, moons[1].2, moons[2].2, moons[3].2],
            velocities: [0; 4],
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
    let mut system1 = System::new([(3, 3, 0), (4, -16, 2), (-10, -6, 5), (-3, 0, -13)]);
    let mut system2 = system1.clone();

    // part 1
    for _ in 0..1000 {
        system1.step();
    }
    println!("Part 1: total energy = {}", system1.total_energy());

    // part 2
    println!("Part 2: cycle length = {}", system2.find_cycle());
}
