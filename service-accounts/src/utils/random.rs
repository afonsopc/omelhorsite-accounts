use crate::{prelude::*, utils::config::AppConfig};
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

pub fn get_random_numbers(length: usize) -> String {
    let mut rng = thread_rng();
    let random_numbers: String = (0..length)
        .map(|_| rng.gen_range(0..9).to_string())
        .collect();

    random_numbers
}

fn get_random_color() -> String {
    let colors = [
        "\x1b[1;91m",
        "\x1b[1;92m",
        "\x1b[1;93m",
        "\x1b[1;94m",
        "\x1b[1;95m",
        "\x1b[1;96m",
    ];
    let mut rng = rand::thread_rng();
    let indice = rng.gen_range(0..colors.len());
    colors[indice].to_string()
}

pub fn get_random_process_id() -> String {
    let app_config = AppConfig::load_from_env().unwrap();
    let process_id_length = app_config.process_id_length;
    let color = get_random_color();
    let process_id = get_random_string(process_id_length);

    f!("{color}{process_id}\x1b[0m")
}
