pub enum TarMode {
    Create,
    List
}

pub struct Config {
    pub mode: TarMode,
    pub block_size: usize
}
