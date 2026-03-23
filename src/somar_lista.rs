//Condição Big O(n)
//Porque o tempo de execução será de um loop unico sobre o numero de elementos
//Quantidade de loops: 1

pub fn somar_lista(lista: &[i32]) -> i32 { 
    let mut total = 0; 
    for &elemento in lista {         
        total += elemento;     
    }     
    total 
} 