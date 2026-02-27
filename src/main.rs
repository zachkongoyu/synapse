mod organism;
mod vitality;
mod providers;
mod skills;
mod tools;

fn main() {
    vitality::assert_vitality_impl::<organism::Organism>();
    println!("Hello, world!");
}