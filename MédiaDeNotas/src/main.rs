// 
use std::io;

// Função responsável por converter um string para inteiro
fn convert(data_input: &String) -> i32{
   let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn main(){

    let mut medias  =String::new();
    
    // Coletando quantidade de médias que serão inseridas
    println!("digite quantas médias vc vai inserir para saber quantos alunos estão na recuperação");
    io::stdin().read_line(&mut medias).expect("Erro ao ler médias");
  
    let mut AlunosEmRec = 0; let mut contador = 0;
    let mut i = 0;

    println!("--------------");

    while convert(&medias) > i{

        let mut media_aluno = String::new();

        contador = &i + 1;
        println!("Digite a {}º média:", contador);
        io::stdin().read_line(&mut media_aluno).expect("Erro ao ler média_aluno");

        i += 1;
        
        if convert(&media_aluno) >= 3 && convert(&media_aluno) < 6 {
            AlunosEmRec += 1;

        }
    }
    println!(" Alunos com notas inferiores a 3 não tem direito a fazer recuperação \n Alunos com nota inferior a 6 estão na recuperação");
    println!("O numero de alunos em rec é {}", AlunosEmRec);

}