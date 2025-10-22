use crate::tasks::c4_structs_methods_enums_pattern_matching::{
    BankAccount,
    Company,
    Operation,
    TrafficLight,
    WeirdLengthMeasures,
    fizzbuzz,
    point_checker,
    rectangle_checker,
};

#[test]
fn test_point_struct() {
    point_checker();
}

#[test]
fn test_rectangle_struct() {
    rectangle_checker();
}

#[test]
fn test_company() {
    let company = Company::new(String::from("ПОРА КВАСИТЬ"), 1999, 69000);
    assert_eq!(1794000, company.total_income())
}

#[test]
fn test_bank_account() {
    let mut acc = BankAccount::new(String::from("Frank Drebin"), 100);

    acc.deposit(50);
    assert_eq!(acc.balance(), 150);

    assert!(acc.withdraw(100));
    assert_eq!(acc.balance(), 50);

    assert!(!acc.withdraw(100)); // insufficient
}

#[test]
fn test_traffic_light() {
    let red = TrafficLight::Red;
    assert!(matches!(red.next(), TrafficLight::Green));
}

#[test]
fn test_operation() {
    assert_eq!(Operation::Add(2, 3).apply(), Some(5));
    assert_eq!(Operation::Subtract(10, 4).apply(), Some(6));
    assert_eq!(Operation::Multiply(3, 7).apply(), Some(21));
    assert_eq!(Operation::Divide(10, 2).apply(), Some(5));
    assert_eq!(Operation::Divide(5, 0).apply(), None);
}

#[test]
fn test_weird_length_measures() {
    assert_eq!(WeirdLengthMeasures::Inch.convert_to_human_format(), 0.0254);
    assert_eq!(WeirdLengthMeasures::Foot.convert_to_human_format(), 0.3048);
    assert_eq!(WeirdLengthMeasures::Yard.convert_to_human_format(), 0.9144);
    assert_eq!(WeirdLengthMeasures::Mile.convert_to_human_format(), 1609.344);
}

#[test]
fn test_fizzbuzz() {
    let expected = ["1", "Fizz"].map(String::from).to_vec();
    assert_eq!(expected, fizzbuzz(2));

    let expected = ["1", "Fizz", "Buzz"].map(String::from).to_vec();
    assert_eq!(expected, fizzbuzz(3));

    let expected = ["1", "Fizz", "Buzz", "Fizz", "5", "FizzBuzz"].map(String::from).to_vec();
    assert_eq!(expected, fizzbuzz(6));
}
