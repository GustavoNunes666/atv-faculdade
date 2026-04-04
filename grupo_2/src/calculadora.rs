//Calculadora RPN
pub fn executar() {
    let expr = "3 4 + 2 *";

    let mut pilha: Vec<f64> = Vec::new();

    for item in expr.split_whitespace() {

        // se for número
        if item.parse::<f64>().is_ok() {
            let numero = item.parse::<f64>().unwrap();
            pilha.push(numero);
        } 
        // se for operador
        else {
            let b = pilha.pop().unwrap();
            let a = pilha.pop().unwrap();

            let resultado = if item == "+" {
                a + b
            } else if item == "-" {
                a - b
            } else if item == "*" {
                a * b
            } else if item == "/" {
                a / b
            } else {
                panic!("Operador inválido");
            };

            pilha.push(resultado);
        }
    }

    let resultado_final = pilha.pop().unwrap();

    println!("Resultado: {}", resultado_final);
}