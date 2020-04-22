#![allow(clippy::derive_hash_xor_eq)]

use std::hash::{Hash, Hasher};

use crate::nodes_container::NodesContainer;
use rpds::HashTrieMap as RpdsMap;
use rpds::HashTrieSet as RpdsSet;

#[derive(PartialEq, Eq, Clone)]
pub struct DiGraph<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    nodes_container: NodesContainer<NodeType>,
    successors: RpdsMap<NodeType, RpdsSet<NodeType>>,
    predecessors: RpdsMap<NodeType, RpdsSet<NodeType>>,
}

impl<NodeType> DiGraph<NodeType>
where
    NodeType: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        DiGraph {
            nodes_container: NodesContainer::new(),
            successors: RpdsMap::new(),
            predecessors: RpdsMap::new(),
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
        let successors = self.successors.insert(node.clone(), RpdsSet::new());
        let predecessors = self.predecessors.insert(node, RpdsSet::new());
        DiGraph {
            nodes_container,
            predecessors,
            successors,
        }
    }

    pub fn remove_node(&self, node: &NodeType) -> Self {
        let nodes_container = self.nodes_container.remove_node(node);

        let successors = self.successors.clone();
        let predecessors = self.predecessors.clone();

        let remove_edge =
            |direction_to_remove_from: RpdsMap<NodeType, RpdsSet<NodeType>>,
             other_direction: RpdsMap<NodeType, RpdsSet<NodeType>>| {
                if direction_to_remove_from.contains_key(node) {
                    return (direction_to_remove_from, other_direction);
                }
                let mut other_direction = other_direction;
                for neighbor in direction_to_remove_from.get(node).unwrap().iter() {
                    let other_direction_edge_set = other_direction[node].remove(node);
                    other_direction =
                        other_direction.insert(neighbor.clone(), other_direction_edge_set);
                }

                let direction_to_remove_from = direction_to_remove_from.remove(node);
                (direction_to_remove_from, other_direction)
            };

        let (successors, predecessors) = remove_edge(successors, predecessors);
        let (predecessors, successors) = remove_edge(predecessors, successors);

        DiGraph {
            nodes_container,
            predecessors,
            successors,
        }
    }

    pub fn add_edge(&self, u: &NodeType, v: &NodeType) -> Self {
        let successors_set = self.successors[u].clone();
        let successors_set = successors_set.insert(v.clone());
        let successors = self.successors.insert(u.clone(), successors_set);

        let predecessors_set = self.predecessors[v].clone();
        let predecessors_set = predecessors_set.insert(u.clone());
        let predecessors = self.predecessors.insert(v.clone(), predecessors_set);

        DiGraph {
            nodes_container: self.nodes_container.clone(),
            predecessors,
            successors,
        }
    }

    pub fn remove_edge(&self, u: &NodeType, v: &NodeType) -> Self {
        let successors = self.successors.clone();
        let predecessors = self.predecessors.clone();

        let remove_edge =
            |node, other_node, direction_to_remove_from: RpdsMap<NodeType, RpdsSet<NodeType>>| {
                if direction_to_remove_from.contains_key(node) {
                    return direction_to_remove_from;
                }
                let edge_set = direction_to_remove_from[node].remove(other_node);
                direction_to_remove_from.insert(node.clone(), edge_set)
            };

        let successors = remove_edge(u, v, successors);
        let predecessors = remove_edge(v, u, predecessors);

        DiGraph {
            nodes_container: self.nodes_container.clone(),
            predecessors,
            successors,
        }
    }

    pub fn edges(&self) -> RpdsMap<NodeType, RpdsSet<NodeType>> {
        self.successors.clone()
    }

    pub fn iter(&self) -> rpds::set::hash_trie_set::Iter<'_, NodeType, archery::RcK> {
        self.nodes_container.iter()
    }
}

impl<NodeType> Hash for DiGraph<NodeType>
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
