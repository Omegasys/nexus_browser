use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

pub struct ExtensionManager {
    extensions: HashMap<String, Extension>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn install(
        &mut self,
        extension: Extension,
    ) {
        self.extensions.insert(
            extension.id.clone(),
            extension
        );
    }

    pub fn uninstall(
        &mut self,
        id: &str,
    ) {
        self.extensions.remove(id);
    }

    pub fn enable(
        &mut self,
        id: &str,
    ) {
        if let Some(ext) = self.extensions.get_mut(id) {
            ext.enabled = true;
        }
    }

    pub fn disable(
        &mut self,
        id: &str,
    ) {
        if let Some(ext) = self.extensions.get_mut(id) {
            ext.enabled = false;
        }
    }

    pub fn get(
        &self,
        id: &str,
    ) -> Option<&Extension> {
        self.extensions.get(id)
    }

    pub fn installed_count(&self) -> usize {
        self.extensions.len()
    }
}
