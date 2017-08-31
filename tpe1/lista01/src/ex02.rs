// Escrever um programa em rust para ler uma temperatura
// em graus Celcius, calcular e escrever o valor
// correspondente em Fahrenheit de acordo com a
// fórmula f = 9*C/5 + 32

#[macro_use]
extern crate text_io;

fn main() {
    println!("Entre com um valor de temperatura em Celcius.:");
    let c: f32 = read!();
    println!(
        "O valor de {:?}oC em Fahrenheit é {:?}F",
        c,
        9.0 * c / 5.0 + 32.0
    );
}
