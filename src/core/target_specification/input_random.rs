use rand::{thread_rng, Rng};

pub fn generate_random_ips(n: u64) -> Vec<String> {
    let mut rng = thread_rng();
    (0..n).map(|_| {
        (0..4).map(|i| rng.gen_range((i == 0) as i32..256).to_string())
            .collect::<Vec<String>>()
            .join(".")
    }).collect::<Vec<String>>()
}