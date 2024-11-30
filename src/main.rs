//use std::rc::Rc;

struct Thing<'a> {
    name: &'a str,
    points_to: &'a Thing<'a>
}

static THIS_THING: Thing = Thing {
    name: "Pepperoni",
    points_to: &THAT_THING
};

static THAT_THING: Thing = Thing {
    name: "Cheese",
    points_to: &THIS_THING
};

fn main() {
    println!("This thing called {} points to a thing called {}", THIS_THING.name, THIS_THING.points_to.name);
    println!("That thing called {} points to a thing called {}", THAT_THING.name, THAT_THING.points_to.name);
}
