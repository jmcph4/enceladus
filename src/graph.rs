use std::fmt::{Debug, Display};

use crate::error::EnceladusError;

pub type VertexNumber = usize;
pub type EdgeNumber = usize;

pub trait Graph<V: Sized + Clone + Eq + Display + Debug,
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
    
    fn insert_vertex(&mut self, label: V) ->
        Result<VertexNumber, EnceladusError>;
    fn remove_vertex(&mut self, vertex: VertexNumber) ->
        Result<(), EnceladusError>;
    
    fn insert_edge(&mut self, label: E, a: VertexNumber, b: VertexNumber) ->
        Result<EdgeNumber, EnceladusError>;
    fn remove_edge(&mut self, edge: EdgeNumber) -> Result<(), EnceladusError>;
   
    fn order(&self) -> Result<usize, EnceladusError>; 
    fn size(&self) -> Result<usize, EnceladusError>;
    
    fn degree(&self, vertex: VertexNumber) -> Result<usize, EnceladusError>;
    fn is_adjacent(&self, a: VertexNumber, b: VertexNumber) ->
        Result<bool, EnceladusError>;
    fn is_incident(&self, vertex: VertexNumber, edge: EdgeNumber) ->
        Result<bool, EnceladusError>;

    fn neighbours(&self, vertex: VertexNumber) ->
        Result<Vec<VertexNumber>, EnceladusError>;
    fn endpoints(&self, edge: EdgeNumber) ->
        Result<(VertexNumber, VertexNumber), EnceladusError>;

    fn clear(&mut self) -> Result<(), EnceladusError>;
}

