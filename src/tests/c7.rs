use crate::tasks::c7_generics_traits_lifetimes::{
    Area,
    Article,
    Book,
    Pair,
    Rectangle,
    Tweet,
    longest_string,
    notify,
};

#[test]
fn test_pair_max() {
    let integer_pair = Pair::new(3, 7);
    assert_eq!(integer_pair.max(), &7);

    let string_pair = Pair::new("apple".to_string(), "pear".to_string());
    assert_eq!(string_pair.max(), "pear");
}

#[test]
fn test_rectangle_area() {
    let rectangle = Rectangle::new(3.0, 4.0);
    assert_eq!(rectangle.area(), 12.0);
}

#[test]
fn test_notify() {
    let article = Article::new(
        "Rust conquers the world".to_string(),
        "Ferris Crab".to_string(),
        "Rust has officially become the most loved language.".to_string(),
    );

    let tweet = Tweet::new("rustacean".to_string(), "I love Rust!".to_string());

    assert_eq!(notify(&article), "Breaking news: Rust conquers the world by Ferris Crab");
    assert_eq!(notify(&tweet), "Breaking news: @rustacean: I love Rust!");
}

#[test]
fn test_longest_string() {
    assert_eq!(longest_string("rust", "cargo"), "cargo");
    assert_eq!(longest_string("longstring", "short"), "longstring");
}

#[test]
fn test_longest_word() {
    let book = Book::new(
        "Rust Adventures",
        "Rust is a systems programming language that runs blazingly fast",
    );

    assert_eq!(book.longest_word(), Some("programming"));

    let empty_book = Book::new("Empty", "");
    assert_eq!(empty_book.longest_word(), None);
}
