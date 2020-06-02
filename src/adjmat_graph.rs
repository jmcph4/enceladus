use std::fmt::{Display, Debug};
use std::fmt;
use std::hash::Hash;

use crate::error::EnceladusError;
use crate::map::Map;
use crate::hashmap::HashMap;
use crate::graph::{VertexNumber, EdgeNumber, Graph};

#[derive(Clone, Debug)]
struct AdjMatGraph<V, E> {
    num_vertices: usize,                        /* number of vertices */
    num_edges: usize,                           /* number of edges */
    adjacency_matrix: Vec<Vec<u64>>,            /* adjacency matrix */
    vertex_labels: HashMap<VertexNumber, V>,    /* vertex labels */
    edge_labels: HashMap<EdgeNumber, E>         /* edge labels */
}

impl<V, E> Display for AdjMatGraph<V, E> where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.num_vertices, self.num_edges)
    }
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

impl<V, E> Graph<V, E> for AdjMatGraph<V, E> where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash {
    fn new() -> Self {
        AdjMatGraph {
            num_vertices: 0,
            num_edges: 0,
            adjacency_matrix: Vec::new(),
            vertex_labels: HashMap::new(),
            edge_labels: HashMap::new()
        }
    }

    fn get_vertex(&self, vertex: VertexNumber) ->
    Result<Option<&V>, EnceladusError> {
        self.vertex_labels.get(vertex)
    }

    fn get_mut_vertex(&self, vertex: VertexNumber) ->
    Result<Option<&mut V>, EnceladusError> {
        self.vertex_labels.get_mut(vertex)
    }

    fn set_vertex(&mut self, vertex: VertexNumber, label: V) ->
    Result<(), EnceladusError> {
        self.vertex_labels.set(vertex, label)
    }

    fn get_edge(&self, edge: EdgeNumber) ->
    Result<Option<&E>, EnceladusError> {
        self.edge_labels.get(edge)
    }

    fn get_mut_edge(&self, edge: EdgeNumber) ->
    Result<Option<&mut E>, EnceladusError> {
        self.edge_labels.get_mut(edge)
    }

    fn set_edge(&mut self, edge: EdgeNumber, label: E) ->
    Result<(), EnceladusError> {
        self.edge_labels.set(edge, label)
    }

    fn insert_vertex(&mut self, label: V) ->
    Result<VertexNumber, EnceladusError> {
        /* add vertex label */
        self.vertex_labels.insert(self.num_vertices, label).unwrap();

        /* expand adjacency matrix */
        let mut new_column: Vec<u64> = Vec::new();
        
        for _i in 0..=self.num_vertices {
            new_column.push(0);
        }

        self.adjacency_matrix.push(new_column);

        /* increment number of vertices */
        self.num_vertices += 1;

        Ok(self.num_vertices - 1)
    }

    fn order(&self) -> Result<usize, EnceladusError> {
       Ok(self.num_vertices)
    }

    fn size(&self) -> Result<usize, EnceladusError> {
        Ok(self.num_edges)
    }
}

