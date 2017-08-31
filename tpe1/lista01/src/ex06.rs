// O custo ao consumidor de um carro novo é a soma do custo de fábrica
// com a percentagem do distribuidor e dos impostos (aplicados ao
// custo de fábrica). Supondo que a percentagem do distribuidor seja
// de 28% e dos impostos de 45%, escrever um algoritmo que leia o
// o custo de fábrica de um carro e escreva o custo  ao consumidor

#[macro_use]
extern crate text_io;

fn main() {
    println!("Entre com o valor do custo de fabricação do carro.:");
    let cost: f32 = read!();
    println!(
        "Custo de fabricação R${:?}, custo de distribuição\
              R${:?}, impostos pagos R${:?}, total pago R${:?}",
        cost,
        cost * 0.28,
        cost * 0.45,
        cost * (1 + 0.28 + 0.45)
    );
}
