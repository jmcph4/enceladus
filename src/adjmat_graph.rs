use crate::graph::{VertexNumber, EdgeNumber};
use crate::hashmap::HashMap;

#[derive(Clone, Debug)]
struct AdjMatGraph<V, E> {
    adjacency_matrix: Vec<Vec<u64>>,            /* adjacency matrix */
    vertex_labels: HashMap<VertexNumber, V>,    /* vertex labels */
    edge_labels: HashMap<EdgeNumber, E>         /* edge labels */
}

