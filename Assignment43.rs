fn main() {
    let a = [0; 100]; // Create an array with 100 elements, each initialized to 0

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
