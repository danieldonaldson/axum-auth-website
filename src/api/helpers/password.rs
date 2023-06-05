use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::{distributions::Alphanumeric, Rng};

pub fn hash_password(
    password: String,
    salt: Option<String>,
) -> (String, String) {
    // we could technically do this hashing client side and
    // let them send us the hashed password directly from the browser
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 2,
        lanes: 8,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    // let config = Config::default();
    let salt = salt.unwrap_or(generate_salt(16));
    let hash = argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config)
        .unwrap();
    println!("hash={}\nsalt={}", hash, salt);
    (hash, salt)
}

fn generate_salt(length: usize) -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}
