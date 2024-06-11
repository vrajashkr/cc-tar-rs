pub enum TarMode {
    Create,
    List
}

#[derive(PartialEq)]
pub enum InputSource {
    Stdin,
    File
}

pub struct Config {
    pub mode: TarMode,
    pub block_size: usize,
    pub input_src: InputSource,
    pub input_file: String
}
