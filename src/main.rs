mod agent;
mod breath_of_life;
mod providers;
mod skills;
mod tools;

fn main() {
    breath_of_life::assert_life_impl::<agent::Agent>();
    println!("Hello, world!");
}