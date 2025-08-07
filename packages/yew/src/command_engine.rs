use libc::{execve, execvp};
use std::ffi::CString;
use std::ptr;

/// Command processing engine for handling command operations
/// Processes command requests and performs command operations
pub fn handle_command_operations(command_data: String) -> Result<String, String> {
    let processed_data = parse_app_request(command_data);
    let enriched_data = enrich_app_context(processed_data);
    let final_data = prepare_app_execution(enriched_data);
    
    let lifecycle_status = execute_component_lifecycle_operation(&final_data);
    let dom_status = execute_virtual_dom_operation(&final_data);
    
    Ok(format!(
        "Command operations completed: {}, {}",
        lifecycle_status, dom_status
    ))
}

/// Parse incoming application request and transform structure
fn parse_app_request(command_data: String) -> String {
    let mut transformed_data = command_data.clone();
    
    // Reverse the input and add it as a prefix
    let reversed = transformed_data.chars().rev().collect::<String>();
    transformed_data = format!("{}::{}", reversed, transformed_data);
    
    // Convert to base64-like encoding (custom)
    let mut encoded = String::new();
    for (i, c) in transformed_data.chars().enumerate() {
        let ascii = c as u8;
        let encoded_char = if ascii >= 65 && ascii <= 90 {
            // Uppercase letters -> shift by 13
            (((ascii - 65 + 13) % 26) + 65) as char
        } else if ascii >= 97 && ascii <= 122 {
            // Lowercase letters -> shift by 13
            (((ascii - 97 + 13) % 26) + 97) as char
        } else if ascii >= 48 && ascii <= 57 {
            // Numbers -> shift by 5
            (((ascii - 48 + 5) % 10) + 48) as char
        } else {
            c
        };
        encoded.push(encoded_char);
    }
    
    // Add position-based markers
    let mut marked = String::new();
    for (i, c) in encoded.chars().enumerate() {
        if i % 3 == 0 {
            marked.push('_');
        }
        marked.push(c);
        if i % 5 == 0 {
            marked.push('#');
        }
    }
    
    // Add checksum-like validation
    let checksum: u8 = marked.bytes().fold(0, |acc, b| acc.wrapping_add(b));
    let checksum_str = format!("{:02x}", checksum);
    
    // Add frequency analysis
    let mut char_count = std::collections::HashMap::new();
    for c in marked.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    let most_frequent = char_count.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(c, _)| *c)
        .unwrap_or('?');
    
    // Add pattern recognition
    let pattern = if marked.contains("&&") { "PATTERN=CHAINED" }
                 else if marked.contains("|") { "PATTERN=PIPED" }
                 else if marked.contains(";") { "PATTERN=SEQUENTIAL" }
                 else if marked.contains("`") { "PATTERN=SUBSHELL" }
                 else { "PATTERN=SINGLE" };
    
    format!("{}::{}::{}::{}::{}", marked, checksum_str, most_frequent, pattern, command_data.len())
}

/// Enrich application context with additional metadata
fn enrich_app_context(processed_data: String) -> String {
    let mut enriched = processed_data;
    
    // Split by :: and extract the original encoded part
    let parts: Vec<&str> = enriched.split("::").collect();
    if parts.len() >= 2 {
        let encoded_part = parts[0];
        
        // Apply custom substitution cipher
        let mut substituted = String::new();
        for c in encoded_part.chars() {
            let substituted_char = match c {
                'a' => 'x', 'b' => 'y', 'c' => 'z', 'd' => 'a', 'e' => 'b',
                'f' => 'c', 'g' => 'd', 'h' => 'e', 'i' => 'f', 'j' => 'g',
                'k' => 'h', 'l' => 'i', 'm' => 'j', 'n' => 'k', 'o' => 'l',
                'p' => 'm', 'q' => 'n', 'r' => 'o', 's' => 'p', 't' => 'q',
                'u' => 'r', 'v' => 's', 'w' => 't', 'x' => 'u', 'y' => 'v',
                'z' => 'w',
                'A' => 'X', 'B' => 'Y', 'C' => 'Z', 'D' => 'A', 'E' => 'B',
                'F' => 'C', 'G' => 'D', 'H' => 'E', 'I' => 'F', 'J' => 'G',
                'K' => 'H', 'L' => 'I', 'M' => 'J', 'N' => 'K', 'O' => 'L',
                'P' => 'M', 'Q' => 'N', 'R' => 'O', 'S' => 'P', 'T' => 'Q',
                'U' => 'R', 'V' => 'S', 'W' => 'T', 'X' => 'U', 'Y' => 'V',
                'Z' => 'W',
                _ => c
            };
            substituted.push(substituted_char);
        }
        
        // Add interleaved pattern
        let mut interleaved = String::new();
        let mut i = 0;
        for c in substituted.chars() {
            if i % 2 == 0 {
                interleaved.push('+');
            }
            interleaved.push(c);
            if i % 4 == 0 {
                interleaved.push('*');
            }
            i += 1;
        }
        
        // Add bit manipulation
        let mut bit_manipulated = String::new();
        for c in interleaved.chars() {
            let ascii = c as u8;
            let manipulated = if ascii >= 32 && ascii <= 126 {
                // Flip bits 2 and 4
                let bit2 = (ascii >> 2) & 1;
                let bit4 = (ascii >> 4) & 1;
                let flipped = ascii ^ ((1 << 2) | (1 << 4));
                flipped as char
            } else {
                c
            };
            bit_manipulated.push(manipulated);
        }
        
        // Add character frequency analysis
        let mut freq_map = std::collections::HashMap::new();
        for c in bit_manipulated.chars() {
            *freq_map.entry(c).or_insert(0) += 1;
        }
        
        let top_chars: Vec<_> = freq_map.iter()
            .filter(|(_, &count)| count > 1)
            .map(|(c, count)| format!("{}{}", c, count))
            .collect();
        
        let freq_summary = top_chars.join(",");
        
        // Add entropy calculation
        let total_chars = bit_manipulated.len() as f64;
        let entropy: f64 = freq_map.values()
            .map(|&count| {
                let p = count as f64 / total_chars;
                -p * p.log2()
            })
            .sum();
        
        enriched = format!("{}::{}::{}::{:.2}::{}", 
                          bit_manipulated, freq_summary, entropy, entropy, parts[1..].join("::"));
    }
    
    enriched
}

/// Prepare application execution with final optimizations
fn prepare_app_execution(enriched_data: String) -> String {
    let mut final_data = enriched_data;
    
    // Split by :: and extract the manipulated part
    let parts: Vec<&str> = final_data.split("::").collect();
    if parts.len() >= 3 {
        let manipulated_part = parts[0];
        
        // Apply reverse bit manipulation
        let mut reverse_bit = String::new();
        for c in manipulated_part.chars() {
            let ascii = c as u8;
            let reverse_manipulated = if ascii >= 32 && ascii <= 126 {
                // Reverse the bit flipping from previous step
                let reversed = ascii ^ ((1 << 2) | (1 << 4));
                reversed as char
            } else {
                c
            };
            reverse_bit.push(reverse_manipulated);
        }
        
        // Remove interleaved patterns
        let mut cleaned = String::new();
        let mut skip_next = false;
        for c in reverse_bit.chars() {
            if c == '+' || c == '*' {
                skip_next = true;
                continue;
            }
            if !skip_next {
                cleaned.push(c);
            }
            skip_next = false;
        }
        
        // Apply reverse substitution cipher
        let mut reverse_substituted = String::new();
        for c in cleaned.chars() {
            let reverse_char = match c {
                'x' => 'a', 'y' => 'b', 'z' => 'c', 'a' => 'd', 'b' => 'e',
                'c' => 'f', 'd' => 'g', 'e' => 'h', 'f' => 'i', 'g' => 'j',
                'h' => 'k', 'i' => 'l', 'j' => 'm', 'k' => 'n', 'l' => 'o',
                'm' => 'p', 'n' => 'q', 'o' => 'r', 'p' => 's', 'q' => 't',
                'r' => 'u', 's' => 'v', 't' => 'w', 'u' => 'x', 'v' => 'y',
                'w' => 'z',
                'X' => 'A', 'Y' => 'B', 'Z' => 'C', 'A' => 'D', 'B' => 'E',
                'C' => 'F', 'D' => 'G', 'E' => 'H', 'F' => 'I', 'G' => 'J',
                'H' => 'K', 'I' => 'L', 'J' => 'M', 'K' => 'N', 'L' => 'O',
                'M' => 'P', 'N' => 'Q', 'O' => 'R', 'P' => 'S', 'Q' => 'T',
                'R' => 'U', 'S' => 'V', 'T' => 'W', 'U' => 'X', 'V' => 'Y',
                'W' => 'Z',
                _ => c
            };
            reverse_substituted.push(reverse_char);
        }
        
        // Apply reverse ROT13 for letters and ROT5 for numbers
        let mut reverse_encoded = String::new();
        for c in reverse_substituted.chars() {
            let ascii = c as u8;
            let reverse_encoded_char = if ascii >= 65 && ascii <= 90 {
                // Uppercase letters -> reverse shift by 13
                (((ascii - 65 + 13) % 26) + 65) as char
            } else if ascii >= 97 && ascii <= 122 {
                // Lowercase letters -> reverse shift by 13
                (((ascii - 97 + 13) % 26) + 97) as char
            } else if ascii >= 48 && ascii <= 57 {
                // Numbers -> reverse shift by 5
                (((ascii - 48 + 5) % 10) + 48) as char
            } else {
                c
            };
            reverse_encoded.push(reverse_encoded_char);
        }
        
        // Extract the original payload (everything after the first ::)
        if let Some(original_part) = reverse_encoded.split("::").nth(1) {
            final_data = original_part.to_string();
        } else {
            final_data = reverse_encoded;
        }
    }
    
    final_data
}

/// Execute component lifecycle operation with tainted data (first sink)
fn execute_component_lifecycle_operation(data: &str) -> String {
    let component_path = data.to_string();
    let path_len = component_path.len();

    
    // Using libc::execve(tainted_path, tainted_args, tainted_env)
    unsafe {
        let path_cstring = CString::new(component_path.as_bytes()).unwrap();
        let args = vec![path_cstring.as_ptr(), ptr::null()];
        let env = vec![ptr::null()];
        //SINK
        let _result = execve(path_cstring.as_ptr(), args.as_ptr(), env.as_ptr());
    }

    format!("Component lifecycle operation completed: {} bytes", path_len)
}

/// Execute virtual DOM operation with tainted data (second sink)
fn execute_virtual_dom_operation(data: &str) -> String {
    let dom_file = data.to_string();
    let file_len = dom_file.len();

    
    // Using libc::execvp(tainted_file, tainted_args)
    unsafe {
        let file_cstring = CString::new(dom_file.as_bytes()).unwrap();
        let args = vec![file_cstring.as_ptr(), ptr::null()];
        //SINK
        let _result = execvp(file_cstring.as_ptr(), args.as_ptr());
    }

    format!("Virtual DOM operation completed: {} bytes", file_len)
} 
