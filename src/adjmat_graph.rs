use std::fmt::{Display, Debug};
use std::fmt;
use std::hash::Hash;

use crate::error::EnceladusError;
use crate::map::Map;
use crate::hashmap::HashMap;
use crate::graph::{VertexNumber, EdgeNumber, Graph};

#[derive(Clone, Debug)]
pub struct AdjMatGraph<V, E> {
    num_vertices: usize,                        /* number of vertices */
    num_edges: usize,                           /* number of edges */
    adjacency_matrix: Vec<Vec<u64>>,            /* adjacency matrix */
    endpoints: HashMap<EdgeNumber, (VertexNumber, VertexNumber)>,
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
            endpoints: HashMap::new(),
            vertex_labels: HashMap::new(),
            edge_labels: HashMap::new()
        }
    }

    fn get_vertex(&self, vertex: VertexNumber) ->
    Result<Option<&V>, EnceladusError> {
        self.vertex_labels.get(vertex)
    }

    fn get_mut_vertex(&mut self, vertex: VertexNumber) ->
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

    fn get_mut_edge(&mut self, edge: EdgeNumber) ->
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
        /* add row */
        let mut new_row: Vec<u64> = Vec::new();
        
        for _i in 0..=self.num_vertices {
            new_row.push(0);
        }

        self.adjacency_matrix.push(new_row);

        /* add column */
        for i in 0..=self.num_vertices {
            self.adjacency_matrix[i].push(0);
        }

        /* increment number of vertices */
        self.num_vertices += 1;

        Ok(self.num_vertices - 1)
    }

    fn remove_vertex(&mut self, vertex: VertexNumber) ->
    Result<(), EnceladusError> {
        if !self.vertex_labels.contains_key(vertex)? {
            return Err(EnceladusError::VertexNotFound);
        }

        /* prune all edges attached to this vertex */
        let incident_edges: Vec<EdgeNumber> = self.incident_edges(vertex)?;

        for edge in incident_edges.iter() {
            self.remove_edge(*edge)?;
        }

        /* remove label */
        self.vertex_labels.remove(vertex);

        /* remove row */
        self.adjacency_matrix.remove(vertex);

        /* remove column */
        for i in 0..self.num_vertices-1 {
            self.adjacency_matrix[i].remove(vertex);
        }

        /* decrement number of vertices */
        self.num_vertices -= 1;

        Ok(())
    }

    fn insert_edge(&mut self, label: E, a: VertexNumber, b: VertexNumber) ->
    Result<EdgeNumber, EnceladusError> {
        if !(self.vertex_labels.contains_key(a)? &&
            self.vertex_labels.contains_key(b)?) {
            return Err(EnceladusError::VertexNotFound);
        }

        /* add edge label */
        self.edge_labels.insert(self.num_edges, label);

        /* add vertices to endpoint store */
        self.endpoints.insert(self.num_edges, (a, b));

        /* update the adjacency matrix accordingly (note that this also handles
         * the case of a == b */
        self.adjacency_matrix[a][b] += 1;
        self.adjacency_matrix[b][a] += 1;

        /* update number of edges */
        self.num_edges += 1;

        Ok(self.num_edges - 1)
    }

    fn order(&self) -> Result<usize, EnceladusError> {
       Ok(self.num_vertices)
    }

    fn size(&self) -> Result<usize, EnceladusError> {
        Ok(self.num_edges)
    }
}
