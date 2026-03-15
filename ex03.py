# Exercício 3 - busca_binaria
def busca_binaria(lista, alvo):
    esquerda, direita = 0, len(lista) - 1
    while esquerda <= direita:
        meio = (esquerda + direita) // 2
        if lista[meio] == alvo:
            return meio
        elif lista[meio] < alvo:
            esquerda = meio + 1
        else:
            direita = meio - 1
    return -1


# Testes (lista deve estar ordenada)
print(busca_binaria([1, 3, 5, 7, 9, 11], 7))   # 3
print(busca_binaria([1, 3, 5, 7, 9, 11], 1))   # 0
print(busca_binaria([1, 3, 5, 7, 9, 11], 10))  # -1
