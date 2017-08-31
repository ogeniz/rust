// Escrever um programa em rust para ler uma temperatura
// em graus Fahrenheit, calcular e escrever o valor
// correspondente em graus Celcius de acordo com a
// fórmula C = 5*(F - 32)/9

#[macro_use]
extern crate text_io;

fn main() {
    println!("Entre com um valor de temperatura em Fahrenheit.:");
    let f: f32 = read!();
    println!(
        "O valor de {:?}F em Celcius é {:?}oC",
        f,
        5.0 * (f - 32.0) / 9.0
    );
}
