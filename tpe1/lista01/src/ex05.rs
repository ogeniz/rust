// Fazer um algoritmo que leia o tempo de duração de um evento em uma
// fábrica expresso em segundos e mostre-o expresso em horas, minutos
// e segundos.

#[macro_use]
extern crate text_io;

fn main() {
    println!(
        "Entre com o valor do tempo decorrido com o \
         evento em segundos, no máximo de 24 horas \
         ou 86400s.:"
    );
    let intro: u32 = read!();
    let hour: u8 = (intro / 3600) as u8;
    let min: u8 = ((intro % 3600) / 60) as u8;
    let sec: u8 = (intro - 60 * (hour as u32 * 60 + min as u32)) as u8;
    println!(
        "{:?} segundos é o mesmo que {:?}h:{:?}m:{:?}s",
        intro,
        hour % 24,
        min % 60,
        sec % 60
    );
}
