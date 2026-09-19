use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeAppearance {
    Dark,
    Light,
    HighContrast,
    Custom(String),
}

impl ThemeAppearance {
    fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "dark" => Self::Dark,
            "light" => Self::Light,
            "high-contrast" => Self::HighContrast,
            other => Self::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: String,
    pub surface: String,
    pub surface_alt: String,
    pub foreground: String,
    pub foreground_muted: String,
    pub accent: String,
    pub accent_hover: String,
    pub accent_active: String,
    pub border: String,
    pub divider: String,
    pub selection: String,
    pub focus: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

#[derive(Debug, Clone)]
pub struct BrowserTheme {
    pub toolbar_background: String,
    pub toolbar_foreground: String,
    pub tab_background: String,
    pub tab_active_background: String,
    pub tab_active_foreground: String,
    pub tab_inactive_foreground: String,
    pub address_bar_background: String,
    pub address_bar_foreground: String,
    pub bookmark_background: String,
}

#[derive(Debug, Clone)]
pub struct WorkspaceTheme {
    pub background: String,
    pub active_indicator: String,
    pub inactive_indicator: String,
    pub group_background: String,
    pub group_active_background: String,
}

#[derive(Debug, Clone)]
pub struct PrivacyTheme {
    pub protected: String,
    pub warning: String,
    pub blocked: String,
    pub isolated: String,
}

#[derive(Debug, Clone)]
pub struct TerminalTheme {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub version: String,
    pub appearance: ThemeAppearance,
    pub colors: ThemeColors,
    pub browser: BrowserTheme,
    pub workspace: WorkspaceTheme,
    pub privacy: PrivacyTheme,
    pub terminal: TerminalTheme,
}

#[derive(Debug, Clone)]
pub struct WorkspaceThemeAssignment {
    pub workspace_id: u64,
    pub theme_id: String,
}

#[derive(Debug)]
pub enum ThemeError {
    Io(std::io::Error),
    InvalidJson(String),
    ThemeNotFound(String),
}

impl From<std::io::Error> for ThemeError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl Theme {
    pub fn from_json(data: &str) -> Result<Self, ThemeError> {
        let value: SimpleJson =
            SimpleJson::parse(data)
                .map_err(ThemeError::InvalidJson)?;

        Self::from_value(value)
    }

    fn from_value(value: SimpleJson) -> Result<Self, ThemeError> {
        let object = value
            .as_object()
            .ok_or_else(|| {
                ThemeError::InvalidJson(
                    "Theme root must be an object".to_string()
                )
            })?;

        let id = required_string(object, "id")?;
        let name = required_string(object, "name")?;
        let version = required_string(object, "version")?;
        let appearance = ThemeAppearance::from_str(
            &required_string(object, "appearance")?,
        );

        let colors = ThemeColors::from_object(
            required_object(object, "colors")?,
        )?;

        let browser = BrowserTheme::from_object(
            required_object(object, "browser")?,
        )?;

        let workspace = WorkspaceTheme::from_object(
            required_object(object, "workspace")?,
        )?;

        let privacy = PrivacyTheme::from_object(
            required_object(object, "privacy")?,
        )?;

        let terminal = TerminalTheme::from_object(
            required_object(object, "terminal")?,
        )?;

        Ok(Self {
            id,
            name,
            version,
            appearance,
            colors,
            browser,
            workspace,
            privacy,
            terminal,
        })
    }
}

impl ThemeColors {
    fn from_object(
        object: &HashMap<String, SimpleJson>,
    ) -> Result<Self, ThemeError> {
        Ok(Self {
            background: required_string(object, "background")?,
            surface: required_string(object, "surface")?,
            surface_alt: required_string(object, "surface_alt")?,
            foreground: required_string(object, "foreground")?,
            foreground_muted: required_string(
                object,
                "foreground_muted",
            )?,
            accent: required_string(object, "accent")?,
            accent_hover: required_string(
                object,
                "accent_hover",
            )?,
            accent_active: required_string(
                object,
                "accent_active",
            )?,
            border: required_string(object, "border")?,
            divider: required_string(object, "divider")?,
            selection: required_string(object, "selection")?,
            focus: required_string(object, "focus")?,
            success: required_string(object, "success")?,
            warning: required_string(object, "warning")?,
            error: required_string(object, "error")?,
            info: required_string(object, "info")?,
        })
    }
}

impl BrowserTheme {
    fn from_object(
        object: &HashMap<String, SimpleJson>,
    ) -> Result<Self, ThemeError> {
        Ok(Self {
            toolbar_background: required_string(
                object,
                "toolbar_background",
            )?,
            toolbar_foreground: required_string(
                object,
                "toolbar_foreground",
            )?,
            tab_background: required_string(
                object,
                "tab_background",
            )?,
            tab_active_background: required_string(
                object,
                "tab_active_background",
            )?,
            tab_active_foreground: required_string(
                object,
                "tab_active_foreground",
            )?,
            tab_inactive_foreground: required_string(
                object,
                "tab_inactive_foreground",
            )?,
            address_bar_background: required_string(
                object,
                "address_bar_background",
            )?,
            address_bar_foreground: required_string(
                object,
                "address_bar_foreground",
            )?,
            bookmark_background: required_string(
                object,
                "bookmark_background",
            )?,
        })
    }
}

impl WorkspaceTheme {
    fn from_object(
        object: &HashMap<String, SimpleJson>,
    ) -> Result<Self, ThemeError> {
        Ok(Self {
            background: required_string(object, "background")?,
            active_indicator: required_string(
                object,
                "active_indicator",
            )?,
            inactive_indicator: required_string(
                object,
                "inactive_indicator",
            )?,
            group_background: required_string(
                object,
                "group_background",
            )?,
            group_active_background: required_string(
                object,
                "group_active_background",
            )?,
        })
    }
}

impl PrivacyTheme {
    fn from_object(
        object: &HashMap<String, SimpleJson>,
    ) -> Result<Self, ThemeError> {
        Ok(Self {
            protected: required_string(object, "protected")?,
            warning: required_string(object, "warning")?,
            blocked: required_string(object, "blocked")?,
            isolated: required_string(object, "isolated")?,
        })
    }
}

impl TerminalTheme {
    fn from_object(
        object: &HashMap<String, SimpleJson>,
    ) -> Result<Self, ThemeError> {
        Ok(Self {
            background: required_string(object, "background")?,
            foreground: required_string(object, "foreground")?,
            cursor: required_string(object, "cursor")?,
            selection: required_string(object, "selection")?,
        })
    }
}

#[derive(Debug, Default)]
pub struct WorkspaceThemeManager {
    themes: HashMap<String, Theme>,
    workspace_themes: HashMap<u64, String>,
    theme_directory: Option<PathBuf>,
}

impl WorkspaceThemeManager {
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            workspace_themes: HashMap::new(),
            theme_directory: None,
        }
    }

    pub fn with_directory(
        directory: impl Into<PathBuf>,
    ) -> Self {
        Self {
            themes: HashMap::new(),
            workspace_themes: HashMap::new(),
            theme_directory: Some(directory.into()),
        }
    }

    pub fn register(&mut self, theme: Theme) {
        self.themes.insert(theme.id.clone(), theme);
    }

    pub fn load_file(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<(), ThemeError> {
        let data = fs::read_to_string(path)?;
        let theme = Theme::from_json(&data)?;

        self.register(theme);

        Ok(())
    }

    pub fn load_directory(
        &mut self,
        directory: impl AsRef<Path>,
    ) -> Result<usize, ThemeError> {
        let mut loaded = 0;

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|value| value.to_str())
                != Some("json")
            {
                continue;
            }

            self.load_file(path)?;
            loaded += 1;
        }

        Ok(loaded)
    }

    pub fn load_configured_directory(
        &mut self,
    ) -> Result<usize, ThemeError> {
        let directory = self
            .theme_directory
            .clone()
            .ok_or_else(|| {
                ThemeError::ThemeNotFound(
                    "No theme directory configured".to_string(),
                )
            })?;

        self.load_directory(directory)
    }

    pub fn get(&self, theme_id: &str) -> Option<&Theme> {
        self.themes.get(theme_id)
    }

    pub fn remove(
        &mut self,
        theme_id: &str,
    ) -> Option<Theme> {
        self.themes.remove(theme_id)
    }

    pub fn assign_to_workspace(
        &mut self,
        workspace_id: u64,
        theme_id: &str,
    ) -> Result<(), ThemeError> {
        if !self.themes.contains_key(theme_id) {
            return Err(
                ThemeError::ThemeNotFound(
                    theme_id.to_string(),
                )
            );
        }

        self.workspace_themes.insert(
            workspace_id,
            theme_id.to_string(),
        );

        Ok(())
    }

    pub fn theme_for_workspace(
        &self,
        workspace_id: u64,
    ) -> Option<&Theme> {
        let theme_id =
            self.workspace_themes.get(&workspace_id)?;

        self.themes.get(theme_id)
    }

    pub fn theme_id_for_workspace(
        &self,
        workspace_id: u64,
    ) -> Option<&str> {
        self.workspace_themes
            .get(&workspace_id)
            .map(String::as_str)
    }

    pub fn clear_workspace(
        &mut self,
        workspace_id: u64,
    ) {
        self.workspace_themes.remove(&workspace_id);
    }

    pub fn clear_all_assignments(&mut self) {
        self.workspace_themes.clear();
    }

    pub fn theme_names(&self) -> Vec<&str> {
        self.themes
            .values()
            .map(|theme| theme.name.as_str())
            .collect()
    }

    pub fn theme_ids(&self) -> Vec<&str> {
        self.themes
            .keys()
            .map(String::as_str)
            .collect()
    }

    pub fn theme_count(&self) -> usize {
        self.themes.len()
    }
}

/*
 * Minimal JSON representation used by the theme loader.
 *
 * The browser can replace this parser with serde_json when the
 * dependency stack is established. Keeping the theme manager
 * independent of serde makes the architecture easier to integrate
 * into the browser kernel later.
 */

#[derive(Debug, Clone)]
enum SimpleJson {
    Object(HashMap<String, SimpleJson>),
    String(String),
}

impl SimpleJson {
    fn as_object(&self) -> Option<&HashMap<String, SimpleJson>> {
        match self {
            Self::Object(value) => Some(value),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    fn parse(input: &str) -> Result<Self, String> {
        let mut parser = JsonParser::new(input);
        let value = parser.parse_value()?;

        parser.skip_whitespace();

        if !parser.is_end() {
            return Err(
                "Unexpected data after JSON value".to_string()
            );
        }

        Ok(value)
    }
}

fn required_string(
    object: &HashMap<String, SimpleJson>,
    key: &str,
) -> Result<String, ThemeError> {
    object
        .get(key)
        .and_then(SimpleJson::as_string)
        .map(str::to_string)
        .ok_or_else(|| {
            ThemeError::InvalidJson(format!(
                "Missing or invalid string field: {}",
                key
            ))
        })
}

fn required_object<'a>(
    object: &'a HashMap<String, SimpleJson>,
    key: &str,
) -> Result<&'a HashMap<String, SimpleJson>, ThemeError> {
    object
        .get(key)
        .and_then(SimpleJson::as_object)
        .ok_or_else(|| {
            ThemeError::InvalidJson(format!(
                "Missing or invalid object field: {}",
                key
            ))
        })
}

struct JsonParser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
        }
    }

    fn is_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_end()
            && self.input[self.position].is_ascii_whitespace()
        {
            self.position += 1;
        }
    }

    fn parse_value(&mut self) -> Result<SimpleJson, String> {
        self.skip_whitespace();

        match self.current() {
            Some(b'{') => self.parse_object(),
            Some(b'"') => {
                Ok(SimpleJson::String(
                    self.parse_string()?
                ))
            }
            _ => Err(
                "Unsupported JSON value".to_string()
            ),
        }
    }

    fn parse_object(&mut self) -> Result<SimpleJson, String> {
        self.expect(b'{')?;

        let mut object = HashMap::new();

        loop {
            self.skip_whitespace();

            if self.consume(b'}') {
                break;
            }

            let key = self.parse_string()?;

            self.skip_whitespace();
            self.expect(b':')?;

            let value = self.parse_value()?;

            object.insert(key, value);

            self.skip_whitespace();

            if self.consume(b'}') {
                break;
            }

            self.expect(b',')?;
        }

        Ok(SimpleJson::Object(object))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;

        let mut result = String::new();

        while !self.is_end() {
            let byte = self.input[self.position];
            self.position += 1;

            match byte {
                b'"' => return Ok(result),

                b'\\' => {
                    if self.is_end() {
                        return Err(
                            "Invalid string escape".to_string()
                        );
                    }

                    let escaped = self.input[self.position];
                    self.position += 1;

                    match escaped {
                        b'"' => result.push('"'),
                        b'\\' => result.push('\\'),
                        b'/' => result.push('/'),
                        b'n' => result.push('\n'),
                        b'r' => result.push('\r'),
                        b't' => result.push('\t'),
                        _ => {
                            return Err(
                                "Unsupported escape sequence"
                                    .to_string()
                            )
                        }
                    }
                }

                byte if byte.is_ascii() => {
                    result.push(byte as char);
                }

                _ => {
                    return Err(
                        "Unsupported UTF-8 sequence".to_string()
                    )
                }
            }
        }

        Err("Unterminated JSON string".to_string())
    }

    fn current(&self) -> Option<u8> {
        self.input.get(self.position).copied()
    }

    fn consume(&mut self, expected: u8) -> bool {
        if self.current() == Some(expected) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), String> {
        if self.consume(expected) {
            Ok(())
        } else {
            Err(format!(
                "Expected '{}'",
                expected as char
            ))
        }
    }
}
