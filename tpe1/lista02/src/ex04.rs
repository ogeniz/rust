// Uma certa empresa decidiu conceder um aumento de salários a
// seus funcionários de acordo com a tabela abaixo:
/*
+---------------------------+-----------------+
|       Salário Atual       |Indice de Aumento|
+---------------------------+-----------------+
|       até R$400,00        |       15%       |
+---------------------------+-----------------+
| entre R$401,01 e R$700,00 |       12%       |
+---------------------------+-----------------+
|entre R$701,01 e R$1000,00 |       10%       |
+---------------------------+-----------------+
|entre R$1000,01 e R$1800,00|       7%        |
+---------------------------+-----------------+
|entre R$1800,01 e R$2500,00|       4%        |
+---------------------------+-----------------+
|    acima de R$2500,01     |       0%        |
+---------------------------+-----------------+
*/
// Fazer um algoritmo que leia, para cada funcionário, o seu número e
// o seu salário atual e imprima seu número, seu salário atual, o
// percentual de seu aumento e o valor do salário corrigido.


#[macro_use]
extern crate text_io;

fn main() {
    loop {
        println!(".:Entre com o número de identificação do funcionário..:");
        println!(".:Inteiro positivo entre 1 e 250, 0 fará o programa encerrar.:");
        println!("No.:");
        let func_number: u8 = read!();
        if func_number == 0 {
            break;
        }
        println!(".:Entre com o Salário atual..:");
        let money_earn: f32 = read!();
        if !(money_earn > 0.0) {
            continue;
        }
        match money_earn {
            money_earn if money_earn > 0.0 && money_earn < 400.01 => {
                println!(
                    "No. do funcionário {:?}, Salário atual R${:?}, Percentual de aumento {:?},\
                     Salário Corrigido R${:?}",
                    func_number,
                    money_earn,
                    "15%",
                    money_earn * (1.0 + 0.15)
                );
            }
            money_earn if money_earn > 400.0 && money_earn < 700.01 => {
                println!(
                    "No. do funcionário {:?}, Salário atual R${:?}, Percentual de aumento {:?},\
                     Salário Corrigido R${:?}",
                    func_number,
                    money_earn,
                    "12%",
                    money_earn * (1.0 + 0.12)
                );
            }
            money_earn if money_earn > 700.0 && money_earn < 1000.01 => {
                println!(
                    "No. do funcionário {:?}, Salário atual R${:?}, Percentual de aumento {:?},\
                     Salário Corrigido R${:?}",
                    func_number,
                    money_earn,
                    "10%",
                    money_earn * (1.0 + 0.10)
                );
            }
            money_earn if money_earn > 1000.0 && money_earn < 1800.01 => {
                println!(
                    "No. do funcionário {:?}, Salário atual R${:?}, Percentual de aumento {:?},\
                     Salário Corrigido R${:?}",
                    func_number,
                    money_earn,
                    "7%",
                    money_earn * (1.0 + 0.07)
                );
            }
            money_earn if money_earn > 1800.0 && money_earn < 2500.01 => {
                println!(
                    "No. do funcionário {:?}, Salário atual R${:?}, Percentual de aumento {:?},\
                     Salário Corrigido R${:?}",
                    func_number,
                    money_earn,
                    "4%",
                    money_earn * (1.0 + 0.04)
                );
            }
            _ => {
                println!(
                    "No. do funcionário {:?}, Salário atual R${:?}, Percentual de aumento {:?}",
                    func_number,
                    money_earn,
                    "0%",
                );
            }
        }
    }
    println!(".:Fim do programa.:");
}
