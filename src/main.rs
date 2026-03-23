//Declarando os módulos do programa
mod busca_binaria;
mod fibonacci_recursivo;
mod merge_sort;
mod ordenacao_bolha;
mod verificar_primeiro;
mod imprimir_pares_e_pares;
mod pares_com_soma;
mod potencias_de_dois;
mod produto_de_matrizes;
mod somar_lista;

fn main() {
    println!("Testando os algoritimos");

    //Lista base
    let numeros = vec![10, 20, 30, 40, 50];
    let mut lista = vec![5, 3, 8, 1, 2];

    //verificar_primeiro
    println!("--- verificar_primeiro ---");
    println!("{:?}", verificar_primeiro::verificar_primeiro(&numeros));

    //somar_lista
    println!("\n--- somar_lista ---");
    println!("{}", somar_lista::somar_lista(&numeros));

    //fibonacci_recursivo
    println!("\n--- fibonacci_recursivo ---");
    println!("{}", fibonacci_recursivo::fibonacci_recursivo(5));

    //ordenacao_bolha
    println!("\n--- ordenacao_bolha ---");
    println!("{:?}", ordenacao_bolha::ordenacao_bolha(&mut lista));

    //merge_sort
    println!("\n--- merge_sort ---");
    println!("{:?}", merge_sort::merge_sort(lista.clone()));

    //busca_binaria
    println!("\n--- busca_binaria ---");
    println!("{:?}", busca_binaria::busca_binaria(&numeros, 30));

    //pares_com_soma
    println!("\n--- pares_com_soma ---");
    println!("{:?}", pares_com_soma::pares_com_soma(&numeros, 50));

    //imprimir_pares_e_pares
    println!("\n--- imprimir_pares_e_pares ---");
    imprimir_pares_e_pares::imprimir_pares_e_pares(&numeros);

    //potencias_de_dois
    println!("\n--- potencias_de_dois ---");
    potencias_de_dois::potencias_de_dois(5);

    //produto_de_matrizes
    println!("\n--- produto_de_matrizes ---");
    let matriz_a = vec![
        vec![1, 2],
        vec![3, 4],
    ];

    let matriz_b = vec![
        vec![5, 6],
        vec![7, 8],
    ];

    let n = matriz_a.len();

    println!("{:?}", produto_de_matrizes::produto_de_matrizes(matriz_a, matriz_b, n));

    println!("Fim");
}