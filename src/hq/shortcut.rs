use common::event;
use std::ops::{Deref, DerefMut};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum Node {
    Internal { children: BTreeMap<event::Key, Box<Node>>, },
    Leaf(String),
}

impl Node {
    /// Construct a new internal node.
    fn new_inner() -> Node {
        Node::Internal { children: BTreeMap::new() }
    }

    /// Construct a new leaf node.
    fn new_leaf(value: &str) -> Node {
        Node::Leaf(String::from(value))
    }

    /// Insert a new node.
    fn insert(&mut self, value: &str, keys: Vec<event::Key>, idx: usize) -> bool {
        match self {
            &mut Node::Internal { ref mut children } => {
                if idx == keys.len() - 1 {
                    // Leaf node
                    children.insert(keys[idx], Box::new(Node::new_leaf(value))).is_none()
                } else {
                    // Inner node
                    children.contains_key(&keys[idx]) ||
                    children.insert(keys[idx], Box::new(Node::new_inner())).is_some();
                    children.get_mut(&keys[idx])
                        .map(|n| n.deref_mut().insert(value, keys, idx + 1))
                        .unwrap_or(false)
                }
            }
            _ => false,
        }
    }

    /// Find the next node.
    fn get(&self, key: event::Key) -> Option<&Node> {
        match self {
            &Node::Internal { ref children } => children.get(&key).map(|n| n.deref()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Shortcut {
    head: Node,
    current: Vec<event::Key>,
}

impl Shortcut {
    pub fn new() -> Shortcut {
        Shortcut {
            head: Node::Internal { children: BTreeMap::new() },
            current: Vec::with_capacity(16),
        }
    }

    pub fn key(&mut self, key: event::Key) -> Option<&Node> {
        self.current.push(key);
        self.current.iter().fold(Some(&self.head),
                                 |acc, key| acc.and_then(|n| n.get(key.clone())))
    }

    pub fn add(&mut self, value: &str, keys: Vec<event::Key>) {
        assert!(self.head.insert(value, keys, 0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_shortcut() {
        let mut sc = Shortcut::new();
        sc.add("exit", vec![event::Key::Ctrl('x'), event::Key::Ctrl('c')]);
        sc.key(event::Key::Ctrl('x'));
        sc.key(event::Key::Ctrl('c'));
    }
}
