//Desfazer - Refazer
pub fn executar() {
    let mut texto = String::from("");
    let mut desfazer: Vec<String> = Vec::new();
    let mut refazer: Vec<String> = Vec::new();

    println!("Texto inicial: '{}'", texto);

    desfazer.push(texto.clone());
    texto = String::from("oi");
    refazer.clear();
    println!("Digitou: '{}'", texto);

    desfazer.push(texto.clone());
    texto = String::from("ola");
    refazer.clear();
    println!("Digitou: '{}'", texto);

    refazer.push(texto.clone());
    texto = desfazer.pop().unwrap();
    println!("Desfazer → '{}'", texto);

    refazer.push(texto.clone());
    texto = desfazer.pop().unwrap();
    println!("Desfazer → '{}'", texto);

    desfazer.push(texto.clone());
    texto = refazer.pop().unwrap();
    println!("Refazer → '{}'", texto);

    desfazer.push(texto.clone());
    texto = refazer.pop().unwrap();
    println!("Refazer → '{}'", texto);
}