use hex;
use std::io::{self, Write};
use std::str;

//Future Update
fn _smallest_repeating_pattern(data: &[u8]) -> Vec<u8> {
    for len in 1..=data.len() {
        let pattern = &data[..len];
        if data
            .iter()
            .enumerate()
            .all(|(i, &b)| b == pattern[i % len])
        {
            return pattern.to_vec();
        }
    }
    data.to_vec()
}

fn ask(msg: &str) -> String {
    let mut s = String::new();
    print!("{msg}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}


fn main() {
    // Pedir entradas al usuario
    let encrypted_hex = ask("Ingresa encrypted_hex: ");
    let known_text_input = ask("Ingresa known_text: ");

    if encrypted_hex.is_empty() || known_text_input.is_empty() {
        eprintln!("encrypted_hex y known_text no pueden estar vacios.");
        main();
    }

    // Convertir a data que se puede trabajar
    let known_text = known_text_input.as_bytes();
    let encrypted_bytes = match hex::decode(&encrypted_hex) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("encrypted_hex no es un hexadecimal valido.");
            return;
        }
    };

    if encrypted_bytes.len() < known_text.len() {
        eprintln!(
            "El encrypted_hex es muy corto: necesita al menos {} bytes para known_text.",
            known_text.len()
        );
        return;
    }

    // Mismo tamaño de bloque
    let relevant_bytes = &encrypted_bytes[..known_text.len()];

    // Representación hex prq sino sale feo
    let hex_repr: String = encrypted_bytes
        .iter()
        .map(|b| format!("\\x{:02x}", b))
        .collect();

    // Xor de bytes
    let key: Vec<u8> = relevant_bytes
        .iter()
        .zip(known_text.iter())
        .map(|(enc_byte, known_byte)| enc_byte ^ known_byte)
        .collect();


    println!(
        "Bytes: {:?}\nHex: {}\nLa llave es: {:?}",
        hex_repr,
        encrypted_hex,
        str::from_utf8(&key).expect("Error en convertir a texto.")
    );
}
