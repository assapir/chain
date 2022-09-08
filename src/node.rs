use std::rc::Rc;

type NodeData<T> = Option<Rc<Node<T>>>;
pub(crate) struct Node<T: Copy> {
    pub data: Rc<T>,
    parents: [NodeData<T>; 2],
}

impl<T: Copy> Node<T> {
    pub(crate) fn new(parents: [NodeData<T>; 2], data: T) -> Self {
        Self {
            parents,
            data: Rc::new(data),
        }
    }

    pub(crate) fn get_parent(&self) -> Option<Rc<Node<T>>> {
        self.parents[0].as_ref().cloned()
    }
}
