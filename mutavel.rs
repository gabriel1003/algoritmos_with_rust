fn main() {
    let mut a_value = 15;
    let mut b_value = "a palavra chave mut é o que define uma variável como mutavel.";

    println!("o primeiro valor é: {}", a_value);
    println!("Esta mensagem é importante {}", b_value);

    a_value = 3;
    b_value = "esta é a prova.";

    println!("novo valor {}", a_value);

    println!("leia a mensagem: {}", b_value);
}
