fn main() {
    let letra: char = 'A';
    let emoji: char = '😀';
    let unicode: char = '\u{1F600}'; // Outra forma de representar o emoji "😀"
    println!("Letra: {}, Emoji: {}, Unicode: {}", letra, emoji, unicode);

    let mut mensagem: String = String::from("Olá, mundo!");
    println!("Mensagem: {}", mensagem);

    mensagem.push_str(" Rust é legal!"); // Adiciona texto ao final da string
    println!("Mensagem modificada: {}", mensagem);

    let numero = 42;
    let mensagem_formatada: String = format!("O número é: {}", numero);
    println!("{}", mensagem_formatada);

    // Convertendo um &str para String
    let str_slice: &str = "texto";
    let string: String = str_slice.to_string(); // ou String::from(str_slice);
    println!("menssagem formatada: {}", string);

    // string slice
    let nova_mensagem: &str = "tipo string slice";
    println!("agora estamos estudando uma: {}", nova_mensagem);
    let exemplo_string: String = String::from("aqui temos uma nova string");
    let slice: &str = &exemplo_string[0..7]; // Cria uma fatia da String
    println!("slice: {}", slice);
}