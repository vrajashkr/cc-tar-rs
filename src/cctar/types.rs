pub struct Archive {
    pub contents: Vec<ArchivedFile>
}

#[derive(PartialEq, Debug)]
pub enum ArchivedFileType {
    NormalFile,
    HardLink,
    SymbolicLink,
    CharacterSpecial,
    BlockSpecial,
    Directory,
    Fifo,
    Unknown
}

#[derive(Debug)]
pub struct ArchivedFile {
    pub file_name: String,
    pub file_size: usize,
    pub file_name_prefix: String,
    pub file_type: ArchivedFileType
}

// states for the tar state machine
#[derive(PartialEq, Debug)]
pub enum TarSmState {
    FileHeaderRead,
    FileContentsRead,
    TerminalBlockRead,
    FileHeaderWrite,
    FileContentsWrite,
    TerminalBlockWrite,
    EndOfArchive
}

// state machine representing archive processing
#[derive(Debug)]
pub struct TarStateMachine {
    pub state: TarSmState,
    pub num_blocks_loaded: usize,
    pub current_block_num: usize,
    pub current_file_blocks_remaining: usize
}
