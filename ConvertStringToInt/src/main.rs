use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}


fn main(){

    println!("Digite o número que deseja:");

    // Coleta do input ainda como tipo String 
    let mut number_string = String:: new();
    io::stdin().read_line(&mut number_string).expect("erro ao ler number1");
    
    // Passando String para a função conversora direto para a variável de tipo inteiro
    let number : i32 =  convert_to_int(&number_string);

    println!("O número havia sido inserido como String mas agora foi convertido para Int: {}", number);
    } 
     
