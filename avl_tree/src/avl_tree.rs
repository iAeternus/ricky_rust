//! ## Brief
//! AVL树是一棵平衡二叉树，通过旋转保证二叉树的平衡
//!
//! ## Author
//! Ricky
//!
//! ## Date
//! 2025/5/26
//!
//! ## Version
//! 1.0

use std::fmt::{Debug, Display};

use crate::{avl_node::{AVLNode, RotMod}, iter::{IntoIter, Iter}};

/// AVL树
///
/// ## Example
/// ```rust
/// use avl_tree::avl_tree::AVLTree;
///
/// let mut t = AVLTree::new();
/// t.insert(2);
/// t.insert(1);
/// t.insert(3);
/// assert_eq!(2, t.height());
/// assert!(t.contains(1));
/// ```
#[derive(Debug)]
pub struct AVLTree<T>(pub(crate) Option<Box<AVLNode<T>>>);

impl<T> AVLTree<T> {
    /// 创建一棵空树
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取树的高度
    pub fn height(&self) -> isize {
        self.0.as_ref().map_or(0, |rt| rt.height)
    }

    /// 更新当前节点的高度
    pub(crate) fn update_height(&mut self) {
        if let Some(ref mut rt) = self.0 {
            rt.height = 1 + std::cmp::max(rt.lch.height(), rt.rch.height());
        }
    }

    /// 对根节点左旋
    fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }

    /// 对根节点右旋
    fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }

    fn rotate(&mut self, rot_mod: RotMod) {
        match rot_mod {
            RotMod::LeftRot => self.rot_left(),
            RotMod::RightRot => self.rot_right(),
            RotMod::NotRot => self.update_height(),
        }
    }

    /// 获取不可变引用迭代器
    pub fn iter(&self) -> Iter<'_, T> {
        let mut iter = Iter::new();
        let mut curr = self.0.as_deref();
        while let Some(node) = curr {
            iter.stack.push(node);
            curr = node.lch.0.as_deref();
        }
        iter
    }
}

impl<T: PartialOrd> AVLTree<T> {
    /// 插入
    pub fn insert(&mut self, data: T) {
        let rot_mod = if let Some(rt) = &mut self.0 {
            if data < rt.data {
                rt.lch.insert(data);
            } else if data > rt.data {
                rt.rch.insert(data);
            } else {
                return; // 重复数据不插入
            }
            rt.rot_mod()
        } else {
            self.0 = Some(Box::new(AVLNode::new(
                data,
                AVLTree::default(),
                AVLTree::default(),
            )));
            RotMod::NotRot
        };
        self.rotate(rot_mod);
    }

    /// 判断数据是否存在
    pub fn contains(&self, data: T) -> bool {
        self.0.as_ref().is_some_and(|rt| {
            if data < rt.data {
                rt.lch.contains(data)
            } else if data > rt.data {
                rt.rch.contains(data)
            } else {
                true
            }
        })
    }
}

/// 默认构造
impl<T> Default for AVLTree<T> {
    fn default() -> Self {
        AVLTree(None)
    }
}

/// 打印
impl<T: Display> Display for AVLTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(rt) => rt.fmt(f),
            None => write!(f, "(empty tree)"),
        }
    }
}

impl<T> IntoIterator for AVLTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut t = AVLTree::new();
        t.insert(4);
        t.insert(5);
        t.insert(6);
        t.insert(10);
        t.insert(1);
        t.insert(94);
        t.insert(54);
        t.insert(3);

        println!("{}", t);

        assert!(t.contains(10));
        assert!(!t.contains(100));
    }
}
