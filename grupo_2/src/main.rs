mod calculadora;
mod historico;
mod desfazer;
mod sequencia;
mod pilha_minimo;

fn main() {
    calculadora::executar();
    sequencia::executar();
    historico::executar();
    desfazer::executar();
    pilha_minimo::executar();
}
