fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn main () {
    let formal = "Formal: goodbye";
    let casual = "Casual: See you later!";
    println!("Formas de se despedir em inglês.");
goodbye(formal);
goodbye(casual);
}

