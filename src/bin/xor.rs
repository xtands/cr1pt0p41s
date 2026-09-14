use std::io::{self, Write};

fn main() {
    // Lê a primeira string
    print!("Digite a primeira string: ");
    io::stdout().flush().unwrap();
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).expect("Falha ao ler input");
    let s1 = s1.trim_end_matches(['\r', '\n']);

    // Lê a segunda string
    print!("Digite a segunda string: ");
    io::stdout().flush().unwrap();
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).expect("Falha ao ler input");
    let s2 = s2.trim_end_matches(['\r', '\n']);

    // Faz XOR byte a byte
    let result = xor_strings(s1, s2);

    // Mostra resultado em hex
    let hex: String = result.iter().map(|b| format!("{:02x}", b)).collect();
    println!("\nXOR (hex):    {}", hex);

    // Mostra resultado como texto (se for imprimível)
    let text = String::from_utf8_lossy(&result);
    println!("XOR (texto):  {}", text);

    // Mostra resultado como bytes decimais
    println!("XOR (bytes):  {:?}", result);
}

fn xor_strings(a: &str, b: &str) -> Vec<u8> {
    a.bytes()
        .zip(b.bytes())
        .map(|(x, y)| x ^ y)
        .collect()
}