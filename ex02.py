# Exercício 2 - somar_lista
def somar_lista(lista):
    total = 0
    for elemento in lista:
        total += elemento
    return total


# Testes
print(somar_lista([1, 2, 3, 4, 5]))  # 15
print(somar_lista([]))               # 0
print(somar_lista([100]))            # 100
