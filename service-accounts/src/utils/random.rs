use rand::{thread_rng, Rng};

pub fn get_random_string(length: usize) -> String {
    let characters: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let mut rng = thread_rng();
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters[idx] as char
        })
        .collect();

    random_string
}

pub fn get_random_characters(length: usize) -> String {
    let characters: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

    let mut rng = thread_rng();
    let random_characters: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters[idx] as char
        })
        .collect();

    random_characters
}

pub fn get_random_numbers(length: usize) -> String {
    let mut rng = thread_rng();
    let random_numbers: String = (0..length)
        .map(|_| rng.gen_range(0..9).to_string())
        .collect();

    random_numbers
}
