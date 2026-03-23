//Condição Big O(n²)
//O algoritmo procura dois elementos na lista e soma os mesmos, fazendo com que toda a lista seja percorrida ate os elementos serem localizados.
//Quantidade de loops: 1

pub fn pares_com_soma(lista: &[i32], alvo: i32) { 
    let n = lista.len(); for i in 0..n { 
        for j in (i + 1)..n { 
            if lista[i] + lista[j] == alvo { 
                println!("{} + {} = {}", lista[i], lista[j], alvo);             
            }         
        }     
    } 
} 