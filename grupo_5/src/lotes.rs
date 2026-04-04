// Exercício 19 — Fila com iteração controlada
// processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize)
// Processa elementos em lotes de tamanho fixo, imprimindo cada lote.

use std::collections::VecDeque;

fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    let mut numero_lote = 1;

    while !fila.is_empty() {
        let mut lote: Vec<i32> = Vec::new();

        for _ in 0..tamanho_lote {
            match fila.pop_front() {
                Some(v) => lote.push(v),
                None => break,
            }
        }

        let soma: i32 = lote.iter().sum();
        println!(
            "  Lote {:>2} | elementos: {:>2} | valores: {:>35?} | soma: {}",
            numero_lote,
            lote.len(),
            lote,
            soma
        );
        numero_lote += 1;
    }
}

pub fn executar() {
    println!("=== Exercício 19: Fila com Iteração Controlada (Lotes) ===\n");

    // Teste 1: divisão exata
    let mut fila: VecDeque<i32> = (1..=12).collect();
    println!("  Fila: {:?} | lote = 4", Vec::from(fila.clone()));
    processar_em_lotes(&mut fila, 4);

    println!();

    // Teste 2: último lote incompleto
    let mut fila2: VecDeque<i32> = (1..=10).collect();
    println!("  Fila: {:?} | lote = 3", Vec::from(fila2.clone()));
    processar_em_lotes(&mut fila2, 3);

    println!();
}
