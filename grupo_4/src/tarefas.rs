use std::collections::VecDeque;

#[derive(Debug)]
enum Prioridade {
    Urgente,
    Normal,
}

struct Tarefa {
    nome: String,
    prioridade: Prioridade,
}

pub fn executar() {
    println!("=== Exercício 16: Fila de Tarefas com Prioridade de Frente ===\n");

    let mut deque: VecDeque<Tarefa> = VecDeque::new();

    let entradas: Vec<(&str, Prioridade)> = vec![
        ("Enviar relatório",       Prioridade::Normal),
        ("Responder e-mail",       Prioridade::Normal),
        ("Servidor caiu!",         Prioridade::Urgente),
        ("Reunião de alinhamento", Prioridade::Normal),
        ("Bug em produção!",       Prioridade::Urgente),
        ("Atualizar documentação", Prioridade::Normal),
        ("Cliente sem acesso!",    Prioridade::Urgente),
    ];

    println!("--- Adicionando tarefas ---");
    for (nome, prioridade) in entradas {
        match prioridade {
            Prioridade::Urgente => {
                println!("  [URGENTE → frente] {}", nome);
                deque.push_front(Tarefa { nome: nome.to_string(), prioridade });
            }
            Prioridade::Normal => {
                println!("  [normal  → fundo ] {}", nome);
                deque.push_back(Tarefa { nome: nome.to_string(), prioridade });
            }
        }
    }

    println!("\n--- Executando tarefas (sempre pela frente) ---");
    let mut ordem = 1;
    while let Some(tarefa) = deque.pop_front() {
        println!(
            "  {}º → [{:?}] {}",
            ordem, tarefa.prioridade, tarefa.nome
        );
        ordem += 1;
    }

    println!("\n  Fila vazia? {}\n", deque.is_empty());
}
