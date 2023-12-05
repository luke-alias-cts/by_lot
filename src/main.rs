use rand::Rng;
use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let allocated_numbers = allocated_numbers(&args[1..]);

        for (name, number) in allocated_numbers {
            println!("{}: {}", name, number);
        }
    } else {
        let number = generate_random_number(100);
        println!("your random number is {}", number);
    }
}

fn allocated_numbers(names: &[String]) -> Vec<(String, u32)> {
    let mut numbers = HashSet::new();
    let mut results = Vec::new();
    let count = names.len() as u32;

    for name in names {
        let mut num;
        loop {
            num = generate_random_number(count);
            if !numbers.contains(&num) {
                // println!("loop break {}", num);

                break;
            }
        }
        // println!("loop end {}", num);
        numbers.insert(num);
        results.push((name.to_string(), num));
    }

    results
}

fn generate_random_number(count: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=count)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_generate_random_number() {
        let number = super::generate_random_number(100);
        assert!(number > 0 && number <= 100);
    }

    #[test]
    fn test_allocated_numbers() {
        let names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let results = super::allocated_numbers(&names);
        assert_eq!(results.len(), 3);
    }
}
