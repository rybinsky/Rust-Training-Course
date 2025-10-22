use crate::tasks::c8_iterators_and_closures::{
    filter_and_sort_names,
    group_students_by_grade,
    top_k_most_common_letters,
    word_frequencies,
};

// #[test]
// fn test_() {

// }

#[test]
fn test_word_frequencies() {
    let dogma = "
In the first age, in the first battle
When the shadows first lengthened, one stood
He chose the path of perpetual torment
In his ravenous hatred, he found no peace
And with boiling blood, he scoured the umbral plains, seeking vengeance against the dark lords who had wronged him
And those that tasted the bite of his sword named him...
The Doom Slayer
";

    (0..10).for_each(|_| {
        let frequencies = word_frequencies(dogma);

        assert_eq!(frequencies[0], ("the".to_string(), 8));
        assert_eq!(frequencies[1], ("first".to_string(), 3));
        assert_eq!(frequencies[2], ("he".to_string(), 3));
        assert_eq!(frequencies[3], ("in".to_string(), 3));
        assert_eq!(frequencies[4], ("and".to_string(), 2));
    });
}

#[test]
fn test_top_k_most_common_letters() {
    let dogma = "
In the first age, in the first battle
When the shadows first lengthened, one stood
He chose the path of perpetual torment
In his ravenous hatred, he found no peace
And with boiling blood, he scoured the umbral plains, seeking vengeance against the dark lords who had wronged him
And those that tasted the bite of his sword named him...
The Doom Slayer
";

    (0..10).for_each(|_| {
        let top_k_letters = top_k_most_common_letters(dogma, 6);
        assert_eq!(
            top_k_letters,
            vec![('e', 38), ('t', 28), ('h', 26), ('o', 22), ('a', 21), ('n', 21)]
        );
    });
}

#[test]
fn test_filter_and_sort_names() {
    let student_names = vec![
        "Xyz".to_string(),
        "biba".to_string(),
        "Bob".to_string(),
        "Avdotiya Olegovna".to_string(),
        "Vince Zampella".to_string(),
    ];

    assert_eq!(
        filter_and_sort_names(student_names, 4),
        vec![
            "Avdotiya Olegovna".to_string(),
            "biba".to_string(),
            "Vince Zampella".to_string()
        ]
    );
}

#[test]
fn test_group_students_by_grade() {
    let student_data = vec![
        ("Xyz".to_string(), 50),
        ("biba".to_string(), 15),
        ("Bob".to_string(), 15),
        ("Avdotiya Olegovna".to_string(), 95),
        ("Vince Zampella".to_string(), 95),
    ];

    let grouped = group_students_by_grade(student_data);

    assert_eq!(
        grouped.get(&95).unwrap(),
        &vec!["Avdotiya Olegovna".to_string(), "Vince Zampella".to_string()]
    );
    assert_eq!(grouped.get(&50).unwrap(), &vec!["Xyz".to_string()]);
    assert_eq!(grouped.get(&15).unwrap(), &vec!["biba".to_string(), "Bob".to_string()]);
}
