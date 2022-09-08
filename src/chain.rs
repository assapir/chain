use core::fmt::Debug;
use std::{fmt::Display, rc::Rc};

use crate::node::Node;

#[derive(PartialEq, Eq)]
pub enum Error {
    NotEnoughElements(usize, usize),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughElements(have, asked) => {
                write!(f, "Chain only have {have} elements, requested {asked}.")
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub(crate) struct Chain<T: Copy> {
    root: Option<Rc<Node<T>>>,
    head: Option<Rc<Node<T>>>,
    size: usize,
}

impl<T: Copy> Chain<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            head: None,
            size: 0,
        }
    }

    pub fn commit(&mut self, value: T) {
        if self.size == 0 {
            let node = Rc::new(Node::new([None, None], value));
            self.root = Some(node);
            self.head = self.root.clone();
        } else {
            let node = Rc::new(Node::new([self.head.clone(), None], value));
            self.head = Some(node);
        }
        self.size += 1;
    }

    /// Same as `get_older(0)`, basically.
    pub fn get_head(&self) -> Option<T> {
        self.head.as_ref().map(|f| *f.clone().data)
    }

    pub fn get_older(&self, by: usize) -> Result<T, Error> {
        if by >= self.size {
            return Err(Error::NotEnoughElements(self.size, by));
        }

        let mut i = by;
        let mut node = self.head.clone();
        while i >= 1 {
            let parent = node.unwrap().get_parent();
            node = parent.clone();
            i -= 1;
        }
        Ok(*node.unwrap().data)
    }
}

impl<T: Copy> Default for Chain<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_head_is_none() {
        let chain: Chain<i32> = Chain::new();
        assert_eq!(chain.get_head(), None);
    }

    #[test]
    fn commit_change_head() {
        let mut chain = Chain::new();
        chain.commit(1);
        assert_eq!(chain.get_head(), Some(1));
    }

    #[test]
    fn three_commits_change_head() {
        let mut chain = Chain::new();
        chain.commit(1);
        chain.commit(2);
        chain.commit(3);
        assert_eq!(chain.get_head(), Some(3));
    }

    #[test]
    fn new_get_head_minus_one_is_err() {
        let chain: Chain<i32> = Chain::new();
        assert_eq!(chain.get_older(1), Err(Error::NotEnoughElements(0, 1)));
    }

    #[test]
    fn commit_get_head_minus_one_is_err() {
        let mut chain = Chain::new();
        chain.commit(1);
        assert_eq!(chain.get_older(1), Err(Error::NotEnoughElements(1, 1)));
    }

    #[test]
    fn four_commits_get_head_minus_two() {
        let mut chain = Chain::new();
        chain.commit(1);
        chain.commit(2);
        chain.commit(3);
        chain.commit(4);
        assert_eq!(chain.get_older(2), Ok(2));
    }
}
