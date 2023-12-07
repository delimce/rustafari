pub struct units {
    pub kilobyte: u64,
    pub megabyte: u64,
    pub gigabyte: u64,
    pub terabyte: u64,
    pub petabyte: u64,
    pub exabyte: u64,
    pub zettabyte: u64,
    pub yottabyte: u64,
}

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
