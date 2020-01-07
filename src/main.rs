use std::{thread, time};

type Seconds = f64;
type Kilometers = f64;
type MetersPerSecSquared = f64;

#[derive(Debug)]
struct Projectile {
    origin: Point,
    destination: Point,
}

impl Projectile {
    fn new(origin: Point, destination: Point) -> Projectile {
        Projectile {
            origin,
            destination,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Point {
    x: Kilometers,
    y: Kilometers,
    z: Kilometers,
}

impl Point {
    fn new(x: Kilometers, y: Kilometers, z: Kilometers) -> Point {
        Point {x, y, z}
    }
}

fn main() {
    // simulation state variables
    let projectile = Projectile::new(
        Point::new(1.0, 1.0, 0.0),
        Point::new(10.0, 10.0, 0.0),
    );

    let mut current_position = projectile.origin.clone();

    for seconds in 1.. {

        // Exit conditions
        if current_position.y > 5.0 {
            break;
        }

        current_position = projectile_position(&projectile, seconds as f64);

        println!("{:?}", current_position);

        // Enforce 1 Second "frame rate"
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn projectile_position(p: &Projectile, seconds: f64) -> Point {
    let flight_duration: Seconds = 20.0;
    let x = (p.destination.x - p.origin.x) / flight_duration * seconds;
    let y = (p.destination.y - p.origin.y) / flight_duration * seconds;
    let z = (p.destination.z - p.origin.z) / flight_duration * seconds;
    Point::new(x, y, z)
}

// The gravitational force is propoportional to 1/R^2, where R is the distance
// to the center of the Earth.
fn gravitational_force(altitude: Kilometers) -> MetersPerSecSquared {
    let gravitational_force_at_equator: MetersPerSecSquared = 9.798;
    let equator_to_core: Kilometers = 6378.0;
    let distance: Kilometers = altitude + equator_to_core;
    gravitational_force_at_equator / (distance / equator_to_core).powf(2.0)
}
