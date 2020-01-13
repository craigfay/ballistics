use std::{thread, time};
use std::fs;
use ron::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;

type Seconds = f64;
type Meters = f64;
type MetersPerSec = f64;
type MetersPerSecSquared = f64;

#[derive(Debug, Deserialize, Clone)]
struct Projectile {
    origin: Point,
    destination: Point,
    speed: MetersPerSec,
}

impl Projectile {
    fn new(
        origin: Point,
        destination: Point,
        speed: MetersPerSec
    ) -> Projectile {
        Projectile {
            origin,
            destination,
            speed,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Point {
    x: Meters,
    y: Meters,
    z: Meters,
}

impl Point {
    fn new(x: Meters, y: Meters, z: Meters) -> Point {
        Point {x, y, z}
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Config {
    projectile: Projectile,
}


fn main() {
    // Read config
    let config_file = "config.ron";
    let config_str = fs::read_to_string(config_file)
        .expect("Could not read config file");
    let config: Config = from_str(&config_str)
        .expect("Could not deserialize config");

    // simulation state variables
    let projectile = &config.projectile;

    let mut current_position = projectile.origin.clone();

    for seconds in 1.. {

        // Exit conditions
        if current_position.z < 0.0 && seconds > 1 {
            break;
        }

        current_position = projectile_position(&projectile, seconds as f64);

        println!("{:?}", current_position);

        // Enforce 1 Second "frame rate"
        thread::sleep(time::Duration::from_secs(1));
    }
}

// Given a projectile and its time in flight, calculate its position
fn projectile_position(p: &Projectile, seconds: f64) -> Point {
    let total_distance: Meters = x_y_distance(&p.origin, &p.destination);
    let distance_traveled: Meters = p.speed * seconds;
    let flight_duration: Seconds = total_distance / p.speed;

    let x = (p.destination.x - p.origin.x) / flight_duration * seconds;
    let y = (p.destination.y - p.origin.y) / flight_duration * seconds;
    let z = - (seconds.powf(2.0)) + (flight_duration * seconds);
    // How can we precisely control the z-axis peak?
    Point::new(x, y, z)
}


// Calculate the distance between two points, excluding the z-axis
fn x_y_distance(p1: &Point, p2: &Point) -> Meters {
    ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0)).sqrt()
}

// The gravitational force is propoportional to 1/R^2, where R is the distance
// to the center of the Earth.
fn gravitational_force(altitude: Meters) -> MetersPerSecSquared {
    let gravitational_force_at_equator: MetersPerSecSquared = 9.798;
    let equator_to_core: Meters = 6378.0 * 1000.0;
    let distance: Meters = altitude + equator_to_core;
    gravitational_force_at_equator / (distance / equator_to_core).powf(2.0)
}
