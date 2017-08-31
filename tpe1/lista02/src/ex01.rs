// Ler o comprimento dos três lados de um triângulo (a,b,c) e
// determinar o tipo de triângulo, com base nos seguintes casos
/*
a) Se A >= B+C, nenhum triângulo é formado.
b) Se A^2 = B^2 + C^2, é formado um triângulo retângulo.
c) Se A^2 > B^2 + C^2, é formado um triângulo obtusângulo.
d) SE A^2 < B^2 + C^2, é formado um triângulo acutângulo.
 */
// Observações:
/*
A é o maior dos lados
B e C são os outros lados
Prever no algorítmo a possibilidade de serem fornecidos dados
negativos e indicar erro.
 */

#[macro_use]
extern crate text_io;

fn main() {
    println!("Entre com três valores na seguinte seqüência (A,B,C), onde A > (B e C)");
    println!("A.:");
    let a: u16 = read!();
    println!("B.:");
    let b: u16 = read!();
    if !(a > b) {
        loop {
            println!("B.:");
            let b: u16 = read!();
            if !(a > b) {
                continue;
            } else {
                break;
            }
        }
    }
    println!("C.:");
    let c: u16 = read!();
    if !(a > c) {
        loop {
            println!("C.:");
            let c: u16 = read!();
            if !(a > c) {
                continue;
            } else {
                break;
            }
        }
    }
    if a >= (b + c) {
        println!(".:Nenhum triângulo é formado.:");
    } else if a.pow(2) == (b.pow(2) + c.pow(2)) {
        println!("É formado um triângulo retângulo");
    } else if a.pow(2) > (b.pow(2) + c.pow(2)) {
        println!("É formado um triângulo obtusângulo");
    } else if a.pow(2) < (b.pow(2) + c.pow(2)) {
        println!("É formado um triângulo acutângulo");
    }
}
