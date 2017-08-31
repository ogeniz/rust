// Ler um número X e calcular Y da seguite maneira:
/*
Y = X, se X < 0
Y = 1 / (1 - X²), se 0 <= X <=1
Y = 3ln(X) + X², se X > 1
 */

#[macro_use]
extern crate text_io;

fn main() {
    println!(".:Entre com um número real qualquer.:");
    let x: f32 = read!();
    let x_x: f32 = x.powi(2);
    match x {
        x if x.is_sign_negative() => {
            println!("Para X = {:?}, temos Y = {:?}", x, x);
        }
        x if (x >= 0.0 && x <= 1.0) => {
            println!("Para X = {:?}, temos Y = {:?}", x, 1.0 / (1.0 - x_x));
        }
        _ => {
            println!("Para X = {:?}, temos Y = {:?}", x, 3.0 * x.ln() + x_x);
        }
    }
}
