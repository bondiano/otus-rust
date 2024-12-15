use std::{cell::RefCell, rc::Rc};

type FindResult = Option<Box<dyn Component>>;

pub trait Component {
    fn find(&self, key: &str) -> FindResult;
}

pub struct Folder {
    name: String,
    components: Rc<RefCell<Vec<Box<dyn Component>>>>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn add(&mut self, component: Box<dyn Component>) {
        self.components.borrow_mut().push(component);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Clone for Folder {
    fn clone(&self) -> Self {
        Folder {
            name: self.name.clone(),
            components: self.components.clone(),
        }
    }
}

impl Component for Folder {
    fn find(&self, key: &str) -> FindResult {
        println!("Searching for {} in folder {}", key, self.get_name());
        if self.name == key {
            return Some(Box::new(self.clone()));
        }

        for component in self.components.borrow().iter() {
            if let Some(component) = component.find(key) {
                return Some(component);
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct File {
    name: String,
}

impl File {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Component for File {
    fn find(&self, key: &str) -> FindResult {
        if self.name == key {
            return Some(Box::new(self.clone()));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_existing_component() {
        let mut root = Folder::new("root");
        let leaf = Box::new(File::new("leaf"));
        root.add(leaf);

        assert!(root.find("leaf").is_some());
    }

    #[test]
    fn test_find_non_existing_component() {
        let root = Folder::new("root");

        assert!(root.find("non_existing").is_none());
    }

    #[test]
    fn test_find_in_nested_composite() {
        let mut root = Folder::new("root");
        let mut child = Folder::new("child");
        let leaf = Box::new(Folder::new("leaf"));
        child.add(leaf);
        root.add(Box::new(child));

        assert!(root.find("leaf").is_some());
    }
}
