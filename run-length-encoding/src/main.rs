fn encode(decoded: &str) -> Result<String, &str> {
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
    assert_eq!(Ok(String::from("")), encode(""));
    assert_eq!(Ok(String::from("a5b2c3a2")), encode("aaaaabbcccaa"));
}

fn decode(encoded: &str) -> Result<String, &str> {
    let mut prev_ch_option: Option<char> = None;
    let mut ch_count: u32 = 0;
    let mut decoded: String = String::new();
    for cur_ch in encoded.chars() {
        match prev_ch_option {
            None => {
                prev_ch_option = Some(cur_ch);
            }
            Some(prev_ch) => {
                if cur_ch.is_digit(10) {
                    ch_count = ch_count * 10 + cur_ch.to_digit(10).unwrap();
                } else {
                    decoded.push_str(
                        &std::iter::repeat(prev_ch)
                            .take(ch_count as usize)
                            .collect::<String>(),
                    );
                    prev_ch_option = Some(cur_ch);
                    ch_count = 0;
                }
            }
        }
    }
    match prev_ch_option {
        Some(prev_ch) => {
            decoded.push_str(
                &std::iter::repeat(prev_ch)
                    .take(ch_count as usize)
                    .collect::<String>(),
            );
        }
        None => {}
    }
    Ok(decoded)
}

#[test]
fn test_decode() {
    assert_eq!(Ok(String::from("")), decode(""));
    assert_eq!(Ok(String::from("aaaaabbccca")), decode("a5b2c3a1"));
}

fn main() {
    println!("{}", encode("abcde").unwrap());
    println!("{}", decode("a1b1c2d3e4").unwrap());
}
