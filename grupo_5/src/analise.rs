pub fn executar() {
    println!("=== Exercício 18: Quando Usar Qual TAD? ===\n");

    let cenarios = vec![
        (
            "(a) Implementar o botão \"Ctrl+Z\" de um editor",
            "Pilha (Stack)",
            "Desfazer é LIFO: a última ação feita é a primeira a ser desfeita. \
             Uma pilha modela isso naturalmente com push (ao agir) e pop (ao desfazer).",
        ),
        (
            "(b) Processar pedidos de um restaurante em ordem",
            "Fila (Queue / VecDeque)",
            "Pedidos devem ser atendidos na ordem de chegada — FIFO. \
             VecDeque permite push_back ao receber e pop_front ao servir, ambos O(1).",
        ),
        (
            "(c) Verificar se um arquivo HTML tem tags bem formadas",
            "Pilha (Stack)",
            "Ao abrir uma tag (<div>) empilha; ao fechar (</div>) desempilha e confere. \
             Se a pilha esvaziar no fim sem erros, o HTML está bem formado. Lógica LIFO.",
        ),
        (
            "(d) Navegar nos arquivos de um diretório em largura (BFS)",
            "Fila (Queue / VecDeque)",
            "BFS explora nível a nível: enfileira os filhos de cada nó e processa \
             na ordem de chegada. Fila garante que nós mais rasos sejam visitados primeiro.",
        ),
        (
            "(e) Verificar se uma sequência de palavras é palíndromo",
            "Deque (VecDeque)",
            "O deque permite comparar simultaneamente o primeiro e o último elemento \
             com pop_front e pop_back em O(1), sem precisar de índices ou reversão.",
        ),
    ];

    for (cenario, tad, justificativa) in cenarios {
        println!("  Cenário : {}", cenario);
        println!("  TAD     : {}", tad);
        println!("  Por quê : {}", justificativa);
        println!();
    }
}
