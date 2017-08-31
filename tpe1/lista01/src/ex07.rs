// Um sistema de equações lineares do tipo:
/*
ax + by = c
dx + ey = f
 */
//pode ser resolvido segundo abaixo.
/*
x = (ce - bf)/(ae - bd) y = (af - cd)/(ae - bd)
*/

// Escreva um programa que lê os coeficientes a até f e calcula os
// os valores de x e y.

#[macro_use]
extern crate text_io;

fn main() {
    println!("Entre com os coeficentes abaixo.:");
    println!("a.:");
    let a: f32 = read!();
    println!("b.:");
    let b: f32 = read!();
    println!("c.:");
    let c: f32 = read!();
    println!("d.:");
    let d: f32 = read!();
    println!("e.:");
    let e: f32 = read!();
    println!("f.:");
    let f: f32 = read!();
    //
    let denominator: f32 = a * e - b * d;
    let x: f32 = (c * e - b * f) / denominator;
    let y: f32 = (a * f - c * d) / denominator;
    println!(
        "A solução parar o sistema {:?}x + {:?}y = {:?} e {:?}x + {:?}y = {:?} é x = {:?} e y {:?}",
        a,
        b,
        c,
        d,
        e,
        f,
        x,
        y
    );
}
