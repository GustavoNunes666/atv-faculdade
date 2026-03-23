//Condição Big O(2^n)
//Porque o valor será dobrado a cada interção enquanto o valor for menor que n.
//Quantidade de loops: 1

pub fn potencias_de_dois(n: u64) { 
    let mut i: u64 = 1; 
    while i < n { 
        println!("{}", i);        
         i *= 2;    
     } 
} 