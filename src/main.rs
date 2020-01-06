use std::{thread, time};

#[derive(Debug)]
struct Projectile {
    x: Kilometers,
    y: Kilometers,
    z: Kilometers,
}

impl Projectile {
    fn new(x: Kilometers, y: Kilometers, z: Kilometers) -> Projectile {
        Projectile {
            x,
            y,
            z,
        }
    }
}

fn main() {
    // simulation state variables
    let mut finished = false;
    let mut projectile_a = Projectile::new(1.0, 1.0, 1.0);

    loop {
        if projectile_a.y > 10.0 {
            finished = true;
        }

        projectile_a = calc_projectile_position(projectile_a);

        println!("{:?}", projectile_a);

        thread::sleep(time::Duration::from_secs(1));

        if finished {
            break;
        }
    }
}

fn calc_projectile_position(p: Projectile) -> Projectile {
    let x = p.x + 1.0;
    let y = p.x + 2.0;
    let z = p.x + 1.0;
    Projectile::new(x,y,z)
}

type Kilometers = f64;
type MetersPerSecSquared = f64;

// The gravitational force is propoportional to 1/R^2, where R is the distance
// to the center of the Earth.
fn gravitational_force(altitude: Kilometers) -> MetersPerSecSquared {
    let gravitational_force_at_equator: MetersPerSecSquared = 9.798;
    let equator_to_core: Kilometers = 6378.0;
    let distance: Kilometers = altitude + equator_to_core;
    gravitational_force_at_equator / (distance / equator_to_core).powf(2.0)
}
