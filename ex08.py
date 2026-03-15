# Exercício 8 - ordenacao_bolha (Bubble Sort)
def ordenacao_bolha(lista):
    n = len(lista)
    for i in range(n):
        for j in range(0, n - i - 1):
            if lista[j] > lista[j + 1]:
                lista[j], lista[j + 1] = lista[j + 1], lista[j]
    return lista


# Testes
print(ordenacao_bolha([64, 34, 25, 12, 22, 11, 90]))  # [11, 12, 22, 25, 34, 64, 90]
print(ordenacao_bolha([1]))                            # [1]
print(ordenacao_bolha([]))                             # []
