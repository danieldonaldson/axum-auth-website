use argon2::{self, Config, ThreadMode, Variant, Version};

use super::SALT;

pub fn hash_password(password: String) -> String {
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
    let hash = argon2::hash_encoded(password.as_ref(), SALT.as_ref(), &config)
        .unwrap();
    hash
}
