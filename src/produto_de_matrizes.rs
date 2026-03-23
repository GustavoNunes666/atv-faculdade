//Condição Big O(n²)
//Porque o algoritmo possui loops aninhados e por estar mutando um vetor, obrigatoriamente as interções vão passar por todos os elementos presentes na matriz.
//Quantidade de loops: 3

pub fn produto_de_matrizes(A: Vec<Vec<i32>>, B: Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
    let mut C = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                C[i][j] += A[i][k] * B[k][j];
            }
        }
    }

    C
}