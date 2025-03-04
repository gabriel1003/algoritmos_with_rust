struct Student { name: String, level: u8, remote: bool }

fn main() {
    let declaration = Student { name: String::from("Gabriel"), level: 2, remote: true } ;

    println!("{}, level: {} e remote: {}", declaration.name, declaration.level, declaration.remote);

}