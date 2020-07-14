use std::fmt;
use std::fmt::{Display, Debug};

use crate::error::EnceladusError;
use crate::graph::{VertexNumber, EdgeNumber};
use crate::tree::Tree;

#[derive(Clone, Debug)]
struct ArrayTreeNode<V, E> {
    vlabel: V,
    children: Vec<(Option<E>, ArrayTreeNode<V, E>)>
}

#[derive(Clone, Debug)]
pub struct ArrayTree<V, E> {
    root: Option<ArrayTreeNode<V, E>>
}

impl<V, E> PartialEq for ArrayTreeNode<V, E> where
    V: Sized + Clone + Eq + Display + Debug,
    E: Sized + Clone + Eq + Display + Debug {
    fn eq(&self, other: &Self) -> bool {
        if self.vlabel != other.vlabel ||
            self.children.len() != other.children.len() {
            return false;
        }

        let arity: usize = self.children.len();

        for i in 0..arity {
            if self.children[i] != other.children[i] {
                return false;
            }
        }

        true
    }
}

impl<V, E> Eq for ArrayTreeNode<V, E> where
    V: Sized + Clone + Eq + Display + Debug,
    E: Sized + Clone + Eq + Display + Debug {}

impl<V, E> PartialEq for ArrayTree<V, E> where
    V: Sized + Clone + Eq + Display + Debug,
    E: Sized + Clone + Eq + Display + Debug {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<V, E> Eq for ArrayTree<V, E> where
    V: Sized + Clone + Eq + Display + Debug,
    E: Sized + Clone + Eq + Display + Debug {}

impl<V, E> fmt::Display for ArrayTree<V, E> where
    V: Sized + Clone + Eq + Display + Debug,
    E: Sized + Clone + Eq + Display + Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.root {
            Some(root) => write!(f, "{}", root.vlabel),
            None => write!(f, "null")
        }
    }
}

#[allow(unused)]
impl<V, E> Tree<V, E> for ArrayTree<V, E> where
    V: Sized + Clone + Eq + Display + Debug,
    E: Sized + Clone + Eq + Display + Debug {
    fn new() -> Self {
        Self {
            root: None
        }
    }

    fn get_vertex(&self, vertex: VertexNumber) ->
        Result<Option<&V>, EnceladusError> {
        unimplemented!()
    }

    fn get_mut_vertex(&mut self, vertex: VertexNumber) ->
        Result<Option<&mut V>, EnceladusError> {
        unimplemented!()
    }

    fn set_vertex(&mut self, vertex: VertexNumber, label: V) ->
        Result<(), EnceladusError> {
        unimplemented!()
    }

    fn get_edge(&self, edge: EdgeNumber) -> Result<Option<&E>, EnceladusError> {
        unimplemented!()
    }

    fn get_mut_edge(&mut self, edge: EdgeNumber) ->
        Result<Option<&mut E>, EnceladusError> {
        unimplemented!()
    }

    fn set_edge(&mut self, edge: EdgeNumber, label: E) ->
        Result<(), EnceladusError> {
        unimplemented!()
    }

    fn insert_vertex(&mut self, parent: Option<VertexNumber>, vertex_label: V,
        edge_label: E) -> Result<VertexNumber, EnceladusError> {
        unimplemented!()
    }

    fn remove_vertex(&mut self, vertex: VertexNumber) ->
        Result<(), EnceladusError> {
        unimplemented!()
    }

    fn depth(&self, vertex: VertexNumber) -> Result<usize, EnceladusError> {
        unimplemented!()
    }

    fn height(&self, vertex: VertexNumber) -> Result<usize, EnceladusError> {
        unimplemented!()
    }

    fn parent(&self, vertex: VertexNumber) ->
        Result<Option<VertexNumber>, EnceladusError> {
        unimplemented!()
    }

    fn children(&self, vertex: VertexNumber) ->
        Result<Vec<VertexNumber>, EnceladusError> {
        unimplemented!()
    }

    fn order(&self) -> Result<usize, EnceladusError> {
        unimplemented!()
    }
 
    fn size(&self) -> Result<usize, EnceladusError> {
        unimplemented!()
    }

    
    fn degree(&self, vertex: VertexNumber) -> Result<usize, EnceladusError> {
        unimplemented!()
    }

    fn num_children(&self, vertex: VertexNumber) ->
        Result<usize, EnceladusError> {
        unimplemented!()
    }

    fn arity(&self) -> Result<usize, EnceladusError> {
        unimplemented!()
    }

    fn is_parent(&self, a: VertexNumber, b: VertexNumber) ->
        Result<bool, EnceladusError> {
        unimplemented!()
    }

    fn is_child(&self, a: VertexNumber, b: VertexNumber) ->
        Result<bool, EnceladusError> {
        unimplemented!()
    }

    fn is_adjacent(&self, a: VertexNumber, b: VertexNumber) ->
        Result<bool, EnceladusError> {
        unimplemented!()
    }

    fn is_incident(&self, vertex: VertexNumber, edge: EdgeNumber) ->
        Result<bool, EnceladusError> {
        unimplemented!()
    }

    fn parent_edge(&self, vertex: VertexNumber) ->
        Result<Option<EdgeNumber>, EnceladusError> {
        unimplemented!()
    }

    fn child_edges(&self, vertex: VertexNumber) ->
        Result<Vec<EdgeNumber>, EnceladusError> {
        unimplemented!()
    }

    fn endpoints(&self, edge: EdgeNumber) ->
        Result<(VertexNumber, VertexNumber), EnceladusError> {
        unimplemented!()
    }

    fn clear(&mut self) -> Result<(), EnceladusError> {
        unimplemented!()
    }
}

