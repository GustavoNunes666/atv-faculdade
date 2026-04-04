mod queue;
mod impressora;
mod buffer;
mod prioridade;

fn main() {
    queue::executar();
    impressora::executar();
    buffer::executar();
    prioridade::executar();
}
