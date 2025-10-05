use rand::{Rng, thread_rng};

pub fn generate_memory_address() -> String {
    let mut rng = thread_rng();
    let address: String = (0..8)
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect();
    format!("0x{}", address)
}
