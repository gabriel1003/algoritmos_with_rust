fn main() {
    let min_value: i32 = -2147483648;
    let max_value: i32 = 2147483647;

    println!("temos para representar valores inteiros no rust o (i32) e o (u32) o (i) são valores assinados ou seja, aceitam valores negativos enquanto o (u) só aceita valores negativos.");
    println!("O menor valor do tipo (i32) é: {}", min_value);
    println!("e o maior valor é: {}", max_value);

    let min: u32 = 0;
    let max: u32 = 4294967295;
    println!("Já o menor valor do (u) é: {}", min);
println!("E o menor valor é: {}", max);
}