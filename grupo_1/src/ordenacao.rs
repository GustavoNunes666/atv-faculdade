//Ordenação de vetores
pub fn executar(){
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let mut v3 = Vec :: <i32> :: new();

    v3.extend(&v1);
    v3.extend(&v2);

    v3.sort();

    println!("{:?}", v3);
}