//Condição Big O(n²)
//Porque o algoritmo possui loops aninhados e porque cada elemento seria comparado com os outros elementos presentes.
//Quantidade de loops: 3

pub fn imprimir_pares_e_pares(lista: &[i32]) {
    // 🔹 Bloco 1:
    for &x in lista {
        println!("{}", x);
    }

    // 🔹 Bloco 2: 
    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
        }
    }
}