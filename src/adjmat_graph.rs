use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::error::EnceladusError;
use crate::graph::{EdgeNumber, Graph, VertexNumber};
use crate::hashmap::HashMap;
use crate::map::Map;

#[derive(Clone, Debug)]
pub struct AdjMatGraph<V, E> {
    num_vertices: usize,             /* number of vertices */
    num_edges: usize,                /* number of edges */
    adjacency_matrix: Vec<Vec<u64>>, /* adjacency matrix */
    endpoints: HashMap<EdgeNumber, (VertexNumber, VertexNumber)>,
    vertex_labels: HashMap<VertexNumber, V>, /* vertex labels */
    edge_labels: HashMap<EdgeNumber, E>,     /* edge labels */
}

impl<V, E> Display for AdjMatGraph<V, E>
where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.num_vertices, self.num_edges)
    }
}

impl<V, E> PartialEq for AdjMatGraph<V, E>
where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        /* fieldwise equality */
        self.num_vertices == other.num_vertices
            && self.num_edges == other.num_edges
            && self.adjacency_matrix == other.adjacency_matrix
            && self.endpoints == other.endpoints
            && self.vertex_labels == other.vertex_labels
            && self.edge_labels == other.edge_labels
    }
}

impl<V, E> Eq for AdjMatGraph<V, E>
where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash,
{
}

impl<V, E> Graph<V, E> for AdjMatGraph<V, E>
where
    V: Sized + Clone + Eq + Display + Debug + Hash,
    E: Sized + Clone + Eq + Display + Debug + Hash,
{
    fn new() -> Self {
        AdjMatGraph {
            num_vertices: 0,
            num_edges: 0,
            adjacency_matrix: Vec::new(),
            endpoints: HashMap::new(),
            vertex_labels: HashMap::new(),
            edge_labels: HashMap::new(),
        }
    }

    fn get_vertex(
        &self,
        vertex: VertexNumber,
    ) -> Result<Option<&V>, EnceladusError> {
        self.vertex_labels.get(vertex)
    }

    fn get_mut_vertex(
        &mut self,
        vertex: VertexNumber,
    ) -> Result<Option<&mut V>, EnceladusError> {
        self.vertex_labels.get_mut(vertex)
    }

    fn set_vertex(
        &mut self,
        vertex: VertexNumber,
        label: V,
    ) -> Result<(), EnceladusError> {
        self.vertex_labels.set(vertex, label)
    }

    fn get_edge(&self, edge: EdgeNumber) -> Result<Option<&E>, EnceladusError> {
        self.edge_labels.get(edge)
    }

    fn get_mut_edge(
        &mut self,
        edge: EdgeNumber,
    ) -> Result<Option<&mut E>, EnceladusError> {
        self.edge_labels.get_mut(edge)
    }

    fn set_edge(
        &mut self,
        edge: EdgeNumber,
        label: E,
    ) -> Result<(), EnceladusError> {
        self.edge_labels.set(edge, label)
    }

    fn insert_vertex(
        &mut self,
        label: V,
    ) -> Result<VertexNumber, EnceladusError> {
        /* add vertex label */
        self.vertex_labels.insert(self.num_vertices, label).unwrap();

        /* base case */
        if self.num_vertices == 0 {
            self.adjacency_matrix.push(vec![0]);
            self.num_vertices += 1;
            return Ok(self.num_vertices - 1);
        }

        /* expand adjacency matrix */

        /* note that adding the new column BEFORE the new row avoids the issue
        of a staggered adjacency matrix (additionally observe the invariant
        that the adjacency matrix MUST ALWAYS BE SQUARE */

        /* add column */
        for i in 0..self.num_vertices {
            self.adjacency_matrix[i].push(0);
        }

        /* add row */
        let mut new_row: Vec<u64> = Vec::new();

        for _i in 0..=self.num_vertices {
            new_row.push(0);
        }

        self.adjacency_matrix.push(new_row);

        /* increment number of vertices */
        self.num_vertices += 1;

        Ok(self.num_vertices - 1)
    }

    fn remove_vertex(
        &mut self,
        vertex: VertexNumber,
    ) -> Result<(), EnceladusError> {
        if !self.vertex_labels.contains_key(vertex)? {
            return Err(EnceladusError::VertexNotFound);
        }

        /* base case */
        if self.num_vertices == 1 {
            self.adjacency_matrix = vec![];
            self.vertex_labels = HashMap::new();
            self.num_vertices = 0;
            return Ok(());
        }

        /* prune all edges attached to this vertex */
        let incident_edges: Vec<EdgeNumber> = self.incident_edges(vertex)?;

        for edge in incident_edges.iter() {
            self.remove_edge(*edge)?;
        }

        /* remove label */
        self.vertex_labels.remove(vertex)?;

        /* remove row */
        self.adjacency_matrix.remove(vertex);

        /* remove column */
        for i in 0..self.num_vertices - 1 {
            self.adjacency_matrix[i].remove(vertex);
        }

        /* decrement number of vertices */
        self.num_vertices -= 1;

        Ok(())
    }

    fn insert_edge(
        &mut self,
        label: E,
        a: VertexNumber,
        b: VertexNumber,
    ) -> Result<EdgeNumber, EnceladusError> {
        if !(self.vertex_labels.contains_key(a)?
            && self.vertex_labels.contains_key(b)?)
        {
            return Err(EnceladusError::VertexNotFound);
        }

        /* add edge label */
        self.edge_labels.insert(self.num_edges, label)?;

        /* add vertices to endpoint store */
        self.endpoints.insert(self.num_edges, (a, b))?;

        /* update the adjacency matrix accordingly (note that this also handles
         * the case of a == b */
        self.adjacency_matrix[a][b] += 1;
        self.adjacency_matrix[b][a] += 1;

        /* update number of edges */
        self.num_edges += 1;

        Ok(self.num_edges - 1)
    }

    fn remove_edge(&mut self, edge: EdgeNumber) -> Result<(), EnceladusError> {
        if !self.edge_labels.contains_key(edge)? {
            return Err(EnceladusError::EdgeNotFound);
        }

        /* remove label */
        self.edge_labels.remove(edge)?;

        /* store endpoints for adjacency matrix update */
        let (a, b): (VertexNumber, VertexNumber) =
            *self.endpoints.get(edge)?.unwrap();

        /* remove entry in endpoints table */
        self.endpoints.remove(edge)?;

        /* update adjacency matrix accordingly */
        self.adjacency_matrix[a][b] -= 1;
        self.adjacency_matrix[b][a] -= 1;

        /* decrement number of edges */
        self.num_edges -= 1;

        Ok(())
    }

    fn order(&self) -> Result<usize, EnceladusError> {
        Ok(self.num_vertices)
    }

    fn size(&self) -> Result<usize, EnceladusError> {
        Ok(self.num_edges)
    }

    fn degree(&self, vertex: VertexNumber) -> Result<usize, EnceladusError> {
        if !self.vertex_labels.contains_key(vertex)? {
            return Err(EnceladusError::VertexNotFound);
        }

        let mut degree: u64 = 0;

        for i in 0..self.num_vertices {
            degree += self.adjacency_matrix[vertex][i];
        }

        Ok(degree as usize)
    }

    fn is_adjacent(
        &self,
        a: VertexNumber,
        b: VertexNumber,
    ) -> Result<bool, EnceladusError> {
        if !(self.vertex_labels.contains_key(a)?
            && self.vertex_labels.contains_key(b)?)
        {
            return Err(EnceladusError::VertexNotFound);
        }

        Ok(self.adjacency_matrix[a][b] > 0)
    }

    fn incident_edges(
        &self,
        vertex: VertexNumber,
    ) -> Result<Vec<EdgeNumber>, EnceladusError> {
        if !self.vertex_labels.contains_key(vertex)? {
            return Err(EnceladusError::VertexNotFound);
        }

        let mut edges: Vec<EdgeNumber> = Vec::new();

        for i in 0..self.num_edges {
            let (a, b): (VertexNumber, VertexNumber) =
                *self.endpoints.get(i)?.unwrap();

            if vertex == a || vertex == b {
                edges.push(i);
            }
        }

        Ok(edges)
    }

    fn is_incident(
        &self,
        vertex: VertexNumber,
        edge: EdgeNumber,
    ) -> Result<bool, EnceladusError> {
        if !self.vertex_labels.contains_key(vertex)? {
            return Err(EnceladusError::VertexNotFound);
        }

        if !self.edge_labels.contains_key(edge)? {
            return Err(EnceladusError::EdgeNotFound);
        }

        let mut incident: bool = false;

        for curr_edge in 0..self.num_edges {
            let (a, b): (VertexNumber, VertexNumber) =
                *self.endpoints.get(curr_edge)?.unwrap();

            if edge == curr_edge && (vertex == a || vertex == b) {
                incident = true;
                break;
            }
        }

        Ok(incident)
    }

    fn neighbours(
        &self,
        vertex: VertexNumber,
    ) -> Result<Vec<VertexNumber>, EnceladusError> {
        if !self.vertex_labels.contains_key(vertex)? {
            return Err(EnceladusError::VertexNotFound);
        }

        let mut neighbours: Vec<VertexNumber> = Vec::new();

        for i in 0..self.num_vertices {
            if self.adjacency_matrix[vertex][i] > 0 {
                neighbours.push(i);
            }
        }

        Ok(neighbours)
    }

    fn endpoints(
        &self,
        edge: EdgeNumber,
    ) -> Result<(VertexNumber, VertexNumber), EnceladusError> {
        if !self.edge_labels.contains_key(edge)? {
            return Err(EnceladusError::EdgeNotFound);
        }

        Ok(*self.endpoints.get(edge)?.unwrap())
    }

    fn clear(&mut self) -> Result<(), EnceladusError> {
        self.vertex_labels.clear()?;
        self.edge_labels.clear()?;
        self.adjacency_matrix.clear();
        self.endpoints.clear()?;
        self.num_vertices = 0;
        self.num_edges = 0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_normal() {
        let actual_graph: AdjMatGraph<u64, u64> = AdjMatGraph::new();

        let expected_graph: AdjMatGraph<u64, u64> = AdjMatGraph {
            num_vertices: 0,
            num_edges: 0,
            adjacency_matrix: vec![],
            endpoints: HashMap::new(),
            vertex_labels: HashMap::new(),
            edge_labels: HashMap::new(),
        };

        assert_eq!(actual_graph, expected_graph);
    }

    #[test]
    fn test_insert_vertex_normal() {
        let some_label: u64 = 33;

        let mut actual_graph: AdjMatGraph<u64, u64> = AdjMatGraph::new();

        let actual_res: Result<VertexNumber, EnceladusError> =
            actual_graph.insert_vertex(some_label);

        let expected_vertex_number: VertexNumber = 0;
        let expected_res: Result<VertexNumber, EnceladusError> =
            Ok(expected_vertex_number);

        let expected_graph: AdjMatGraph<u64, u64> = AdjMatGraph {
            num_vertices: 1,
            num_edges: 0,
            adjacency_matrix: vec![vec![0]],
            endpoints: HashMap::new(),
            vertex_labels: {
                let mut map: HashMap<VertexNumber, u64> = HashMap::new();
                map.insert(0, some_label).unwrap();
                map
            },
            edge_labels: HashMap::new(),
        };

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_graph, expected_graph);
    }

    #[test]
    fn test_insert_two_vertices_normal() {
        let some_labels: Vec<u64> = vec![33, 12];

        let mut actual_graph: AdjMatGraph<u64, u64> = AdjMatGraph::new();

        actual_graph.insert_vertex(some_labels[0]).unwrap();

        let actual_res: Result<VertexNumber, EnceladusError> =
            actual_graph.insert_vertex(some_labels[1]);

        let expected_vertex_number: VertexNumber = 1;
        let expected_res: Result<VertexNumber, EnceladusError> =
            Ok(expected_vertex_number);

        let expected_graph: AdjMatGraph<u64, u64> = AdjMatGraph {
            num_vertices: 2,
            num_edges: 0,
            adjacency_matrix: vec![vec![0, 0], vec![0, 0]],
            endpoints: HashMap::new(),
            vertex_labels: {
                let mut map: HashMap<VertexNumber, u64> = HashMap::new();
                map.insert(0, some_labels[0]).unwrap();
                map.insert(1, some_labels[1]).unwrap();
                map
            },
            edge_labels: HashMap::new(),
        };

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_graph, expected_graph);
    }

    #[test]
    fn test_remove_vertex_normal() {
        let some_label: u64 = 33;

        let mut actual_graph: AdjMatGraph<u64, u64> = AdjMatGraph::new();
        let some_vnum: VertexNumber =
            actual_graph.insert_vertex(some_label).unwrap();

        let actual_res: Result<(), EnceladusError> =
            actual_graph.remove_vertex(some_vnum);

        let expected_res: Result<(), EnceladusError> = Ok(());

        let expected_graph: AdjMatGraph<u64, u64> = AdjMatGraph {
            num_vertices: 0,
            num_edges: 0,
            adjacency_matrix: vec![],
            endpoints: HashMap::new(),
            vertex_labels: HashMap::new(),
            edge_labels: HashMap::new(),
        };

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_graph, expected_graph);
    }

    #[test]
    fn test_insert_edge_normal() {
        let some_vlabels: Vec<u64> = vec![33, 12];
        let some_elabel: u64 = 3;

        let mut actual_graph: AdjMatGraph<u64, u64> = AdjMatGraph::new();
        let vertex_a: VertexNumber =
            actual_graph.insert_vertex(some_vlabels[0]).unwrap();
        let vertex_b: VertexNumber =
            actual_graph.insert_vertex(some_vlabels[1]).unwrap();

        let actual_res: Result<EdgeNumber, EnceladusError> =
            actual_graph.insert_edge(some_elabel, vertex_a, vertex_b);

        let actual_edge_number: EdgeNumber = actual_res.unwrap();

        let expected_edge_number: EdgeNumber = 0;
        let expected_res: Result<EdgeNumber, EnceladusError> =
            Ok(expected_edge_number);

        let expected_graph: AdjMatGraph<u64, u64> = AdjMatGraph {
            num_vertices: 2,
            num_edges: 1,
            adjacency_matrix: vec![vec![0, 1], vec![1, 0]],
            endpoints: {
                let mut map: HashMap<EdgeNumber, (VertexNumber, VertexNumber)> =
                    HashMap::new();
                map.insert(actual_edge_number, (vertex_a, vertex_b))
                    .unwrap();
                map
            },
            vertex_labels: {
                let mut map: HashMap<VertexNumber, u64> = HashMap::new();
                map.insert(0, some_vlabels[0]).unwrap();
                map.insert(1, some_vlabels[1]).unwrap();
                map
            },
            edge_labels: {
                let mut map: HashMap<EdgeNumber, u64> = HashMap::new();
                map.insert(0, some_elabel).unwrap();
                map
            },
        };

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_graph, expected_graph);
    }

    #[test]
    fn test_remove_edge_normal() {
        let some_vlabels: Vec<u64> = vec![33, 12];
        let some_elabel: u64 = 3;

        let mut actual_graph: AdjMatGraph<u64, u64> = AdjMatGraph::new();
        let vertex_a: VertexNumber =
            actual_graph.insert_vertex(some_vlabels[0]).unwrap();
        let vertex_b: VertexNumber =
            actual_graph.insert_vertex(some_vlabels[1]).unwrap();

        let some_enum: EdgeNumber = actual_graph
            .insert_edge(some_elabel, vertex_a, vertex_b)
            .unwrap();

        let actual_res: Result<(), EnceladusError> =
            actual_graph.remove_edge(some_enum);

        let expected_res: Result<(), EnceladusError> = Ok(());

        let expected_graph: AdjMatGraph<u64, u64> = AdjMatGraph {
            num_vertices: 2,
            num_edges: 0,
            adjacency_matrix: vec![vec![0, 0], vec![0, 0]],
            endpoints: HashMap::new(),
            vertex_labels: {
                let mut map: HashMap<VertexNumber, u64> = HashMap::new();
                map.insert(0, some_vlabels[0]).unwrap();
                map.insert(1, some_vlabels[1]).unwrap();
                map
            },
            edge_labels: HashMap::new(),
        };

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_graph, expected_graph);
    }
}
