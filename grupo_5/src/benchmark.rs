// Exercício 17 — Comparação de desempenho
// Mede o tempo de enqueue/dequeue de 10.000 elementos em três implementações.

use std::collections::VecDeque;
use std::time::Instant;

const N: usize = 10_000;

// --- Fila ingênua com Vec ---
// push_back = push, pop_front = remove(0) → O(n) por operação
struct FilaVec {
    dados: Vec<i32>,
}

impl FilaVec {
    fn new() -> Self { FilaVec { dados: Vec::new() } }
    fn enqueue(&mut self, v: i32) { self.dados.push(v); }
    fn dequeue(&mut self) -> Option<i32> {
        if self.dados.is_empty() { None } else { Some(self.dados.remove(0)) }
    }
}

// --- Fila circular com array fixo ---
struct FilaCircular {
    buffer: Vec<Option<i32>>,
    capacidade: usize,
    inicio: usize,
    fim: usize,
    tamanho: usize,
}

impl FilaCircular {
    fn new(capacidade: usize) -> Self {
        FilaCircular {
            buffer: vec![None; capacidade],
            capacidade,
            inicio: 0,
            fim: 0,
            tamanho: 0,
        }
    }
    fn enqueue(&mut self, v: i32) -> bool {
        if self.tamanho == self.capacidade { return false; }
        self.buffer[self.fim] = Some(v);
        self.fim = (self.fim + 1) % self.capacidade;
        self.tamanho += 1;
        true
    }
    fn dequeue(&mut self) -> Option<i32> {
        if self.tamanho == 0 { return None; }
        let v = self.buffer[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        self.tamanho -= 1;
        v
    }
}

pub fn executar() {
    println!("=== Exercício 17: Comparação de Desempenho ({} elementos) ===\n", N);

    // --- Vec ingênua ---
    let mut fila_vec = FilaVec::new();
    let inicio = Instant::now();
    for i in 0..N { fila_vec.enqueue(i as i32); }
    for _ in 0..N { fila_vec.dequeue(); }
    let tempo_vec = inicio.elapsed();

    // --- VecDeque ---
    let mut fila_deque: VecDeque<i32> = VecDeque::new();
    let inicio = Instant::now();
    for i in 0..N { fila_deque.push_back(i as i32); }
    for _ in 0..N { fila_deque.pop_front(); }
    let tempo_deque = inicio.elapsed();

    // --- Fila circular ---
    let mut fila_circ = FilaCircular::new(N);
    let inicio = Instant::now();
    for i in 0..N { fila_circ.enqueue(i as i32); }
    for _ in 0..N { fila_circ.dequeue(); }
    let tempo_circ = inicio.elapsed();

    println!("  {:<20} {:>15}", "Implementação", "Tempo total");
    println!("  {}", "-".repeat(37));
    println!("  {:<20} {:>15?}", "Vec ingênua",    tempo_vec);
    println!("  {:<20} {:>15?}", "VecDeque",        tempo_deque);
    println!("  {:<20} {:>15?}", "Fila circular",   tempo_circ);

    println!("\n  Conclusão:");
    println!("  - Vec ingênua é O(n²) no total: cada pop_front desloca todos os elementos.");
    println!("  - VecDeque e Fila circular são O(n): operações O(1) amortizadas.\n");
}
