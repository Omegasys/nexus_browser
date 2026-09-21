use std::collections::HashMap;

/// Type of DOM node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomNodeType {
    Document,
    Element,
    Text,
    Comment,
    Fragment,
}

/// An attribute attached to a DOM element.
#[derive(Debug, Clone)]
pub struct DomAttribute {
    pub name: String,
    pub value: String,
}

/// A DOM node.
#[derive(Debug, Clone)]
pub struct DomNode {
    pub id: u64,
    pub node_type: DomNodeType,
    pub name: String,
    pub value: Option<String>,
    pub parent_id: Option<u64>,
    pub children: Vec<u64>,
}

/// An element-specific representation.
#[derive(Debug, Clone)]
pub struct DomElement {
    pub node: DomNode,
    pub attributes: Vec<DomAttribute>,
}

/// DOM inspection error.
#[derive(Debug, Clone)]
pub enum DomInspectorError {
    NodeNotFound(u64),
    ParentNotFound(u64),
    InvalidNode,
}

/// Developer DOM inspector.
///
/// The inspector stores a lightweight representation of the DOM tree.
/// A rendering engine can later populate this structure from its actual
/// document model.
#[derive(Debug, Clone)]
pub struct DomInspector {
    nodes: HashMap<u64, DomNode>,
    attributes: HashMap<u64, Vec<DomAttribute>>,
    root_id: Option<u64>,
    next_id: u64,
}

impl DomInspector {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            attributes: HashMap::new(),
            root_id: None,
            next_id: 1,
        }
    }

    pub fn create_node(
        &mut self,
        node_type: DomNodeType,
        name: impl Into<String>,
        value: Option<String>,
        parent_id: Option<u64>,
    ) -> Result<u64, DomInspectorError> {
        if let Some(parent) = parent_id {
            if !self.nodes.contains_key(&parent) {
                return Err(DomInspectorError::ParentNotFound(parent));
            }
        }

        let id = self.next_id;
        self.next_id += 1;

        let node = DomNode {
            id,
            node_type,
            name: name.into(),
            value,
            parent_id,
            children: Vec::new(),
        };

        self.nodes.insert(id, node);

        if let Some(parent) = parent_id {
            if let Some(parent_node) = self.nodes.get_mut(&parent) {
                parent_node.children.push(id);
            }
        } else if self.root_id.is_none() {
            self.root_id = Some(id);
        }

        Ok(id)
    }

    pub fn set_attribute(
        &mut self,
        node_id: u64,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), DomInspectorError> {
        let node = self
            .nodes
            .get(&node_id)
            .ok_or(DomInspectorError::NodeNotFound(node_id))?;

        if node.node_type != DomNodeType::Element {
            return Err(DomInspectorError::InvalidNode);
        }

        let attributes = self.attributes.entry(node_id).or_default();

        let name = name.into();
        let value = value.into();

        if let Some(existing) =
            attributes.iter_mut().find(|attribute| attribute.name == name)
        {
            existing.value = value;
        } else {
            attributes.push(DomAttribute { name, value });
        }

        Ok(())
    }

    pub fn get_node(&self, node_id: u64) -> Option<&DomNode> {
        self.nodes.get(&node_id)
    }

    pub fn get_attributes(&self, node_id: u64) -> &[DomAttribute] {
        self.attributes
            .get(&node_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn children(&self, node_id: u64) -> Result<Vec<&DomNode>, DomInspectorError> {
        let node = self
            .nodes
            .get(&node_id)
            .ok_or(DomInspectorError::NodeNotFound(node_id))?;

        Ok(node
            .children
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .collect())
    }

    pub fn root(&self) -> Option<&DomNode> {
        self.root_id.and_then(|id| self.nodes.get(&id))
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.attributes.clear();
        self.root_id = None;
        self.next_id = 1;
    }
}

impl Default for DomInspector {
    fn default() -> Self {
        Self::new()
    }
}
