# Atividade — Reescrita de Algoritmos em Rust

**Disciplina:** Estruturas de Dados e Análise de Algoritmos  
**Professor:** Alexandre de Oliveira

---

## Exercício 1 — Verificar Primeiro

**Complexidade:** O(1)

**Lógica do algoritmo:**  
Retorna o primeiro elemento de uma lista como `Option<i32>`. Se a lista estiver vazia, retorna `None`; caso contrário, retorna `Some(primeiro_elemento)`.

**Justificativa da complexidade:**  
Por não ter loops presentes, o algoritmo será executado de forma constante, tendo seu tempo de execução independente da quantidade de elementos.

---

## Exercício 2 — Somar Lista

**Complexidade:** O(n)

**Lógica do algoritmo:**  
Percorre todos os elementos da lista uma única vez, acumulando a soma em uma variável `total` que é retornada ao final.

**Justificativa da complexidade:**  
O tempo de execução depende de um loop único sobre o número de elementos. Quanto maior a lista, mais tempo levará proporcionalmente.

---

## Exercício 3 — Busca Binária

**Complexidade:** O(log n)

**Lógica do algoritmo:**  
Divide o intervalo de busca ao meio a cada iteração, descartando metade dos elementos restantes. Compara o elemento do meio com o alvo e ajusta os ponteiros esquerdo ou direito até encontrar o elemento ou esgotar a lista.

**Justificativa da complexidade:**  
O algoritmo será executado partindo a quantidade de elementos ao meio e seguirá dessa forma até que o resultado esperado seja encontrado. Para n = 1.000.000, isso representa apenas ~20 iterações.

---

## Exercício 4 — Pares com Soma

**Complexidade:** O(n²)

**Lógica do algoritmo:**  
O algoritmo procura dois elementos na lista e soma os mesmos, fazendo com que toda a lista seja percorrida até os elementos serem localizados.

**Justificativa da complexidade:**  
Possui dois loops aninhados: o externo percorre cada elemento e o interno percorre os elementos seguintes. O total de comparações é aproximadamente n×(n-1)/2, pertencendo à classe O(n²).

---

## Exercício 5 — Imprimir Pares e Pares

**Complexidade:** O(n²)

**Lógica do algoritmo:**  
Possui dois blocos sequenciais: o primeiro percorre a lista e imprime cada elemento individualmente; o segundo usa dois loops aninhados para imprimir todos os pares (i, j), incluindo (i, i).

**Justificativa da complexidade:**  
O algoritmo possui loops aninhados e cada elemento é comparado com os outros elementos presentes. Pela regra da soma O(n) + O(n²) = O(n²), pois o termo de maior grau domina.

---

## Exercício 6 — Potências de Dois

**Complexidade:** O(log n)

**Lógica do algoritmo:**  
Começa com `i = 1` e dobra o valor a cada iteração (`i *= 2`), imprimindo cada potência enquanto `i < n`.

**Justificativa da complexidade:**  
O valor será dobrado a cada iteração enquanto for menor que n. Como i cresce exponencialmente (1, 2, 4, 8...), o loop executa apenas log₂(n) vezes.

---

## Exercício 7 — Fibonacci Recursivo

**Complexidade:** O(2ⁿ)

**Lógica do algoritmo:**  
Calcula o n-ésimo número de Fibonacci por recursão. Para cada chamada com n > 1, faz duas chamadas recursivas: `fibonacci(n-1)` e `fibonacci(n-2)`. O caso base é n ≤ 1.

**Justificativa da complexidade:**  
O algoritmo usa uma recursão de Fibonacci, o que quer dizer que a cada entrada o número de operações dobrará de acordo com o valor informado. Para n = 40, já são mais de 1 bilhão de operações.

---

## Exercício 8 — Ordenação Bolha (Bubble Sort)

**Complexidade:** O(n²)

**Lógica do algoritmo:**  
Percorre a lista repetidamente comparando elementos adjacentes e trocando-os se estiverem fora de ordem. A cada passagem completa, o maior elemento se posiciona corretamente no final.

**Justificativa da complexidade:**  
O algoritmo possui dois loops aninhados e passa por todos os elementos da lista n vezes para poder ordenar os elementos. O total de comparações é n×(n-1)/2 no pior caso.

---

## Exercício 9 — Produto de Matrizes

**Complexidade:** O(n³)

**Lógica do algoritmo:**  
Calcula o produto de duas matrizes n×n usando três loops aninhados. Para cada posição (i, j) da matriz resultado, acumula o produto ponto a ponto da linha i de A com a coluna j de B.

**Justificativa da complexidade:**  
O algoritmo possui três loops aninhados e, por estar mutando um vetor, obrigatoriamente as iterações vão passar por todos os elementos presentes na matriz, resultando em n × n × n = n³ operações.

---

## Exercício 10 — Merge Sort

**Complexidade:** O(n log n)

**Lógica do algoritmo:**  
Divide recursivamente a lista ao meio até obter sublistas de tamanho 1, depois as funde em ordem crescente. A fusão percorre as duas metades em paralelo, copiando o menor elemento a cada passo.

**Justificativa da complexidade:**  
O algoritmo divide todos os elementos pela metade (log n divisões) e processa todos os novos resultados para dar origem a uma nova lista ordenada (n operações por nível). Multiplicando: n × log n = O(n log n).
