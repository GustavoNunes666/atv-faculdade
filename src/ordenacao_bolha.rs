//Condição Big O(n²)
//O aloritmo Possui dois loops aninhados e passa por todos os elementos da lista n vezes para poder ordenar os elementos.
//Quantidade de loops: 2

pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();

    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}