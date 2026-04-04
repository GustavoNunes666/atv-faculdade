use std::collections::VecDeque;

fn é_palindromo(texto: &str) -> bool {
    
    let mut deque: VecDeque<char> = texto
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    
    while deque.len() > 1 {
        if deque.pop_front() != deque.pop_back() {
            return false;
        }
    }
    true
}

pub fn executar() {
    println!("=== Exercício 14: Palíndromo com Deque ===\n");

    let testes = vec![
        "A man a plan a canal Panama",
        "racecar",
        "hello",
        "Was it a car or a cat I saw",
        "Rust",
        "Anotaram a data da maratona",
    ];

    for texto in testes {
        let resultado = if é_palindromo(texto) { "✓ É palíndromo" } else { "✗ Não é palíndromo" };
        println!("  \"{}\"\n   → {}\n", texto, resultado);
    }
}
