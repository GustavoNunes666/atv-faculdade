// Exercício 11 — Impressora compartilhada
// Simula uma fila de impressão onde cada trabalho tem nome e número de páginas.
// Imprime os trabalhos na ordem de chegada, reportando cada um.

use std::collections::VecDeque;

struct Trabalho {
    nome: String,
    paginas: u32,
}

pub fn executar() {
    println!("=== Exercício 11: Impressora Compartilhada ===\n");

    let mut fila: VecDeque<Trabalho> = VecDeque::new();

    // Enfileirando trabalhos de impressão
    let trabalhos = vec![
        ("Relatório Mensal",   15),
        ("Currículo",           2),
        ("Apresentação TCC",   42),
        ("Nota Fiscal",         1),
        ("Manual do Produto",  80),
        ("Carta de Demissão",   1),
    ];

    for (nome, paginas) in trabalhos {
        println!("  [+] Enfileirando: \"{}\" ({} páginas)", nome, paginas);
        fila.push_back(Trabalho {
            nome: nome.to_string(),
            paginas,
        });
    }

    println!("\n{} trabalho(s) na fila. Iniciando impressão...\n", fila.len());
    println!("{:<5} {:<25} {:<10} {}", "Nº", "Trabalho", "Páginas", "Status");
    println!("{}", "-".repeat(55));

    let mut numero = 1u32;
    let mut total_paginas = 0u32;

    while let Some(trabalho) = fila.pop_front() {
        total_paginas += trabalho.paginas;
        println!(
            "{:<5} {:<25} {:<10} {}",
            numero,
            trabalho.nome,
            trabalho.paginas,
            "✓ Impresso"
        );
        numero += 1;
    }

    println!("{}", "-".repeat(55));
    println!("Total: {} trabalho(s) | {} página(s) impressa(s)\n", numero - 1, total_paginas);
}
