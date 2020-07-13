use std::fmt;
use std::fmt::{Display, Debug};

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

