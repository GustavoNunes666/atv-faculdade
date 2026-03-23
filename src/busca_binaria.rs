//Condição Big O(log n)
//O aloritmo será executado partindo a quantidade de elementos ao meio e seguirá dessa forma ate que o resultado esperado seja encontrado.
//Quantidade de loops: 1

pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> { 
    let mut esquerda: isize = 0; 
    let mut direita: isize = lista.len() as isize - 1; 
    while esquerda <= direita {
        let meio = (esquerda + direita) / 2; 
        let idx = meio as usize; 
        if lista[idx] == alvo { return Some(idx);        
         } else if lista[idx] < alvo {             
            esquerda = meio + 1;        
         } else {
            direita = meio - 1;
         }     
    } 
    None 
} 