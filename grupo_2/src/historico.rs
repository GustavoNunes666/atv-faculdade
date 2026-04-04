//Historico de pesquisa
pub fn executar() {
    let mut historico_back: Vec<String> = Vec::new();
    let mut historico_forward: Vec<String> = Vec::new();

    let mut atual = String::new();

    println!("Atual: {}", atual);

    //Nova página
    historico_back.push(atual.clone());
    atual = String::new();
    historico_forward.clear();

    println!("Atual: {}", atual);

    //Outra página
    historico_back.push(atual.clone());
    atual = String::new();
    historico_forward.clear();

    println!("Atual: {}", atual);

    //Voltar
    historico_forward.push(atual.clone());
    atual = historico_back.pop().unwrap();

    println!("Voltar → Atual: {}", atual);

    //Avançar
    historico_back.push(atual.clone());
    atual = historico_forward.pop().unwrap();

    println!("Avançar → Atual: {}", atual);
}