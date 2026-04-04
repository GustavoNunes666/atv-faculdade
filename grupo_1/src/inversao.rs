//Invertendo vetor 
pub fn executar(){
    let mut original = vec![1, 2, 3, 4, 5];

    //Criando um vetor vazio
    let mut invertido = Vec :: new();

    //Criando o loop para a inversão do vetor
    while original.len() > 0 {
        let valor = original.pop().unwrap();

        invertido.push(valor);
    }

    //Exibindo na tela
    println!("{:?}", invertido);
}