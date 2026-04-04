//Pilha minima 
pub fn executar() {
    let mut stack = StackMin::new();

    stack.push(5);
    stack.push(2);
    stack.push(8);
    stack.push(1);

    println!("Min atual: {:?}", stack.min()); // 1

    stack.pop();
    println!("Min atual: {:?}", stack.min()); // 2

    stack.pop();
    println!("Min atual: {:?}", stack.min()); // 2

    stack.pop();
    println!("Min atual: {:?}", stack.min()); // 5
}


struct StackMin {
    pilha: Vec<i32>,
    min_pilha: Vec<i32>,
}

impl StackMin {
    fn new() -> Self {
        Self {
            pilha: Vec::new(),
            min_pilha: Vec::new(),
        }
    }

    fn push(&mut self, valor: i32) {
        self.pilha.push(valor);

        if self.min_pilha.is_empty() || valor <= *self.min_pilha.last().unwrap() {
            self.min_pilha.push(valor);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        let valor = self.pilha.pop()?;

        if valor == *self.min_pilha.last().unwrap() {
            self.min_pilha.pop();
        }

        Some(valor)
    }

    fn min(&self) -> Option<i32> {
        self.min_pilha.last().copied()
    }
}