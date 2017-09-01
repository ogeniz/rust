/*
Escrever um algoritmo que lê o número de identificação e as 3 notas
obtidas por um aluno [0.0..10.0] nas 3 verificações e a média dos
exercícios [0.0..10.0] que fazem parte da avaliação. Para cada aluno,
calcular a média de aproveitamento, usando a fórmula:
*/
/*
MA = (N1 + N2 * 2 + N3 * 3 + ME) / 7
 */
// A atribuição dos conceitos obedece a tabela abaixo:
/*
+-----------------------+---------+
|Média de Aproveitamento|Conceito |
+-----------------------+---------+
|       MA >= 9.0       |   A     |
+-----------------------+---------+
|    9.0 > MA >= 7.5    |   B     |
+-----------------------+---------+
|    7.5 > MA >= 6.0    |   C     |
+-----------------------+---------+
|    6.0 > MA >= 4.0    |   D     |
+-----------------------+---------+
|       4.0 > MA        |   E     |
+-----------------------+---------+
*/
/*
O algoritmo deve escrever o número do aluno, suas notas, a média dos
exercícios, a média de aproveitamento, o conceito correspondente e a
mensagem: "APROVADO" se o conceito for A, B, ou C e "REPROVADO" se o
conceito for D ou E.
*/


#[macro_use]
extern crate text_io;

fn main() {
    loop {
        println!(".:Entre com o número de identificação do aluno.: (0,255)");
        println!(".:0 Encerra o programa");
        let id_student: u8 = read!();
        if id_student == 0 {
            println!(".:Fim do programa.:");
            break;
        }
        println!(".:Entre com as notas do aluno no.{:?}.:", id_student);
        println!(".:1a. Nota.: [0.0,10.0]");
        let note_1: f32 = read!();
        if note_1.is_sign_negative() {
            println!(".:Notas negativas não são permitidas.:");
            continue;
        }
        println!(".:2a. Nota.: [0.0,10.0]");
        let note_2: f32 = read!();
        if note_2.is_sign_negative() {
            println!(".:Notas negativas não são permitidas.:");
            continue;
        }
        println!(".:3a. Nota.: [0.0,10.0]");
        let note_3: f32 = read!();
        if note_3.is_sign_negative() {
            println!(".:Notas negativas não são permitidas.:");
            continue;
        }
        println!(".:Média dos exercícios.: [0.0,10.0]");
        let avg_exe: f32 = read!();
        if avg_exe.is_sign_negative() {
            println!(".:Notas negativas não são permitidas.:");
            continue;
        }
        let ma: f32 = (note_1 + note_2 * 2.0 + note_3 * 3.0 + avg_exe) / 7.0;
        match ma {
            ma if ma >= 9.0 => {
                println!(
                    ".:O aluno {:?}, obteve as notas {:?}, {:?}, {:?} com sua média dos exercícios {:?}.:",
                    id_student,
                    note_1,
                    note_2,
                    note_3,
                    avg_exe
                );
                println!(
                    ".:Sua média final foi de {:?}, com o conceito A assim APROVADO.:",
                    ma
                );
            }
            ma if ma < 9.0 && ma >= 7.5 => {
                println!(
                    ".:O aluno {:?}, obteve as notas {:?}, {:?}, {:?} com sua média dos exercícios {:?}.:",
                    id_student,
                    note_1,
                    note_2,
                    note_3,
                    avg_exe
                );
                println!(
                    ".:Sua média final foi de {:?}, com o conceito B assim APROVADO.:",
                    ma
                );
            }
            ma if ma < 7.5 && ma >= 6.0 => {
                println!(
                    ".:O aluno {:?}, obteve as notas {:?}, {:?}, {:?} com sua média dos exercícios {:?}.:",
                    id_student,
                    note_1,
                    note_2,
                    note_3,
                    avg_exe
                );
                println!(
                    ".:Sua média final foi de {:?}, com o conceito C assim APROVADO.:",
                    ma
                );
            }
            ma if ma < 6.0 && ma >= 4.0 => {
                println!(
                    ".:O aluno {:?}, obteve as notas {:?}, {:?}, {:?} com sua média dos exercícios {:?}.:",
                    id_student,
                    note_1,
                    note_2,
                    note_3,
                    avg_exe
                );
                println!(
                    ".:Sua média final foi de {:?}, com o conceito D assim REPROVADO.:",
                    ma
                );
            }
            _ => {
                println!(
                    ".:O aluno {:?}, obteve as notas {:?}, {:?}, {:?} com sua média dos exercícios {:?}.:",
                    id_student,
                    note_1,
                    note_2,
                    note_3,
                    avg_exe
                );
                println!(
                    ".:Sua média final foi de {:?}, com o conceito E assim REPROVADO.:",
                    ma
                );
            }
        }
        println!("");
    }
}
