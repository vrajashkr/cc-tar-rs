pub enum Mode {
    Create,
    List
}

pub struct Config {
    pub mode: Mode,
    pub block_size: usize
}
