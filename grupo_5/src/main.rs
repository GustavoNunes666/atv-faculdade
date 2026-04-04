mod benchmark;
mod analise;
mod lotes;
mod roundrobin;

fn main() {
    benchmark::executar();
    analise::executar();
    lotes::executar();
    roundrobin::executar();
}
