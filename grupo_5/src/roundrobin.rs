// Exercício 20 — Mini-projeto: Round Robin
// N processos com tempo de execução definido, quantum Q.
// Simula o escalonamento e imprime o tempo total de conclusão de cada processo.

use std::collections::VecDeque;

struct Processo {
    id: u32,
    nome: String,
    tempo_restante: u32,
    tempo_conclusao: Option<u32>,
}

pub fn executar() {
    println!("=== Exercício 20: Round Robin ===\n");

    let quantum: u32 = 3;

    let processos_iniciais = vec![
        ("P1", 10),
        ("P2",  4),
        ("P3",  7),
        ("P4",  2),
        ("P5",  6),
    ];

    let mut fila: VecDeque<Processo> = processos_iniciais
        .iter()
        .enumerate()
        .map(|(i, (nome, tempo))| Processo {
            id: (i + 1) as u32,
            nome: nome.to_string(),
            tempo_restante: *tempo,
            tempo_conclusao: None,
        })
        .collect();

    println!("  Quantum = {} | Processos:", quantum);
    for (nome, tempo) in &processos_iniciais {
        println!("    {} → tempo de execução: {}", nome, tempo);
    }

    println!("\n  {:<6} {:<6} {:<15} {:<15}", "Tempo", "Proc", "Executou", "Restante");
    println!("  {}", "-".repeat(44));

    let mut tempo_atual: u32 = 0;
    let mut concluidos: Vec<Processo> = Vec::new();

    while let Some(mut proc) = fila.pop_front() {
        let executado = proc.tempo_restante.min(quantum);
        proc.tempo_restante -= executado;
        tempo_atual += executado;

        println!(
            "  {:<6} {:<6} {:<15} {:<15}",
            tempo_atual,
            proc.nome,
            executado,
            proc.tempo_restante
        );

        if proc.tempo_restante == 0 {
            proc.tempo_conclusao = Some(tempo_atual);
            concluidos.push(proc);
        } else {
            fila.push_back(proc);
        }
    }

    println!("\n  --- Tempo de conclusão por processo ---");
    println!("  {:<6} {:<20}", "Proc", "Concluído no tempo");
    println!("  {}", "-".repeat(28));

    // Reordena por id para exibição
    concluidos.sort_by_key(|p| p.id);
    let mut soma = 0;
    for p in &concluidos {
        let t = p.tempo_conclusao.unwrap();
        soma += t;
        println!("  {:<6} {}", p.nome, t);
    }
    println!("  {}", "-".repeat(28));
    println!("  Tempo médio de conclusão: {:.1}\n", soma as f32 / concluidos.len() as f32);
}
