#![allow(clippy::derive_hash_xor_eq)]

use std::hash::{Hash, Hasher};

use crate::nodes_container::NodesContainer;
use rpds::HashTrieMap as RpdsMap;
use rpds::HashTrieSet as RpdsSet;

#[derive(PartialEq, Eq, Clone)]
pub struct Graph<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    nodes_container: NodesContainer<NodeType>,
    neighbors_map: RpdsMap<NodeType, RpdsSet<NodeType>>,
}

impl<NodeType> Graph<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Graph {
            nodes_container: NodesContainer::new(),
            neighbors_map: RpdsMap::new(),
        }
    }

    pub fn contains(&self, node: &NodeType) -> bool {
        self.nodes_container.contains(node)
    }

    pub fn size(&self) -> usize {
        self.nodes_container.size()
    }

    pub fn add_node(&self, node: NodeType) -> Self {
        let nodes_container = self.nodes_container.add_node(node.clone());
        let neighbors_map = self.neighbors_map.insert(node, RpdsSet::new());
        Graph {
            nodes_container,
            neighbors_map,
        }
    }

    pub fn remove_node(&self, node: &NodeType) -> Self {
        let nodes_container = self.nodes_container.remove_node(node);

        let mut neighbors_map = self.neighbors_map.clone();

        for neighbor in self.neighbors_map[node].iter() {
            let neighbor_neighbors = neighbors_map[neighbor].remove(node);
            neighbors_map = neighbors_map.insert(neighbor.clone(), neighbor_neighbors)
        }

        let neighbors_map = neighbors_map.remove(node);

        Graph {
            nodes_container,
            neighbors_map,
        }
    }

    pub fn add_edge(&self, u: &NodeType, v: &NodeType) -> Self {
        let neighbors_map = self.neighbors_map.clone();

        let neighbors = neighbors_map[u].clone();
        let neighbors = neighbors.insert(v.clone());
        let neighbors_map = self.neighbors_map.insert(u.clone(), neighbors);

        let neighbors = neighbors_map[v].clone();
        let neighbors = neighbors.insert(u.clone());
        let neighbors_map = self.neighbors_map.insert(v.clone(), neighbors);

        Graph {
            nodes_container: self.nodes_container.clone(),
            neighbors_map,
        }
    }

    pub fn remove_edge(&self, u: &NodeType, v: &NodeType) -> Self {
        let neighbors_map = self.neighbors_map.clone();

        let neighbors = neighbors_map[u].clone();
        let neighbors = neighbors.remove(v);
        let neighbors_map = neighbors_map.insert(u.clone(), neighbors);

        let neighbors = neighbors_map[v].clone();
        let neighbors = neighbors.remove(u);
        let neighbors_map = neighbors_map.insert(v.clone(), neighbors);

        Graph {
            nodes_container: self.nodes_container.clone(),
            neighbors_map,
        }
    }

    pub fn edges(&self) -> RpdsMap<NodeType, RpdsSet<NodeType>> {
        self.neighbors_map.clone()
    }

    pub fn iter(&self) -> rpds::set::hash_trie_set::Iter<'_, NodeType, archery::RcK> {
        self.nodes_container.iter()
    }
}

impl<NodeType> Hash for Graph<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Add the hash of length so that if two collections are added one after the other it doesn't
        // hash to the same thing as a single collection with the same elements in the same order.
        self.nodes_container.size().hash(state);
        for element in self.nodes_container.iter() {
            element.hash(state);
        }
    }
}
