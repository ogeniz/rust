// Construir um algoritmo que leia as 3 notas de um aluno e calcule
// a média final deste aluno. Considerar a média é ponderada e que
// o peso das notas é: 2, 3 e 5 respectivamente.

#[macro_use]
extern crate text_io;

fn main() {

    println!("Entre com as três notas dos testes.:");

    println!("n1.:");
    let n1: f32 = read!();
    println!("n2.:");
    let n2: f32 = read!();
    println!("n3.:");
    let n3: f32 = read!();

    println!(
        "As notas dos três testes são {:?}, {:?} e {:?}, com a média igual a {:?}",
        n1,
        n2,
        n3,
        (n1 * 7.0 + n2 * 15.0 + n3 * 10.0) / 32.0
    );
}
