use std::io::{self, Write};

fn main() {

    // Read hex input from user
    print!("Enter hexadecimal string: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Remove whitespace and optional "0x" prefix
    let hex_input = input.trim().trim_start_matches("0x").replace(" ", "");

    //let hex_input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let bytes = hex_to_bytes(&hex_input);

    let (key, plaintext, score) = find_single_byte_xor(&bytes);

    println!("Key: 0x{:02x} ('{}')", key, key as char);
    println!("Score: {:.2}", score);
    println!("Plaintext: {}", plaintext);
}

/// Converte uma string hex para bytes.
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

/// Testa todas as 256 chaves possíveis e retorna a de melhor score.
fn find_single_byte_xor(data: &[u8]) -> (u8, String, f64) {
    let mut best_key = 0u8;
    let mut best_plaintext = String::new();
    let mut best_score = f64::NEG_INFINITY;

    for key in 0u8..=255 {
        let candidate: Vec<u8> = data.iter().map(|&b| b ^ key).collect();

        // Só consideramos candidatos que são UTF-8 válido.
        if let Ok(text) = String::from_utf8(candidate) {
            let score = score_english(&text);
            if score > best_score {
                best_score = score;
                best_key = key;
                best_plaintext = text;
            }
        }
    }

    (best_key, best_plaintext, best_score)
}

/// Pontua um texto baseado em frequência de caracteres do inglês.
/// Quanto maior, mais "parecido com inglês".
fn score_english(text: &str) -> f64 {
    let mut score = 0.0;

    for c in text.chars() {
        score += match c.to_ascii_lowercase() {
            'e' => 12.70,
            't' => 9.06,
            'a' => 8.17,
            'o' => 7.51,
            'i' => 6.97,
            'n' => 6.75,
            's' => 6.33,
            'h' => 6.09,
            'r' => 5.99,
            'd' => 4.25,
            'l' => 4.03,
            'c' => 2.78,
            'u' => 2.76,
            'm' => 2.41,
            'w' => 2.36,
            'f' => 2.23,
            'g' => 2.02,
            'y' => 1.97,
            'p' => 1.93,
            'b' => 1.29,
            'v' => 0.98,
            'k' => 0.77,
            'j' => 0.15,
            'x' => 0.15,
            'q' => 0.10,
            'z' => 0.07,
            ' ' => 13.00, // espaço é muito comum em inglês
            _ => -20.0,   // penalidade para qualquer outra coisa
        };
    }

    score
}