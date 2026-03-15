# Exercício 7 - fibonacci_recursivo
def fibonacci_recursivo(n):
    if n <= 1:
        return n
    return fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)


# Testes
for i in range(10):
    print(f"fib({i}) = {fibonacci_recursivo(i)}")
