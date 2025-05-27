use crate::{avl_node::AvlNode, avl_tree::AvlTree};

/// AVL树不可变引用迭代器
pub struct Iter<'a, T> {
    pub(crate) stack: Vec<&'a AvlNode<T>>,
}

impl<T> Iter<'_, T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        // 处理右子树
        let mut curr = node.rch.0.as_deref();
        while let Some(child) = curr {
            self.stack.push(child);
            curr = child.lch.0.as_deref();
        }
        Some(&node.data)
    }
}

/// AVL树所有权转移迭代器
pub struct IntoIter<T> {
    stack: Vec<AvlNode<T>>,
}

impl<T> IntoIter<T> {
    pub fn new(tree: AvlTree<T>) -> Self {
        let mut iter = Self { stack: vec![] };
        iter.push_left(tree.0);
        iter
    }

    fn push_left(&mut self, mut node: Option<Box<AvlNode<T>>>) {
        while let Some(mut boxed_node) = node {
            let left = boxed_node.lch.0.take();
            node = left;
            self.stack.push(*boxed_node);
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop()?;
        self.push_left(node.rch.0.take());
        Some(node.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        // Given
        let mut t = AvlTree::new();
        t.insert(4);
        t.insert(2);
        t.insert(6);
        t.insert(1);
        t.insert(3);
        t.insert(5);
        t.insert(7);

        // When
        let nums: Vec<_> = t.iter().copied().collect();

        // Then
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], nums);
    }

    #[test]
    fn test_into_iter() {
        // Given
        let mut t = AvlTree::new();
        t.insert(4);
        t.insert(2);
        t.insert(6);
        t.insert(1);
        t.insert(3);
        t.insert(5);
        t.insert(7);

        // When
        let nums: Vec<_> = t.into_iter().collect();

        // Then
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], nums);
    }
}
