pub trait Visitor<T> {
    fn visit(&mut self, v: &TreeNode<T>);
}

pub struct TreeNode<T> {
    value: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    // Define a new function to create a new node
    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            children: vec![],
        }
    }

    // Define a function to add a child to a node
    pub fn add_child(&mut self, node: TreeNode<T>) {
        self.children.push(node);
    }

    // Define a function to accept a visitor
    pub fn accept(&self, visitor: &mut dyn Visitor<T>) {
        visitor.visit(self);
        for child in &self.children {
            child.accept(visitor);
        }
    }
}

pub struct PrintVisitor;

impl Visitor<i32> for PrintVisitor {
    fn visit(&mut self, node: &TreeNode<i32>) {
        println!("{}", node.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_creation() {
        let node = TreeNode::new(5);
        assert_eq!(node.value, 5);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_add_child() {
        let mut node = TreeNode::new(5);
        let child = TreeNode::new(10);
        node.add_child(child);
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].value, 10);
    }

    #[test]
    fn test_print_visitor() {
        let mut node = TreeNode::new(5);
        let child = TreeNode::new(10);
        node.add_child(child);

        let mut visitor = PrintVisitor;
        node.accept(&mut visitor);
    }
}
