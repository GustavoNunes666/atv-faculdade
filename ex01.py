# Exercício 1 - verificar_primeiro
def verificar_primeiro(lista):
    if len(lista) == 0:
        return None
    return lista[0]


# Testes
print(verificar_primeiro([10, 20, 30]))  # 10
print(verificar_primeiro([]))            # None
print(verificar_primeiro([99]))          # 99
