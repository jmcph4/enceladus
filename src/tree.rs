use std::fmt::{Debug, Display};

use crate::error::EnceladusError;
use crate::graph::{VertexNumber, EdgeNumber};

pub trait Tree<V: Sized + Clone + Eq + Display + Debug,
    E: Sized + Clone + Eq + Display + Debug>: Clone + Eq + Debug + Display {
    fn new() -> Self;
    
    fn get_vertex(&self, vertex: VertexNumber) ->
        Result<Option<&V>, EnceladusError>;
    fn get_mut_vertex(&mut self, vertex: VertexNumber) ->
        Result<Option<&mut V>, EnceladusError>;
    fn set_vertex(&mut self, vertex: VertexNumber, label: V) ->
        Result<(), EnceladusError>;
    
    fn get_edge(&self, edge: EdgeNumber) -> Result<Option<&E>, EnceladusError>;
    fn get_mut_edge(&mut self, edge: EdgeNumber) ->
        Result<Option<&mut E>, EnceladusError>;
    fn set_edge(&mut self, edge: EdgeNumber, label: E) ->
        Result<(), EnceladusError>;

    fn insert_vertex(&mut self, parent: Option<VertexNumber>, vertex_label: V,
        edge_label: E) -> Result<VertexNumber, EnceladusError>;
    fn remove_vertex(&mut self, vertex: VertexNumber) ->
        Result<(), EnceladusError>;

    fn depth(&self, vertex: VertexNumber) -> Result<usize, EnceladusError>;
    fn height(&self, vertex: VertexNumber) -> Result<usize, EnceladusError>;

    fn parent(&self, vertex: VertexNumber) ->
        Result<Option<VertexNumber>, EnceladusError>;
    fn children(&self, vertex: VertexNumber) ->
        Result<Vec<VertexNumber>, EnceladusError>;

    fn order(&self) -> Result<usize, EnceladusError>; 
    fn size(&self) -> Result<usize, EnceladusError>;
    
    fn degree(&self, vertex: VertexNumber) -> Result<usize, EnceladusError>;
    fn num_children(&self, vertex: VertexNumber) ->
        Result<usize, EnceladusError>;
    fn arity(&self) -> Result<usize, EnceladusError>;

    fn is_parent(&self, a: VertexNumber, b: VertexNumber) ->
        Result<bool, EnceladusError>;
    fn is_child(&self, a: VertexNumber, b: VertexNumber) ->
        Result<bool, EnceladusError>;
    fn is_adjacent(&self, a: VertexNumber, b: VertexNumber) ->
        Result<bool, EnceladusError>;
    fn is_incident(&self, vertex: VertexNumber, edge: EdgeNumber) ->
        Result<bool, EnceladusError>;

    fn parent_edge(&self, vertex: VertexNumber) ->
        Result<Option<EdgeNumber>, EnceladusError>;
    fn child_edges(&self, vertex: VertexNumber) ->
        Result<Vec<EdgeNumber>, EnceladusError>;
    fn endpoints(&self, edge: EdgeNumber) ->
        Result<(VertexNumber, VertexNumber), EnceladusError>;

    fn clear(&mut self) -> Result<(), EnceladusError>;
}

