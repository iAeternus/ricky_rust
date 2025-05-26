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

/// AVL树节点
#[derive(Debug)]
pub struct AVLNode<T> {
    data: T,
    height: isize,
    lch: AVLTree<T>,
    rch: AVLTree<T>,
}

/// 旋转模式
#[derive(Debug)]
pub enum RotMod {
    LeftRot,
    RightRot,
    NotRot,
}

impl<T> AVLNode<T> {
    /// 创建一个新节点
    pub fn new(data: T, lch: AVLTree<T>, rch: AVLTree<T>) -> Self {
        Self {
            data,
            height: 0,
            lch,
            rch,
        }
    }

    /// 获取节点旋转模式
    fn rot_mod(&self) -> RotMod {
        if self.lch.height() - self.rch.height() > 1 {
            RotMod::RightRot
        } else if self.rch.height() - self.lch.height() > 1 {
            RotMod::LeftRot
        } else {
            RotMod::NotRot
        }
    }

    /// 左旋
    fn rot_left(mut self) -> Box<Self> {
        let mut right = match self.rch.0.take() {
            Some(right) => right,
            None => return Box::new(self),
        };

        self.rch = AVLTree(right.lch.0.take());
        self.rch.update_height();
        right.lch = AVLTree(Some(Box::new(self)));
        right.height = 1 + std::cmp::max(right.lch.height(), right.rch.height());
        right
    }

    /// 右旋
    fn rot_right(mut self) -> Box<Self> {
        let mut left = match self.lch.0.take() {
            Some(left) => left,
            None => return Box::new(self),
        };

        self.lch = AVLTree(left.rch.0.take());
        self.lch.update_height();
        left.rch = AVLTree(Some(Box::new(self)));
        left.height = 1 + std::cmp::max(left.lch.height(), left.rch.height());
        left
    }
}

impl<T: Display> AVLNode<T> {
    /// 节点打印支持函数
    fn fmt_helper(
        &self,
        prefix: &str,
        is_left: bool,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let connector = if is_left { "└── " } else { "├── " };
        writeln!(f, "{}{}{}", prefix, connector, self.data)?;

        let new_prefix = format!("{}{}", prefix, if is_left { "    " } else { "│   " });
        if let Some(ref left) = self.lch.0 {
            left.fmt_helper(&new_prefix, false, f)?;
        }
        if let Some(ref right) = self.rch.0 {
            right.fmt_helper(&new_prefix, true, f)?;
        }
        Ok(())
    }
}

/// 节点打印
impl<T: Display> Display for AVLNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper("", true, f)
    }
}

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
pub struct AVLTree<T>(Option<Box<AVLNode<T>>>);

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
    fn update_height(&mut self) {
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
}

impl<T> Default for AVLTree<T> {
    fn default() -> Self {
        AVLTree(None)
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

impl<T: Display> Display for AVLTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(rt) => rt.fmt(f),
            None => write!(f, "(empty tree)"),
        }
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
