use std::fs::File;
use std::io::Read;

fn file_to_vector(file: &str) -> Option<Vec<String>> {
    let mut s = String::new();

    File::open(file).ok()?.read_to_string(&mut s).ok()?;
    Some(s.lines().map(|n| n.to_string()).collect())
}

fn bigger(vector: Vec<String>, index: usize) -> Vec<String> {

    let a: Vec<String> = vector.clone().into_iter().filter(|v| v.chars().nth(index).unwrap().to_string() == "0").collect();
    let b: Vec<String> = vector.clone().into_iter().filter(|v| v.chars().nth(index).unwrap().to_string() == "1").collect();

    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn smaller(vector: Vec<String>, index: usize) -> Vec<String> {
    let a: Vec<String> = vector.clone().into_iter().filter(|v| v.chars().nth(index).unwrap().to_string() == "0").collect();
    let b: Vec<String> = vector.clone().into_iter().filter(|v| v.chars().nth(index).unwrap().to_string() == "1").collect();

    if a.is_empty() {
        return b;
    }

    if b.is_empty() {
        return a;
    }
    if b.len() < a.len() {
        b
    } else {
        a
    }
}

fn vector_to_decimal(code: String) -> i32 {
    let mut index = 0;
    let mut result = 0;
    for c in code.chars() {
        if c.to_string() == "1" {
            result += 2_i32.pow(code.len() as u32 - index - 1)
        }

        index += 1;
    }

    result
}

fn resolve(file: &str)-> i32 {
    let data = file_to_vector(file);

    match data {
        Some(data) => {
            let length = data[0].len();
            let mut index = 0;
            let mut b = data.clone();
            let mut s = data.clone();
            while index < length {
                b = bigger(b, index);
                s = smaller(s, index);

                index += 1;
            }

            vector_to_decimal(b.first().unwrap().clone()) * vector_to_decimal(s.first().unwrap().clone())
        },
        None => panic!("IO Error")
    }
}

fn main() {
    println!("Resolution: {}", resolve("./input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_is_230() {
        assert_eq!(230, resolve("./example"));
    }
}
