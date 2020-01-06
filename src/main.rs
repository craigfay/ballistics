fn main() {
    let result = gravitational_force(40.0);
    println!("{}", result);
}

type Kilometers = f64;

// The gravitational force is propoportional to 1/R^2, where R is the distance
// to the center of the Earth.
fn gravitational_force(altitude: Kilometers) -> f64 {
    let gravitational_force_at_equator = 9.8;
    let equator_to_core: Kilometers = 6400.0;
    let distance: Kilometers = altitude + equator_to_core;
    gravitational_force_at_equator / (distance / equator_to_core).powf(2.0)
}


