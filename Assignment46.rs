//#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1; // Access the second element of the tuple

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
