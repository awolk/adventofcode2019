use std::iter::Sum;
use std::ops::Add;

fn cmp(a: i64, b: i64) -> i64 {
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        0
    }
}

#[derive(Clone, Copy)]
struct Vec3(i64, i64, i64);

impl Vec3 {
    fn abs_sum(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn cmp(&self, other: &Vec3) -> Vec3 {
        Vec3(
            cmp(self.0, other.0),
            cmp(self.1, other.1),
            cmp(self.2, other.2),
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        iter.fold(Vec3(0, 0, 0), |acc, vec| acc + vec)
    }
}

struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new_at_rest(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            pos: Vec3(x, y, z),
            vel: Vec3(0, 0, 0),
        }
    }

    fn potential_energy(&self) -> i64 {
        self.pos.abs_sum()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel.abs_sum()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn apply_gravity<'a>(&self, other_moons: impl Iterator<Item = &'a Moon>) -> Moon {
        let acceleration = other_moons.map(|moon| moon.pos.cmp(&self.pos)).sum();

        let vel = self.vel + acceleration;
        let pos = self.pos + vel;
        Moon { pos, vel }
    }
}

fn main() {
    let mut moons = vec![
        Moon::new_at_rest(3, 3, 0),
        Moon::new_at_rest(4, -16, 2),
        Moon::new_at_rest(-10, -6, 5),
        Moon::new_at_rest(-3, 0, -13),
    ];

    for i in 0..1000 {
        let new_moons: Vec<Moon> = moons
            .iter()
            .map(|m| m.apply_gravity(moons.iter()))
            .collect();
        moons = new_moons;
    }

    let total_energy: i64 = moons.iter().map(|moon| moon.total_energy()).sum();

    println!("Total energy: {}", total_energy);
}
