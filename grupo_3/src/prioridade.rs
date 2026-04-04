// Exercício 13 — Fila de prioridade manual
// Itens com prioridade maior saem antes.
// Itens de mesma prioridade seguem ordem FIFO.
// Implementação com busca linear (sem heap).

struct Item {
    valor: String,
    prioridade: u32,
    ordem_insercao: usize, // desempate FIFO
}

pub struct FilaPrioridade {
    itens: Vec<Item>,
    contador: usize,
}

impl FilaPrioridade {
    pub fn new() -> Self {
        FilaPrioridade {
            itens: Vec::new(),
            contador: 0,
        }
    }

    /// Insere um item com a prioridade dada.
    pub fn inserir(&mut self, valor: &str, prioridade: u32) {
        self.itens.push(Item {
            valor: valor.to_string(),
            prioridade,
            ordem_insercao: self.contador,
        });
        self.contador += 1;
    }

    /// Remove e retorna o item de maior prioridade (FIFO se empate).
    /// Busca linear O(n).
    pub fn remover(&mut self) -> Option<String> {
        if self.itens.is_empty() {
            return None;
        }

        // Encontra o índice do item com maior prioridade (menor ordem_insercao no empate)
        let mut idx_melhor = 0;
        for i in 1..self.itens.len() {
            let melhor = &self.itens[idx_melhor];
            let atual = &self.itens[i];

            if atual.prioridade > melhor.prioridade
                || (atual.prioridade == melhor.prioridade
                    && atual.ordem_insercao < melhor.ordem_insercao)
            {
                idx_melhor = i;
            }
        }

        Some(self.itens.remove(idx_melhor).valor)
    }

    pub fn tamanho(&self) -> usize {
        self.itens.len()
    }

    pub fn esta_vazio(&self) -> bool {
        self.itens.is_empty()
    }

    /// Exibe o estado atual da fila (não ordenado, pois a ordenação ocorre na remoção).
    pub fn exibir(&self) {
        print!("  Fila: [");
        for (i, item) in self.itens.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("\"{}\"(p={})", item.valor, item.prioridade);
        }
        println!("]");
    }
}

pub fn executar() {
    println!("=== Exercício 13: Fila de Prioridade Manual ===\n");

    let mut fila = FilaPrioridade::new();

    // Inserção de itens variados
    let entradas = vec![
        ("Tarefa Normal 1",    1),
        ("Tarefa Urgente",     5),
        ("Tarefa Normal 2",    1),
        ("Tarefa Importante",  3),
        ("Emergência",         5), // mesma prioridade que Urgente → deve sair depois (FIFO)
        ("Tarefa Baixa",       0),
        ("Tarefa Média",       3),
    ];

    println!("--- Inserindo itens ---");
    for (nome, prio) in &entradas {
        fila.inserir(nome, *prio);
        println!("  [+] \"{}\" (prioridade {})", nome, prio);
    }

    println!();
    fila.exibir();
    println!();

    println!("--- Removendo em ordem de prioridade ---");
    let mut posicao = 1;
    while let Some(item) = fila.remover() {
        println!("  {}º removido: \"{}\"", posicao, item);
        posicao += 1;
    }

    println!("\n  Fila vazia? {}\n", fila.esta_vazio());
}
