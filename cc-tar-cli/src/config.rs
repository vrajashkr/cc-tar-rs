#[derive(Debug)]
pub enum TarMode {
    Create,
    List,
}

#[derive(PartialEq, Debug)]
pub enum InputSource {
    Stdin,
    File,
}

pub struct Config {
    pub mode: TarMode,
    pub input_src: InputSource,
    pub input_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: TarMode::Create,
            input_src: InputSource::Stdin,
            input_file: "".to_string(),
        }
    }
}
