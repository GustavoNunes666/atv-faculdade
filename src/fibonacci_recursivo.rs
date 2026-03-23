//Condição Big O(2ⁿ)
//O aloritmo usa uma recurção de fibonacci, o que quer dizer que a cada entrada o número de operações dobrará de acordo com o valor informado.
//Quantidade de loops: 0

pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}