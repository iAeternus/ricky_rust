use std::fmt::Display;

use crate::avl_tree::AVLTree;

/// AVL树节点
#[derive(Debug)]
pub struct AVLNode<T> {
    pub(crate) data: T,
    pub(crate) height: isize,
    pub(crate) lch: AVLTree<T>,
    pub(crate) rch: AVLTree<T>,
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
    pub(crate) fn rot_mod(&self) -> RotMod {
        if self.lch.height() - self.rch.height() > 1 {
            RotMod::RightRot
        } else if self.rch.height() - self.lch.height() > 1 {
            RotMod::LeftRot
        } else {
            RotMod::NotRot
        }
    }

    /// 左旋
    pub(crate) fn rot_left(mut self) -> Box<Self> {
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
    pub(crate) fn rot_right(mut self) -> Box<Self> {
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