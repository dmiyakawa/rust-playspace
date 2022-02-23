fn encode(decoded: &str) -> Result<String, &str> {
    if decoded.len() == 0 {
        return Err("Empty string given");
    }
    let mut prev_ch_option: Option<char> = None;
    let mut ch_count: u32 = 0;
    let mut encoded: String = String::new();

    for cur_ch in decoded.chars() {
        match prev_ch_option {
            None => {
                prev_ch_option = Some(cur_ch);
                ch_count = 1;
            }
            Some(prev_ch) => {
                if prev_ch == cur_ch {
                    ch_count += 1;
                } else {
                    encoded.push(prev_ch);
                    encoded.push_str(&ch_count.to_string());
                    prev_ch_option = Some(cur_ch);
                    ch_count = 1;
                }
            }
        }
    }
    match prev_ch_option {
        Some(prev_ch) => {
            assert!(ch_count > 0);
            encoded.push(prev_ch);
            encoded.push_str(&ch_count.to_string());
        }
        None => {}
    }

    Ok(encoded)
}

#[test]
fn test_encode() {
    assert!(encode("").is_err());
    assert_eq!(Ok(String::from("a5b2c3a2")), encode("aaaaabbcccaa"));
}

fn main() {
    let encoded = match encode("abcde") {
        Ok(value) => value,
        Err(msg) => {
            panic!("Error: {:?}", msg);
        }
    };
    println!("{}", encoded);
}
