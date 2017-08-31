// Construir um algoritmo que, tendo como dados de entrada dois pontos
// quaisquer no plano, A(x1,y1) e B(x2,y2), escrever a distância entre
// eles. Para o cálculo use d = sqrt((x2 - x1)^2 - (y2 - y1)^2)

#[macro_use]
extern crate text_io;

fn main() {

    println!("Entre com dois pares ordenados, ou seja 4 valores.:");

    println!("x1.:");
    let x1: f32 = read!();
    println!("x2.:");
    let x2: f32 = read!();
    println!("y1.:");
    let y1: f32 = read!();
    println!("y2.:");
    let y2: f32 = read!();

    println!(
        "A distancia euclidiana entre os pontos no plano A({:?},{:?}) e B({:?},{:?}) é {:?}",
        x1,
        y1,
        x2,
        y2,
        ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt()
    );
}
