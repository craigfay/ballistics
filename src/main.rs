use std::{thread, time};

fn main() {
    // simulation state variables
    let mut finished = false;
    let mut projectile_x: f64 = 0.0;
    let mut projectile_y: f64 = 0.0;

    loop {
        if projectile_y > 10.0 {
            finished = true;
        }

        let (new_x, new_y) = calc_projectile_position(projectile_x, projectile_y);
        projectile_x = new_x;
        projectile_y = new_y;

        println!("{}, {}", projectile_x, projectile_y);

        thread::sleep(time::Duration::from_secs(1));

        if finished {
            break;
        }
    }
}

fn calc_projectile_position(x: f64, y: f64) -> (f64, f64) {
    (x + 1.0, y + 2.0)
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
