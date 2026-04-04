//Contando ocorrencias
//Biblioteca do HashMap
use std::collections::HashMap;

//Declarando a função
pub fn executar() {
    let mut letras = vec!['a', 'b', 'c', 'd'];

    //Criando o contador de letras
    let mut contador = HashMap :: <char, i32> :: new();

    for letra in &letras {
        println!("{:?}", letra);

        //Verificando a quantidade de itens
        if let Some(valor) = contador.get(letra) {
            
            contador.insert(*letra, *valor + 1);

        } else {
                contador.insert(*letra, 1);
            }

            println!("{:?}", contador);
    }

}