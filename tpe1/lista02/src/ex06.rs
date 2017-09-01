/*
Escrever um algoritmo que lê o instante do início e do término do
jogo, ambos subdivididos em 2 valores distintos, a saber: horas e
minutos. Calcular e escrever a duração do jogo, também em horas e
minutos, considerando que o tempo máximo de duração de um jogo é de 24
horas e que o jogo pode iniciar em um dia e terminar no dia seguinte.
*/


#[macro_use]
extern crate text_io;

fn main() {
    let mut tmp: u8 = 0;
    println!(".:Entre com a horas:minutos do início do jogo.:");
    println!(".:Horas.:");
    tmp = read!();
    let hour_i: u8 = tmp % 24;
    println!(".:Minutos.:");
    tmp = read!();
    let minute_i: u8 = tmp % 60;
    println!(".:Entre com a horas:minutos do encerramento do jogo.:");
    println!(".:Horas.:");
    tmp = read!();
    let hour_f: u8 = tmp % 24;
    println!(".:Minutos.:");
    tmp = read!();
    let minute_f: u8 = tmp % 60;
    println!(
        ".:Horário de início {:?}:{:?}. Horário de término {:?}:{:?}.\
         Duração da partida {:?}h:{:?}m.:",
        hour_i,
        minute_i,
        hour_f,
        minute_f,
        if (hour_f as i8 - hour_i as i8) < 0 {
            24 + (hour_f as i8 - hour_i as i8)
        } else {
            (hour_f as i8 - hour_i as i8).abs()
        },
        if (minute_f as i8 - minute_i as i8) < 0 {
            60 + (minute_f as i8 - minute_i as i8)
        } else {
            (minute_f as i8 - minute_i as i8).abs()
        }
    );
}
