pub fn convert_kb_to_gb(kb: u64) -> u64 {
    (kb + 1024 * 1024 - 1) / (1024 * 1024)
}

pub fn from_bytes_to_gb(bytes: u64) -> u64 {
    (bytes + 1024 * 1024 * 1024 - 1) / (1024 * 1024 * 1024)
}

pub fn kb_to_gb(kb: u64) -> String {
    format!("{} GB", convert_kb_to_gb(kb))
}

pub fn bytes_to_gb(bytes: u64) -> String {
    format!("{} GB", from_bytes_to_gb(bytes))
}
