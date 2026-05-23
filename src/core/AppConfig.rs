pub struct AppConfig {
    pub tabCols: i32,
    pub warnCol: i32,
    pub limitCol: i32,
    pub nearbyLineCount: i32,
    pub extensions: Vec<String>,
    pub foldersToScanForSources: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tabCols: 4,
            warnCol: 72,
            limitCol: 80,
            nearbyLineCount: 2,
            extensions: vec![
                ".hpp".to_string(), // bullshit syntax...
                ".h".to_string(),
                ".c".to_string(),
                ".cpp".to_string(),
            ],
            foldersToScanForSources: vec![
                "inc".to_string(),
                "src".to_string(),
            ],
        }
    }
}

