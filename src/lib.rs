#[cfg(test)]
mod tests {
  #[test]
  fn empty() {
    let test_tree = crate::BinaryTree::<i32>::new(None);

    assert!(test_tree.root().is_none());
  }

  #[test]
  fn only_root() {
    let test_tree = crate::BinaryTree::new(Some(5));

    assert!(test_tree.root().is_some());
    assert_eq!(test_tree.root().unwrap().value(), &5);
    assert_eq!(test_tree.root().unwrap().has_left(), false);
    assert_eq!(test_tree.root().unwrap().has_right(), false);
  }

  #[test]
  fn root_has_left() {
    let mut test_tree = crate::BinaryTree::new(Some(5));
    test_tree.insert(4);

    assert!(test_tree.root().is_some());
    assert_eq!(test_tree.root().unwrap().value(), &5);
    assert_eq!(test_tree.root().unwrap().has_left(), true);
    assert_eq!(test_tree.root().unwrap().has_right(), false);

    let node = test_tree.search(4);

    assert!(node.is_some());
    assert_eq!(node.unwrap().has_left(), false);
    assert_eq!(node.unwrap().has_right(), false);
  }

  #[test]
  fn root_has_right() {
    let mut test_tree = crate::BinaryTree::new(Some(5));
    test_tree.insert(6);

    assert!(test_tree.root().is_some());
    assert_eq!(test_tree.root().unwrap().value(), &5);
    assert_eq!(test_tree.root().unwrap().has_left(), false);
    assert_eq!(test_tree.root().unwrap().has_right(), true);

    let node = test_tree.search(6);

    assert!(node.is_some());
    assert_eq!(node.unwrap().has_left(), false);
    assert_eq!(node.unwrap().has_right(), false);
  }

  #[test]
  fn root_has_both() {
    let mut test_tree = crate::BinaryTree::new(Some(5));
    test_tree.insert(4);
    test_tree.insert(6);

    assert!(test_tree.root().is_some());
    assert_eq!(test_tree.root().unwrap().value(), &5);
    assert_eq!(test_tree.root().unwrap().has_left(), true);
    assert_eq!(test_tree.root().unwrap().has_right(), true);

    let left_node = test_tree.search(4);

    assert!(left_node.is_some());
    assert_eq!(left_node.unwrap().has_left(), false);
    assert_eq!(left_node.unwrap().has_right(), false);

    let right_node = test_tree.search(6);

    assert!(right_node.is_some());
    assert_eq!(right_node.unwrap().has_left(), false);
    assert_eq!(right_node.unwrap().has_right(), false);
  }

  #[test]
  fn leaf_has_left() {
    let mut test_tree = crate::BinaryTree::new(Some(5));
    test_tree.insert(4);
    test_tree.insert(3);

    assert!(test_tree.root().is_some());
    assert_eq!(test_tree.root().unwrap().value(), &5);
    assert_eq!(test_tree.root().unwrap().has_left(), true);
    assert_eq!(test_tree.root().unwrap().has_right(), false);

    let left_node = test_tree.search(4);

    assert!(left_node.is_some());
    assert_eq!(left_node.unwrap().has_left(), true);
    assert_eq!(left_node.unwrap().has_right(), false);

    let left_node2 = test_tree.search(3);

    assert!(left_node2.is_some());
    assert_eq!(left_node2.unwrap().has_left(), false);
    assert_eq!(left_node2.unwrap().has_right(), false);
  }
}

pub struct BinaryTree<T: Ord> {
  root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinaryTree<T> {
  pub fn new(root: Option<T>) -> BinaryTree<T> {
    BinaryTree::<T> {
      root: root.and_then(|value| Some(Box::new(Node::new(value)))),
    }
  }

  pub fn root(&self) -> Option<&Box<Node<T>>> {
    self.root.as_ref()
  }

  pub fn insert(&mut self, value: T) {
    match &mut self.root {
      Some(root_value) => {
        root_value.insert(value)
      },
      None => self.root = Some(Box::new(Node::new(value)))
    }
  }

  pub fn search(&self, value: T) -> Option<&Box<Node<T>>> {
    match &self.root {
      Some(ref root_value) => self.search_node(value, root_value),
      None => None,
    }
  }

  fn search_node<'a>(&self, value: T, leaf: &'a Box<Node<T>>) -> Option<&'a Box<Node<T>>> {
    if value < leaf.value {
      match &leaf.left {
        Some(node) => self.search_node(value, node),
        None => None,
      }
    } else if value > leaf.value {
      match &leaf.right {
        Some(node) => self.search_node(value, node),
        None => None,
      }
    } else {
      Some(leaf)
    }
  }
}

pub struct Node<T: Ord> {
  pub value: T,
  left: Option<Box<Node<T>>>,
  right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
  fn new(value: T) -> Node<T> {
    Node::<T> {
      value,
      left: None,
      right: None,
    }
  }

  fn insert(&mut self, value: T) {
    if value < self.value {
      if let Some(node) = &mut self.left {
        node.insert(value)
      } else {
        self.left = Some(Box::new(Node::new(value)))
      }
    } else if value > self.value {
      if let Some(node) = &mut self.right {
        node.insert(value)
      } else {
        self.right = Some(Box::new(Node::new(value)))
      }
    }
  }

  pub fn value(&self) -> &T {
    &self.value
  }

  pub fn has_left(&self) -> bool {
    self.left.is_some()
  }

  pub fn has_right(&self) -> bool {
    self.right.is_some()
  }
}
