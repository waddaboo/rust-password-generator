use rand::{
    distributions::{Distribution, Uniform, WeightedIndex},
    Rng,
};

const ALPHABETS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SYMBOLS: &[char] = &[
    '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '{', '}', '[',
    ']', '|', '\\', ':', ';', '"', '\'', ',', '<', '>', '.', '/', '?',
];

pub fn generate_password<R: Rng>(rng: &mut R, length: u32, numbers: bool, symbols: bool) -> String {
    let mut character_sets = vec![ALPHABETS];

    if numbers {
        character_sets.push(NUMBERS);
    }

    if symbols {
        character_sets.push(SYMBOLS);
    }

    // generate password with weighted distribution based on condition
    let weights = match (numbers, symbols) {
        // 60% alphabets, 20% numbers, 20% symbols
        (true, true) => vec![6, 2, 2],
        // 80% numbers, 20% symbols
        (true, false) | (false, true) => vec![8, 2],
        // 100% alphabets
        (false, false) => vec![10],
    };

    let weighted_dist = WeightedIndex::new(weights).unwrap();

    let mut password = String::with_capacity(length as usize);

    for _ in 0..length {
        let selected_set = character_sets.get(weighted_dist.sample(rng)).unwrap();
        let dist_char = Uniform::from(0..selected_set.len());
        let index = dist_char.sample(rng);

        password.push(selected_set[index]);
    }

    password
}

pub fn generate_pin_number<R: Rng>(rng: &mut R, length: u32) -> String {
    (0..length)
        .map(|_| NUMBERS[rng.gen_range(0..NUMBERS.len())])
        .collect()
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::generate_password::{ALPHABETS, NUMBERS, SYMBOLS};

    use super::{generate_password, generate_pin_number};

    #[test]
    fn test_password() {
        let mut rng = StdRng::seed_from_u64(0);
        let length = 10;

        let alphabets = generate_password(&mut rng, length, false, false);
        assert!(alphabets.chars().all(|x| ALPHABETS.contains(&x)));

        let alphanumeric = generate_password(&mut rng, length, true, false);
        assert!(alphanumeric.chars().any(|x| NUMBERS.contains(&x)));

        let symbols = generate_password(&mut rng, length, false, true);
        assert!(symbols.chars().any(|x| SYMBOLS.contains(&x)));

        let alphanumeric_symbols = generate_password(&mut rng, length, true, true);
        assert!(alphanumeric_symbols
            .chars()
            .any(|x| NUMBERS.contains(&x) || SYMBOLS.contains(&x)));
    }

    #[test]
    fn test_password_length() {
        let mut rng = StdRng::seed_from_u64(0);
        let length = 10;

        let password = generate_password(&mut rng, length, true, true);

        assert_eq!(password.len(), length as usize);
    }

    #[test]
    fn test_pin_number() {
        let mut rng = StdRng::seed_from_u64(0);
        let length = 6;

        let pin = generate_pin_number(&mut rng, length);

        assert!(pin.chars().all(|x| NUMBERS.contains(&x)));
    }

    #[test]
    fn test_pin_number_length() {
        let mut rng = StdRng::seed_from_u64(0);
        let length = 6;

        let pin = generate_pin_number(&mut rng, length);

        assert_eq!(pin.len(), length as usize);
    }
}
