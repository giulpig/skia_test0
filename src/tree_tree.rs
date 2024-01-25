use std::cell::RefCell;
use std::rc::UniqueRc;

struct TreeBranch {
    weigth: u32,
    child: UniqueRc<RefCell<TreeNode>>,
}

struct TreeNode {
    id: u32,
    children_branches: Vec<TreeBranch>,
}

struct Tree {
    root: TreeNode,
}

impl Tree {
    fn new(root: TreeNode) -> Self {
        Self { root }
    }
    fn traverse(&self) {}
}

impl TreeNode {
    fn new() -> Self {
        Self {
            children_branches: vec![],
        }
    }
    fn add_child(&mut self, child: TreeNode, weigth: u32) {
        self.children_branches.push(TreeBranch {
            weigth,
            child: UniqueRc::new(RefCell::new(child)),
        });
    }
    fn add_child_branch(&mut self, child_branch: TreeBranch) {
        self.children_branches.push(child_branch);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_creation() {
        let root = TreeNode::new();
        let mut tree = Tree::new(root);
        tree.root.add_child(TreeNode::new(), 1);
        tree.root
            .children_branches
            .get(0)
            .unwrap()
            .child
            .borrow_mut()
            .add_child(TreeNode::new(), 2);
    }
}
