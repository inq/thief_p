use common::Key;
use std::ops::DerefMut;
use std::collections::BTreeMap;

#[derive(Clone)]
pub enum Node {
    Internal { children: BTreeMap<Key, Box<Node>> },
    Leaf(String)
}

impl Node {
    /// Insert a new node.
    fn insert(&mut self, keys: Vec<Key>, idx: usize, value: String) {
        if let &mut Node::Internal { ref mut children } = self {
            if idx == keys.len() - 1 {
                // Leaf node
                let leaf = Node::Leaf(value.clone());
                children.insert(keys[idx], Box::new(leaf));
            } else {
                // Inner node
                if !children.contains_key(&keys[idx]) {
                    let inner = Node::Internal { children: BTreeMap::new() };
                    children.insert(keys[idx], Box::new(inner));
                }
                if let Some(inner) = children.get_mut(&keys[idx]) {
                    inner.deref_mut().insert(keys, idx + 1, value);
                } else {
                    panic!("Internal Error.");
                }
            }
        } else {
            panic!("Internal error.");
        }
    }
}

pub struct Shortcut {
    head: Node,
}

impl Shortcut {
    pub fn new() -> Shortcut {
        Shortcut {
            head: Node::Internal { children: BTreeMap::new() }
        }
    }

    pub fn add(&mut self, keys: Vec<Key>, value: String) {
        self.head.insert(keys, 0, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_shortcut() {
        let mut sc = Shortcut::new();
        sc.add(vec![Key::Ctrl('x'), Key::Ctrl('c')], String::from("exit"));
    }
}
