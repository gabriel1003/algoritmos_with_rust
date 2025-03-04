struct Grades(char, char, char, char, f32);

fn main() {
    let declaration = Grades('A', 'B', 'C', 'D', 3.14);

    println!("This is the first four letters of the alphabet: '{}', '{}', '{}', and '{}'. This is the value of PI: {}", declaration.0, declaration.1, declaration.2, declaration.3, declaration.4);
}
