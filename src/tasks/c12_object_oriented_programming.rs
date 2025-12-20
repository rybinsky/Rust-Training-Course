// This chapter is dedicated to the object oriented programming features of Rust.

// DYNAMIC DISPATCH
// ================================================================================================

// ----- 1 --------------------------------------
// Design a small simulation of a zoo where you have different animal types that can make noise and
// move. You have to use dynamic dispatch (trait objects) so that a collection can hold a mix of 
// different types of animals and call methods uniformly.
//
// - Define a trait `Animal` with the following methods:
//   - `fn name(&self) -> &str;`
//   - `fn make_noise(&self) -> String;`
//   - `fn move_position(&mut self, delta_x: f64, delta_y: f64);`
//   - `fn position(&self) -> (f64, f64);`
// - Create at least two concrete types implementing `Animal`, e.g. `Lion` and `BritishPigeon`. Each
//   has its own name, own noise (e.g., "Roar!", "Oi mate! Bloody hell I love fish'n'chips brof!'"),
//   and keeps track of its position `(x, y)` as `f64`.
//
// Create a struct `Zoo` that holds a vector of animals. Provide the following methods for this 
// struct:
// - `fn new -> Self`: just a basic constructor.
// - `add_animal`: adds a new animal to the zoo.
// - `make_all_noises -> Vec<String>`: calls `make_noise()` on each animal and collects the 
//   strings.
// - `move_all`: moves every animal by the given delta.
// - `positions -> Vec<(&str, (f64, f64))>`: returns a vector of tuples with each animal’s name and
//   its current position.
//
// White a small example function which creates a zoo, adds your animals there and calls the 
// `make_all_noises`, `move_all` and `positions` Zoo methods to show that they're working correctly.

trait Animal {
    fn name(&self) -> &str;
    fn make_noise(&self) -> String;
    fn move_position(&mut self, delta_x: f64, delta_y: f64);
    fn position(&self) -> (f64, f64);
}

struct Lion {
    name: String,
    x: f64,
    y: f64,
}

impl Animal for Lion {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_noise(&self) -> String {
        "Roar!".to_string()
    }

    fn move_position(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x;
        self.y += delta_y;
    }

    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

struct BritishPigeon {
    name: String,
    x: f64,
    y: f64,
}

impl Animal for BritishPigeon {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_noise(&self) -> String {
        "Oi mate! Bloody hell I love fish'n'chips brof!".to_string()
    }

    fn move_position(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x;
        self.y += delta_y;
    }

    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

struct Zoo {
    animals: Vec<Box<dyn Animal>>,
}

impl Zoo {
    fn new() -> Self {
        Self {
            animals: Vec::new(),
        }
    }

    fn add_animal(&mut self, animal: Box<dyn Animal>) {
        self.animals.push(animal);
    }

    fn make_all_noises(&self) -> Vec<String> {
        self.animals.iter().map(|a| a.make_noise()).collect()
    }

    fn move_all(&mut self, dx: f64, dy: f64) {
        for animal in &mut self.animals {
            animal.move_position(dx, dy);
        }
    }

    fn positions(&self) -> Vec<(&str, (f64, f64))> {
        self.animals
            .iter()
            .map(|a| (a.name(), a.position()))
            .collect()
    }
}

// SUPERTRAITS
// ================================================================================================

// ----- 2 --------------------------------------
// Implement the `BackTo2007` trait with the `std::fmt::Display` as a supertrait for it. Implement 
// `BackTo2007` trait for the `Account` struct which consists of `name: String` and 
// `year_of_birth: u32` fields.
//
// This `BackTo2007` trait should have just one `cringify(&self) -> String` method, which will
// make the account much more cringy by adding "★彡Xx_" to the left of the `self.to_string()` and 
// "_xX彡★" to the right. Just like that: ★彡Xx_NAGIBATOR1999_xX彡★
//
// Notice that you also should decide how to display the account.

use std::fmt;

trait BackTo2007: fmt::Display {
    fn cringify(&self) -> String {
        format!("★彡Xx_{}_xX彡★", self.to_string())
    }
}

struct Account {
    name: String,
    year_of_birth: u32,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.year_of_birth)
    }
}

impl BackTo2007 for Account {}


// DEFAULT GENERIC TYPE PARAMETERS AND ASSOCIATED TYPES
// ================================================================================================

// ----- 3 --------------------------------------
// Implement a `Converter` trait with `Input` and `Output` associated types. `Input` should have a 
// `String` default type. This trait should have a `convert` method which takes a value of type 
// `Input` and returns a value of type `Output`.
//
// Implement `Converter` for two stucts:
// - `StringToIntConverter`: converts the provided String to `i32`.
// - `IntToHexConverter`: converts the provided `i32` into the String holding its hex
//   representation.

trait Converter {
    type Input = String;
    type Output;

    fn convert(&self, input: Self::Input) -> Self::Output;
}

struct StringToIntConverter;

impl Converter for StringToIntConverter {
    type Output = i32;

    fn convert(&self, input: String) -> i32 {
        input.parse().unwrap()
    }
}

struct IntToHexConverter;

impl Converter for IntToHexConverter {
    type Input = i32;
    type Output = String;

    fn convert(&self, input: i32) -> String {
        format!("{:x}", input)
    }
}
