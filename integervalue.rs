fn main() {

    let min_u8: u8 = u8::MIN;   // Usa a constante u8::MIN
    let max_u8: u8 = u8::MAX; // Usa a constante u8::MAX

    let min_i8: i8 = i8::MIN;  // Usa a constante i8::MIN
    let max_i8: i8 = i8::MAX; // Usa a constante i8::MAX

    let min_u16: u16 = u16::MIN; // Usa a constante u16::MIN
    let max_u16: u16 = u16::MAX; // Usa a constante u16::MAX

    let min_i16: i16 = i16::MIN; // Usa a constante i16::MIN
    let max_i16: i16 = i16::MAX; // Usa a constante i16::MAX

    let min_value_i32: i32 = i32::MIN; // Usa a constante i32::MIN
    let max_value_i32: i32 = i32::MAX; // Usa a constante i32::MAX

    let min_u32: u32 = u32::MIN; // Usa a constante u32::MIN
    let max_u32: u32 = u32::MAX; // Usa a constante u32::MAX

    let min_u64: u64 = u64::MIN; // Usa a constante u64::MIN
    let max_u64: u64 = u64::MAX; // Usa a constante u64::MAX

    let min_i64: i64 = i64::MIN; // Usa a constante i64::MIN
    let max_i64: i64 = i64::MAX; // Usa a constante i64::MAX

    let min_u128: u128 = u128::MIN; // Usa a constante u128::MIN
    let max_u128: u128 = u128::MAX; // Usa a constante u128::MAX

    let min_i128: i128 = i128::MIN; // Usa a constante i128::MIN
    let max_i128: i128 = i128::MAX; // Usa a constante i128::MAX

    println!("Tipos inteiros (i) são assinados (aceitam negativos), (u) não.");

    println!("O menor valor do (u8) é: {}", min_u8);
    println!("E o maior valor é: {}", max_u8);

    println!("Já o menor valor do (i8) é: {} e o maior é: {}", min_i8, max_i8);

    println!("Já no (u16) o menor valor é: {} e o maior valor é: {}", min_u16, max_u16);

    println!("O menor valor do (i16) é: {} e o maior valor é: {}", min_i16, max_i16);

    println!("O menor valor do tipo (i32) é: {}", min_value_i32);
    println!("e o maior valor é: {}", max_value_i32);

    println!("Já o menor valor do (u32) é: {}", min_u32);
    println!("E o maior valor é: {}", max_u32);
    println!("O menor valor do tipo (u64) é: {} e o maior valor é: {}", min_u64, max_u64);

    println!("O menor valor do (i64) é: {} e o maior valor é: {}", min_i64, max_i64);

    println!("O menor valor do (u128) é: {} e o maior valor é: {}", min_u128, max_u128);

    println!("O menor valor do (i128) é: {} e o maior valor é: {}", min_i128, max_i128);
}