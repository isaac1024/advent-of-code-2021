use std::fs::File;
use std::io::Read;

fn file_to_vector(file: &str) -> Option<Vec<u32>> {
    let mut s = String::new();

    File::open(file).ok()?.read_to_string(&mut s).ok()?;
    Some(s.lines().filter_map(|n| n.parse::<u32>().ok()).collect())
}

fn combine(vector: Vec<u32>, combinations: usize) -> Vec<u32> {
    let mut new_vector = Vec::new();
    let mut new_number = Vec::new();

    for v in vector {
        new_number.push(v);

        if new_number.iter().count() > combinations {
            new_number.remove(0);
        }

        if new_number.iter().count() == 3 {
            new_vector.push(new_number.iter().sum())
        }
    }

    new_vector
}

fn resolve(file: &str)-> u32 {
    let sonar_reports = file_to_vector(file);

    match sonar_reports {
        Some(reports) => {
            let mut combination_reports = combine(reports, 3);

            let mut increases = 0;
            let mut actual_report;
            let mut previous_report = combination_reports.remove(0);

            while combination_reports.iter().count() > 0 {
                actual_report = combination_reports.remove(0);

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
    fn resolution_is_5() {
        assert_eq!(5, resolve("./example"));
    }
}
