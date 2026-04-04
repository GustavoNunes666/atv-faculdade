// Exercício 12 — Buffer de mensagens (FilaCircular)
// Buffer de capacidade fixa que descarta a mensagem mais antiga quando cheio (overwrite).

pub struct FilaCircular {
    buffer: Vec<Option<String>>,
    capacidade: usize,
    inicio: usize, // índice da mensagem mais antiga
    fim: usize,    // índice onde a próxima mensagem será inserida
    tamanho: usize,
}

impl FilaCircular {
    pub fn new(capacidade: usize) -> Self {
        FilaCircular {
            buffer: vec![None; capacidade],
            capacidade,
            inicio: 0,
            fim: 0,
            tamanho: 0,
        }
    }

    /// Insere uma mensagem. Se cheio, sobrescreve a mais antiga.
    pub fn enqueue(&mut self, mensagem: String) {
        if self.tamanho == self.capacidade {
            // Buffer cheio: avança o inicio (descarta a mensagem mais antiga)
            println!(
                "  [OVERWRITE] Buffer cheio! Descartando: \"{}\"",
                self.buffer[self.inicio].as_deref().unwrap_or("?")
            );
            self.inicio = (self.inicio + 1) % self.capacidade;
            self.tamanho -= 1;
        }

        self.buffer[self.fim] = Some(mensagem);
        self.fim = (self.fim + 1) % self.capacidade;
        self.tamanho += 1;
    }

    /// Remove e retorna a mensagem mais antiga.
    pub fn dequeue(&mut self) -> Option<String> {
        if self.tamanho == 0 {
            return None;
        }
        let msg = self.buffer[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        self.tamanho -= 1;
        msg
    }

    pub fn esta_cheio(&self) -> bool {
        self.tamanho == self.capacidade
    }

    pub fn esta_vazio(&self) -> bool {
        self.tamanho == 0
    }

    pub fn tamanho(&self) -> usize {
        self.tamanho
    }

    /// Exibe o estado atual do buffer sem consumir as mensagens.
    pub fn exibir(&self) {
        print!("  Buffer [cap={}]: [", self.capacidade);
        for i in 0..self.tamanho {
            let idx = (self.inicio + i) % self.capacidade;
            if let Some(ref msg) = self.buffer[idx] {
                if i > 0 { print!(", "); }
                print!("\"{}\"", msg);
            }
        }
        println!("] ({}/{})", self.tamanho, self.capacidade);
    }
}

pub fn executar() {
    println!("=== Exercício 12: Buffer de Mensagens (FilaCircular) ===\n");

    let mut buf = FilaCircular::new(4);

    let mensagens = vec![
        "Msg A", "Msg B", "Msg C", "Msg D",
        "Msg E", // deve sobrescrever A
        "Msg F", // deve sobrescrever B
    ];

    println!("--- Inserindo mensagens ---");
    for msg in &mensagens {
        println!("\n  Enfileirando: \"{}\"", msg);
        buf.enqueue(msg.to_string());
        buf.exibir();
    }

    println!("\n--- Consumindo mensagens ---");
    while let Some(msg) = buf.dequeue() {
        println!("  Lida: \"{}\" | restam: {}", msg, buf.tamanho());
    }

    println!("\n  Buffer vazio? {}\n", buf.esta_vazio());
}
