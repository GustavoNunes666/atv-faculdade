//Remoção condicional
pub fn executar(){
    let mut numeros = vec![1, 2, 3, 4, 5, 6];
    
    let mut novo = Vec :: new();

    for numero in &numeros{
        if *numero % 2 != 0{
            novo.push(*numero);

        } else {
            
        println!("Esse número não é par");

        }
    }

    println!("{:?}", novo);
    
}