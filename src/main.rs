mod mass;
mod skills;
mod tools;
mod runtime;

fn main() {
    let masses = vec![mass::Mass::placeholder()];
    println!("Ignited {} Mass instance(s)", masses.len());
}