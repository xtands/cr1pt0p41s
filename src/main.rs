use std::io::{self, Write};

fn main() {
    // Read hex input from user
    print!("Enter hexadecimal string: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Remove whitespace and optional "0x" prefix
    let hex_input = input.trim().trim_start_matches("0x").replace(" ", "");
    
    // Validate that the input is valid hex
    if hex_input.len() % 2 != 0 {
        eprintln!("Error: Hex string must have an even number of digits");
        std::process::exit(1);
    }
    
    if !hex_input.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("Error: Invalid hexadecimal character found");
        std::process::exit(1);
    }
    
    // Convert hex to bytes
    let bytes: Vec<u8> = (0..hex_input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_input[i..i + 2], 16).unwrap())
        .collect();
    
    // Convert bytes to base64
    let base64_output = base64_encode(&bytes);
    
    println!("Base64: {}", base64_output);
}

/// Encode bytes to base64 without external dependencies
fn base64_encode(input: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        
        let n = (b0 << 16) | (b1 << 8) | b2;
        
        result.push(CHARSET[((n >> 18) & 0x3F) as usize] as char);
        result.push(CHARSET[((n >> 12) & 0x3F) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(CHARSET[((n >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARSET[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hex_to_base64() {
        // "Hello" in hex -> "SGVsbG8="
        let hex = "48656c6c6f";
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect();
        assert_eq!(base64_encode(&bytes), "SGVsbG8=");
        
        // "Man" -> "TWFu"
        assert_eq!(base64_encode(b"Man"), "TWFu");
        
        // "Ma" -> "TWE="
        assert_eq!(base64_encode(b"Ma"), "TWE=");
        
        // "M" -> "TQ=="
        assert_eq!(base64_encode(b"M"), "TQ==");
    }
}