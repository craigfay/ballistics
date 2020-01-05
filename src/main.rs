fn main() {
    let result = gravitational_force(40.0);
    println!("{}", result);
}

type Kilometers = f64;

fn gravitational_force(altitude: Kilometers) -> f64 {
    let equator_to_core: Kilometers = 6400.0;
    let distance: Kilometers = altitude + equator_to_core;

    1.0 / distance.powf(2.0)
}
