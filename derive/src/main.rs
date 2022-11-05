use derive::*;

trait DogTraits {
    fn barks(&self);
    fn fetch_ball(&self);
}

#[derive(DogTraits)]
struct Dog {
    name: String,
}

fn main() {
    let tommy = Dog {
        name: "Tommy".to_owned(),
    };

    tommy.barks();
    tommy.fetch_ball();
}
