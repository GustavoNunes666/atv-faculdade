// Exercício 10 — Simulador de fila de banco
// Clientes chegam com intervalos aleatórios e são atendidos em ordem.
// Registra o tempo médio de espera.

use std::collections::VecDeque;

struct Cliente {
    id: u32,
    chegada: u32,
}

/// LCG (Linear Congruential Generator) — PRNG simples sem dependências externas.
fn prng(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *seed >> 33
}

pub fn executar() {
    println!("=== Exercício 10: Simulador de Fila de Banco ===\n");

    let mut fila: VecDeque<Cliente> = VecDeque::new();
    let mut seed: u64 = 42;

    let mut tempo: u32 = 0;
    let mut tempo_total_espera: u32 = 0;
    let mut clientes_atendidos: u32 = 0;
    let mut proximo_cliente_id: u32 = 1;

    // Próximo instante em que um novo cliente vai chegar
    let mut proxima_chegada: u32 = prng(&mut seed) as u32 % 4 + 1; // 1 a 4

    let duracao_simulacao: u32 = 30;
    let tempo_atendimento: u32 = 2; // cada cliente leva 2 unidades de tempo
    let mut tempo_livre_atendente: u32 = 0;

    println!("{:<6} {:<30} {:<20}", "Tempo", "Evento", "Fila (tamanho)");
    println!("{}", "-".repeat(58));

    while tempo <= duracao_simulacao {
        // Chegada de cliente
        if tempo == proxima_chegada {
            let cliente = Cliente {
                id: proximo_cliente_id,
                chegada: tempo,
            };
            println!(
                "{:<6} {:<30} {:<20}",
                tempo,
                format!("Cliente {} chegou", cliente.id),
                fila.len() + 1
            );
            fila.push_back(cliente);
            proximo_cliente_id += 1;

            // Agenda próxima chegada com intervalo aleatório de 1 a 5
            let intervalo = prng(&mut seed) as u32 % 5 + 1;
            proxima_chegada = tempo + intervalo;
        }

        // Atendimento: atendente fica livre a partir de tempo_livre_atendente
        if tempo >= tempo_livre_atendente {
            if let Some(cliente) = fila.pop_front() {
                let espera = tempo - cliente.chegada;
                tempo_total_espera += espera;
                clientes_atendidos += 1;
                tempo_livre_atendente = tempo + tempo_atendimento;
                println!(
                    "{:<6} {:<30} {:<20}",
                    tempo,
                    format!(
                        "Cliente {} atendido (espera: {})",
                        cliente.id, espera
                    ),
                    fila.len()
                );
            }
        }

        tempo += 1;
    }

    println!("{}", "-".repeat(58));
    println!("\nClientes atendidos : {}", clientes_atendidos);
    println!("Clientes na fila   : {} (não atendidos)", fila.len());

    if clientes_atendidos > 0 {
        let media = tempo_total_espera as f32 / clientes_atendidos as f32;
        println!("Tempo médio de espera: {:.2} unidades de tempo", media);
    } else {
        println!("Nenhum cliente foi atendido.");
    }

    println!();
}
