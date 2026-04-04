//Sequencia de simbolos

pub fn executar() {
    println!("{}", verificar("{[()]}")); // true
    println!("{}", verificar("([)]"));   // false
    println!("{}", verificar("((("));    // false
}

fn verificar(expressao: &str) -> bool {
    let mut pilha: Vec<char> = Vec::new();

    for simbolo in expressao.chars() {
        if simbolo == '(' || simbolo == '[' || simbolo == '{' {
            pilha.push(simbolo);

        } else if simbolo == ')' || simbolo == ']' || simbolo == '}' {
            if pilha.is_empty() {
                return false;
            }

            let topo = pilha.pop().unwrap();

            if simbolo == ')' && topo != '(' {
                return false;
            }
            if simbolo == ']' && topo != '[' {
                return false;
            }
            if simbolo == '}' && topo != '{' {
                return false;
            }
        }
    }

    pilha.is_empty()
}