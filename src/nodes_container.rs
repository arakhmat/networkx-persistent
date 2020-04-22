#![allow(clippy::derive_hash_xor_eq)]

use std::hash::{Hash, Hasher};

use rpds::HashTrieSet as RpdsSet;

#[derive(PartialEq, Eq, Clone)]
pub struct NodesContainer<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    nodes: RpdsSet<NodeType>,
}

impl<NodeType> NodesContainer<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        NodesContainer {
            nodes: RpdsSet::new(),
        }
    }

    pub fn contains(&self, node: &NodeType) -> bool {
        self.nodes.contains(node)
    }

    pub fn size(&self) -> usize {
        self.nodes.size()
    }

    pub fn add_node(&self, node: NodeType) -> Self {
        let nodes = self.nodes.insert(node);
        NodesContainer { nodes }
    }

    pub fn remove_node(&self, node: &NodeType) -> Self {
        let nodes = self.nodes.remove(node);
        NodesContainer { nodes }
    }

    pub fn iter(&self) -> rpds::set::hash_trie_set::Iter<'_, NodeType, archery::RcK> {
        self.nodes.iter()
    }
}

impl<NodeType> Hash for NodesContainer<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Add the hash of length so that if two collections are added one after the other it doesn't
        // hash to the same thing as a single collection with the same elements in the same order.
        self.nodes.size().hash(state);
        for element in self.nodes.iter() {
            element.hash(state);
        }
    }
}
