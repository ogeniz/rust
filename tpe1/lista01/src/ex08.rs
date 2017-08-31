// Escreva um algoritmo que leia três números inteiros e
// positivos (A,B,C) e calcule a seguinte expressão
// D = (R + S) << 1, onde R = (A + B)^2 S = (B + C)^2

#[macro_use]
extern crate text_io;

fn main() {
    println!("Entre com três valores inteiros e positivos.:");
    println!("A.:");
    let A: u16 = read!();
    println!("B.:");
    let B: u16 = read!();
    println!("C.:");
    let C: u16 = read!();
    println!("Para {:?}, {:?} e {:?} temos.:", A, B, C);
    println!("R = ({:?} + {:?})^2 e S = ({:?} + {:?})^2", A, B, B, C);
    println!(
        "Assim D = ({:?} + {:?}) * 0.5 = {:?}",
        (A + B).pow(2),
        (B + C).pow(2),
        0.5 * ((A + B).pow(2) + (B + C).pow(2)) as f32
    )
}
