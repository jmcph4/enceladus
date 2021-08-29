use std::fmt::{Debug, Display};

use crate::error::EnceladusError;

pub trait Graph<V: Sized + Clone + Eq + Display + Debug, E: Sized + Clone + Eq + Display + Debug>:
    Clone + Eq + Debug + Display
{
    type VertexNumber;
    type EdgeNumber;

    fn new() -> Self;

    fn get_vertex(&self, vertex: Self::VertexNumber) -> Result<Option<&V>, EnceladusError>;
    fn get_mut_vertex(
        &mut self,
        vertex: Self::VertexNumber,
    ) -> Result<Option<&mut V>, EnceladusError>;
    fn set_vertex(&mut self, vertex: Self::VertexNumber, label: V) -> Result<(), EnceladusError>;

    fn get_edge(&self, edge: Self::EdgeNumber) -> Result<Option<&E>, EnceladusError>;
    fn get_mut_edge(&mut self, edge: Self::EdgeNumber) -> Result<Option<&mut E>, EnceladusError>;
    fn set_edge(&mut self, edge: Self::EdgeNumber, label: E) -> Result<(), EnceladusError>;

    fn insert_vertex(&mut self, label: V) -> Result<Self::VertexNumber, EnceladusError>;
    fn remove_vertex(&mut self, vertex: Self::VertexNumber) -> Result<(), EnceladusError>;

    fn insert_edge(
        &mut self,
        label: E,
        a: Self::VertexNumber,
        b: Self::VertexNumber,
    ) -> Result<Self::EdgeNumber, EnceladusError>;
    fn remove_edge(&mut self, edge: Self::EdgeNumber) -> Result<(), EnceladusError>;

    fn order(&self) -> Result<usize, EnceladusError>;
    fn size(&self) -> Result<usize, EnceladusError>;

    fn degree(&self, vertex: Self::VertexNumber) -> Result<usize, EnceladusError>;
    fn is_adjacent(
        &self,
        a: Self::VertexNumber,
        b: Self::VertexNumber,
    ) -> Result<bool, EnceladusError>;
    fn is_incident(
        &self,
        vertex: Self::VertexNumber,
        edge: Self::EdgeNumber,
    ) -> Result<bool, EnceladusError>;

    fn neighbours(
        &self,
        vertex: Self::VertexNumber,
    ) -> Result<Vec<Self::VertexNumber>, EnceladusError>;
    fn incident_edges(
        &self,
        vertex: Self::VertexNumber,
    ) -> Result<Vec<Self::EdgeNumber>, EnceladusError>;
    fn endpoints(
        &self,
        edge: Self::EdgeNumber,
    ) -> Result<(Self::VertexNumber, Self::VertexNumber), EnceladusError>;

    fn clear(&mut self) -> Result<(), EnceladusError>;
}
