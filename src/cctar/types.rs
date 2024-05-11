pub struct Archive {
    pub contents: Vec<ArchivedFile>
}

pub struct ArchivedFile {
    pub file_name: String,
    pub file_size: usize,
    pub file_name_prefix: String
}
