use rand::{rng, Rng};

pub fn generate_memory_address() -> String {
    let mut rng = rng();
    let address: String = (0..8)
        .map(|_| format!("{:x}", rng.random_range(0..16)))
        .collect();
    format!("0x{}", address)
}
