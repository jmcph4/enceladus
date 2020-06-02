use std::fmt::{Display, Debug};
use std::hash::Hash;

use crate::hashmap::HashMap;

use crate::graph::{VertexNumber, EdgeNumber};

#[derive(Clone, Debug)]
struct AdjMatGraph<V, E> {
    num_vertices: usize,                        /* number of vertices */
    num_edges: usize,                           /* number of edges */
    adjacency_matrix: Vec<Vec<u64>>,            /* adjacency matrix */
    vertex_labels: HashMap<VertexNumber, V>,    /* vertex labels */
    edge_labels: HashMap<EdgeNumber, E>         /* edge labels */
}

impl<V, E> PartialEq for AdjMatGraph<V, E> where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash {
    fn eq(&self, other: &Self) -> bool {
        if self.num_vertices == other.num_vertices &&
        self.num_edges == other.num_edges && 
        self.vertex_labels == other.vertex_labels &&
        self.edge_labels == other.edge_labels {
            return true;
        }

        if self.num_vertices != other.num_vertices ||
        self.num_edges != other.num_edges {
            return false;
        }

        unimplemented!()
    }
}

impl<V, E> Eq for AdjMatGraph<V, E> where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash {}

