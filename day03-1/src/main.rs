use std::fs::File;
use std::io::Read;

fn file_to_vector(file: &str) -> Option<Vec<String>> {
    let mut s = String::new();

    File::open(file).ok()?.read_to_string(&mut s).ok()?;
    Some(s.lines().map(|n| n.to_string()).collect())
}

fn reverse(vector: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for v in vector {
        if v == 1 {
            result.push(0)
        } else {
            result.push(1)
        }
    }

    result
}

fn vector_to_decimal(mut vector: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut result = 0;
    while !vector.is_empty() {
        let v = vector.pop().unwrap();

        if v == 1 {
            result += 2_i32.pow(index)
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
            let mut result = Vec::new();
            while index < length {
                let mut v = 0;
                for d in &data {
                    if d.chars().nth(index).unwrap().to_string() == "1" {
                        v += 1;
                    } else {
                        v -= 1;
                    }
                }
                if v > 0 {
                    result.push(1)
                } else {
                    result.push(0)
                }
                index += 1;
            }

            let reverse_result = reverse(result.clone());

            vector_to_decimal(result) * vector_to_decimal(reverse_result)
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
    fn resolution_is_198() {
        assert_eq!(198, resolve("./example"));
    }
}
