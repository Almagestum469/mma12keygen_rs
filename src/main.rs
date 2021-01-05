use std::io;
use std::vec;
fn main() {
    let magic_number = vec![
        10690, 12251, 17649, 24816, 33360, 35944, 36412, 42041, 42635, 44011, 53799, 56181, 58536,
        59222, 61041,
    ];
    println!("MMA 12.0 keygen");
    println!("Please input your math ID: ");
    let mut math_id = String::new();
    io::stdin()
        .read_line(&mut math_id)
        .expect("Failed to read line");
    println!("Activation Key: 1234-4321-123456");
    println!("Pick any password from below:");
    for h in magic_number.iter() {
        println!(
            "{}",
            gen_password(format!("{}{}", math_id.trim(), "$1&1234-4321-123456"), *h)
        )
    }
    io::stdin().read_line(&mut math_id)
        .expect("Failed to read line");
}

fn f1(mut n: i32, byte: i32, c: i32) -> i32 {
    for bit_index in 0..8 {
        let bit = (byte >> bit_index) & 1;
        if bit + ((n - bit) & !1) == n {
            n = (n - bit) >> 1;
        } else {
            n = ((c - bit) ^ n) >> 1;
        }
    }
    n
}

fn gen_password(str1: String, h: i32) -> String {
    let mut hash = h;
    for byte_index in (0..str1.len()).rev() {
        hash = f1(hash, str1.as_bytes()[byte_index] as i32, 0x105C3);
    }

    let mut n1 = 0;
    while f1(f1(hash, n1 & 0xFF, 0x105C3), n1 >> 8, 0x105C3) != 0xA5B6 {
        n1 = n1 + 1;
        if n1 >= 0xFFFF {
            eprintln!("Failed to find a key!");
            return String::from("");
        }
    }
    n1 = (((n1 + 0x72FA) & 0xFFFF) as f64 * 99999.0 / 0xFFFF as f64) as i32;
    let mut n1str = format!("{}{}", "0000", n1);
    n1str = String::from(&n1str[(n1str.len() - 5)..n1str.len()]);

    let temp_str = format!(
        "{}{}{}",
        &n1str[0..(n1str.len() - 3)],
        &n1str[(n1str.len() - 2)..n1str.len()],
        &n1str[(n1str.len() - 3)..(n1str.len() - 2)]
    );
    let mut temp: i32 = temp_str.parse::<i32>().unwrap();
    temp = (((temp as f64 / 99999.0) * 0xFFFF as f64) + 1.0) as i32;
    temp = f1(f1(0, temp & 0xFF, 0x1064B), temp >> 8, 0x1064B);
    for byte_index in (0..str1.len()).rev() {
        temp = f1(temp, str1.as_bytes()[byte_index] as i32, 0x1064B);
    }

    let mut n2 = 0;
    while f1(f1(temp, n2 & 0xFF, 0x1064B), n2 >> 8, 0x1064B) != 0xA5B6 {
        n2 = n2 + 1;
        if n2 >= 0xFFFF {
            eprintln!("Failed to find a key!");
            return String::from("");
        }
    }
    n2 = (((n2 & 0xFFFF) as f64 * 99999.0) / 0xFFFF as f64) as i32;
    let mut n2str = format!("{}{}", "0000", n2);
    n2str = String::from(&n2str[(n2str.len() - 5)..n2str.len()]);

    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}",
        n2str.chars().nth(3).unwrap(),
        n1str.chars().nth(3).unwrap(),
        n1str.chars().nth(1).unwrap(),
        n1str.chars().nth(0).unwrap(),
        '-',
        n2str.chars().nth(4).unwrap(),
        n1str.chars().nth(2).unwrap(),
        n2str.chars().nth(0).unwrap(),
        '-',
        n2str.chars().nth(2).unwrap(),
        n1str.chars().nth(4).unwrap(),
        n2str.chars().nth(1).unwrap(),
        "::1"
    )
}
