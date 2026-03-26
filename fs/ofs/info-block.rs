// fs/ofs/info_block.rs

#[derive(Debug, Clone)]
pub enum FileType {
    Binary,
    Package,   // .titn
    File,
    Directory,
}

#[derive(Debug, Clone, Copy)]
pub enum PermissionLevel {
    None,        // 0
    Read,        // 1
    Write,       // 2
    Execute,     // 3
    ReadWrite,   // 4
    ReadExecute, // 5
    WriteExecute,// 6
    ReadWriteExecute, // 7
}

impl PermissionLevel {
    pub fn to_octal(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Read => 1,
            Self::Write => 2,
            Self::Execute => 3,
            Self::ReadWrite => 4,
            Self::ReadExecute => 5,
            Self::WriteExecute => 6,
            Self::ReadWriteExecute => 7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Permissions {
    pub owner: PermissionLevel,
    pub group: PermissionLevel,
    pub others: PermissionLevel,
}

impl Permissions {
    /// Retorna formato tipo "750"
    pub fn to_octal_string(&self) -> String {
        format!(
            "{}{}{}",
            self.owner.to_octal(),
            self.group.to_octal(),
            self.others.to_octal()
        )
    }
}

#[derive(Debug, Clone)]
pub struct Timestamps {
    pub created: u64,
    pub modified: u64,
}

#[derive(Debug, Clone)]
pub enum MediaSource {
    Device(String), // "Nokia Lumia 1020"
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Language {
    Rust,
    C,
    Python,
    JavaScript,
    OCaml,
    Mojo,
    Cpp,
    Csharp,
    Other(String),
}

impl Language {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rs" => Some(Self::Rust),
            "c" => Some(Self::C),
            "py" => Some(Self::Python),
            "js" => Some(Self::JavaScript),
            "ml" => Some(Self::OCaml),
            "mojo" => Some(Self::Mojo),
            "cpp" | "cc" | "cxx" => Some(Self::Cpp),
            "cs" => Some(Self::Csharp),
            // if necessary, put more languages
            // "" => Some(Self::),
            // "" => Some(Self::),
            // "" => Some(Self::),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::C => "C",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::OCaml => "OCaml",
            Language::Mojo => "Mojo",
            Language::Cpp => "C++",
            Language::Csharp => "C#",
            Language::Other(s) => s,
            // if necessary, put more languages
            // Language:: => "",
            // Language:: => "",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommonInfo {
    pub name: String,
    pub block_pointer: u64,

    pub extension: Option<String>,
    pub open_with: Option<String>,

    pub timestamps: Timestamps,
    pub permissions: Permissions,

    pub keywords: Vec<String>,

    pub duration_secs: Option<u64>,      // áudio/vídeo
    pub media_source: Option<MediaSource>, // imagem/vídeo
    pub language: Option<Language>,      // código
}

impl CommonInfo {
    pub fn detect_language(&mut self) {
        if let Some(ext) = &self.extension {
            self.language = Language::from_extension(ext);
        }
    }
}

let mut info = CommonInfo {
    name: "main.rs".into(),
    block_pointer: 42,

    extension: Some("rs".into()),
    open_with: None,

    timestamps: Timestamps { created: 0, modified: 0 },
    permissions: Permissions {
        owner: PermissionLevel::ReadWriteExecute,
        group: PermissionLevel::ReadExecute,
        others: PermissionLevel::None,
    },

    keywords: vec![],
    duration_secs: None,
    media_source: None,
    language: None,
};

info.detect_language();

println!("{:?}", info.language); // Some(Rust)

#[derive(Debug, Clone)]
pub enum InfoBlock {
    File(CommonInfo),
    Directory(CommonInfo),
    Binary(CommonInfo),
    Package(CommonInfo),
}