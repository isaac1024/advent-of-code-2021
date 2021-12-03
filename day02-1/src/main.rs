use std::fs::File;
use std::io::Read;

struct Command {
    key: String,
    value: i32
}

fn file_to_vector(file: &str) -> Option<Vec<Command>> {
    let mut s = String::new();

    File::open(file).ok()?.read_to_string(&mut s).ok()?;
    Some(s.lines().filter_map(|n| {
        let line: Vec<&str> = n.split(" ").collect();

        Some(Command {
            key: line[0].to_string(),
            value: line[1].parse::<i32>().expect("")
        })
    }).collect())
}

fn resolve(file: &str)-> i32 {
    let commands = file_to_vector(file);

    let mut forward = 0;
    let mut up = 0;
    let mut down = 0;

    match commands {
        Some(commands) => {
            for command in commands {
                match command.key.as_str() {
                    "forward" => forward += command.value,
                    "up" => up += command.value,
                    "down" => down += command.value,
                    _ => {}
                }
            }
        }
        None => panic!("IO Error")
    }

    forward * (up - down).abs()
}

fn main() {
    println!("Resolution: {}", resolve("./input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_is_150() {
        assert_eq!(150, resolve("./example"));
    }
}
