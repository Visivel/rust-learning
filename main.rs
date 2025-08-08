use std::io;
fn main(){
    let mut pergunta = String::new();
    const SENHA: &str = "beterraba";
    const PROCURAR: u32 = 2000000;

    io::stdin()
        .read_line(&mut pergunta)
        .expect("Erro");
    
    if pergunta.trim() != SENHA {
        println!("Voce nao foi autenticado");
    } else {
        println!("Autenticado com sucesso");

        for i in 1..{
            if i == PROCURAR {
                println!("ENCONTRADO: {}",i);
                break;
            } else {
                println!("nao encontrado - {}",i)
            }
        }

    }
}