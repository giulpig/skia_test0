use anyhow::Result;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::rc::Rc;

/////////////////////////////////////////////////////////
//                      Struct definitions
/////////////////////////////////////////////////////////

#[derive(Debug)]
struct TreeNode {
    id: u32,
    parent: Option<Rc<RefCell<TreeNode>>>,
    children: Vec<Rc<RefCell<TreeBranch>>>,
}

#[derive(Debug)]
struct TreeBranch {
    dst: Rc<RefCell<TreeNode>>,
    weigth: u32,
}

#[derive(Debug)]
struct TreeBranchRaw {
    src: u32,
    dst: u32,
    weigth: u32,
}

#[derive(Debug)]
struct Tree {
    root: Rc<RefCell<TreeNode>>,
    nodes: HashMap<u32, Rc<RefCell<TreeNode>>>,
}

struct TreeIterator<'a> {
    tree: &'a Tree,
    visit_list: VecDeque<Rc<RefCell<TreeNode>>>,
    iteration_kind: TreeIteratorKind,
}

enum TreeIteratorKind {
    Bfs,
    Dfs,
}

/////////////////////////////////////////////////////////
//                      Struct Implementations
/////////////////////////////////////////////////////////

impl Tree {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(TreeNode {
            id: 0,
            parent: None,
            children: vec![],
        }));
        Tree {
            root: root.clone(),
            nodes: [(0, root.clone())].into_iter().collect(),
        }
    }

    fn add_child(&mut self, parent_id: u32, child: TreeNode, weigth: u32) -> anyhow::Result<()> {
        if !self.nodes.contains_key(&parent_id) {
            return Err(anyhow::anyhow!("parent node not in the tree"));
        }
        if self.nodes.contains_key(&child.id) {
            return Err(anyhow::anyhow!("child node already in the tree"));
        }

        let branch = TreeBranch {
            dst: Rc::new(RefCell::new(child)),
            weigth,
        };
        self.nodes
            .get(&parent_id)
            .unwrap()
            .borrow_mut()
            .children
            .push(Rc::new(RefCell::new(branch)));
        Ok(())
    }

    fn iter_dfs(&self) -> TreeIterator {
        TreeIterator {
            tree: &self,
            visit_list: VecDeque::from(vec![self.root.clone()]),
            iteration_kind: TreeIteratorKind::Dfs,
        }
    }
    fn iter_bfs(&self) -> TreeIterator {
        TreeIterator {
            tree: &self,
            visit_list: VecDeque::from(vec![self.root.clone()]),
            iteration_kind: TreeIteratorKind::Dfs,
        }
    }
    fn from_branch_list(branch_list_raw: &[TreeBranchRaw]) -> Self {
        let node_map: HashMap<u32, Rc<RefCell<TreeNode>>> = branch_list_raw
            .iter()
            .map(|branch_raw| {
                [
                    (
                        branch_raw.src,
                        Rc::new(RefCell::new(TreeNode {
                            id: branch_raw.src,
                            parent: None,
                            children: vec![],
                        })),
                    ),
                    (
                        branch_raw.dst,
                        Rc::new(RefCell::new(TreeNode {
                            id: branch_raw.dst,
                            parent: None,
                            children: vec![],
                        })),
                    ),
                ]
            })
            .flatten()
            .collect();

        for branch_raw in branch_list_raw.iter() {
            dbg!(&branch_raw);
            // Assign parent
            node_map
                .get(&branch_raw.dst)
                .unwrap()
                .borrow_mut()
                .parent
                .as_mut()
                .map_or_else(
                    || node_map.get(&branch_raw.src).unwrap().clone(),
                    |_| panic!("multiple branches pointing to a node"),
                );

            // Assign children
            node_map
                .get(&branch_raw.src)
                .unwrap()
                .borrow_mut()
                .children
                .push(Rc::new(RefCell::new(TreeBranch {
                    dst: node_map.get(&branch_raw.dst).unwrap().clone(),
                    weigth: branch_raw.weigth,
                })));
        }

        // Create tree and place root
        let mut tree = Tree::new();
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;
        for node in node_map.into_iter() {
            if node.1.borrow_mut().parent.is_none() {
                if root.is_none() {
                    root = Some(node.1)
                } else {
                    panic!("multiple roots found");
                }
            }
        }
        tree.root = root.expect("no root found");

        tree
    }
}

/////////////////////////////////////////////////////////
//                      Trait Implementations
/////////////////////////////////////////////////////////

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Rc<RefCell<TreeNode>>;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.visit_list.pop_front() {
            None => None,
            Some(node_to_visit) => {
                match self.iteration_kind {
                    TreeIteratorKind::Bfs => {
                        node_to_visit.borrow().children.iter().for_each(|branch| {
                            self.visit_list.push_back(branch.borrow().dst.clone())
                        });
                    }
                    TreeIteratorKind::Dfs => {
                        node_to_visit.borrow().children.iter().for_each(|branch| {
                            self.visit_list.push_front(branch.borrow().dst.clone())
                        });
                    }
                }
                Some(node_to_visit)
            }
        };
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, rhs: &Self) -> bool {
        return self.id == rhs.id;
    }
}
impl Eq for TreeNode {}

/////////////////////////////////////////////////////////
//                            Tests
/////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::*;

    fn get_example_tree() -> Tree {
        let branches: Vec<TreeBranchRaw> = vec![
            TreeBranchRaw {
                src: 0,
                dst: 1,
                weigth: 1,
            },
            TreeBranchRaw {
                src: 0,
                dst: 2,
                weigth: 1,
            },
        ];

        Tree::from_branch_list(&branches)
    }

    #[test]
    fn add_child() {
        let mut tree = Tree::new();

        let child = TreeNode {
            id: 69,
            parent: Some(Rc::clone(&tree.root)),
            children: vec![],
        };

        let root_id = tree.root.borrow().id;
        tree.add_child(root_id, child, 1).unwrap();
    }

    #[test]
    fn iterate_tree() {
        let tree = get_example_tree();

        for node in tree.iter_dfs() {
            dbg!(node.borrow().id);
        }
    }
}
