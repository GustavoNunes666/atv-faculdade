//Condição Big O(1)
//Porque por não ter loops presentes o algoritmo sera executado de forma constante, tendo seu tempo de execução proporcional a quantidade de elementos.
//Quantidade de loops: 0

pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {     
    lista.first().copied() 
} 