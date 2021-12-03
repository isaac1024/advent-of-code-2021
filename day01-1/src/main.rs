use std::fs::File;
use std::io::Read;

fn file_to_vector(file: &str) -> Option<Vec<u32>> {
    let mut s = String::new();

    File::open(file).ok()?.read_to_string(&mut s).ok()?;
    Some(s.lines().filter_map(|n| n.parse::<u32>().ok()).collect())
}

fn resolve(file: &str)-> u32 {
    let sonar_reports = file_to_vector(file);

    match sonar_reports {
        Some(mut reports) => {
            let mut increases = 0;
            let mut actual_report;
            let mut previous_report = reports.remove(0);

            while reports.iter().count() > 0 {
                actual_report = reports.remove(0);

                if actual_report > previous_report {
                    increases += 1;
                }

                previous_report = actual_report;
            }

            increases
        }
        None => panic!("Error with input file.")
    }
}

fn main() {
    println!("Resolution: {}", resolve("./input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_is_7() {
        assert_eq!(7, resolve("./example"));
    }
}
