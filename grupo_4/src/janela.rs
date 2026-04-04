use std::collections::VecDeque;

fn janela_maxima(vetor: &[i32], k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new(); // guarda índices
    let mut resultado: Vec<i32> = Vec::new();

    for i in 0..vetor.len() {
    
        while let Some(&frente) = deque.front() {
            if frente + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }

    
        while let Some(&fundo) = deque.back() {
            if vetor[fundo] <= vetor[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        
        if i >= k - 1 {
            resultado.push(vetor[*deque.front().unwrap()]);
        }
    }

    resultado
}

pub fn executar() {
    println!("=== Exercício 15: Janela Deslizante Máxima ===\n");

    let testes: Vec<(&str, Vec<i32>, usize)> = vec![
        ("Exemplo básico",  vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        ("Janela de 1",     vec![4, 2, 7, 1, 9],             1),
        ("Janela total",    vec![4, 2, 7, 1, 9],             5),
        ("Valores iguais",  vec![2, 2, 2, 2, 2],             3),
        ("Decrescente",     vec![9, 7, 5, 3, 1],             2),
    ];

    for (nome, vetor, k) in testes {
        let maximos = janela_maxima(&vetor, k);
        println!("  {}", nome);
        println!("  Vetor : {:?}", vetor);
        println!("  k = {} → Máximos: {:?}\n", k, maximos);
    }
}
