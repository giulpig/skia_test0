use rug::float::OrdFloat;
use rug::Float;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

/////////////////////////////////////////////////////////
//                          Consts
/////////////////////////////////////////////////////////
const FLOAT_PRECISION: u32 = 300;

/////////////////////////////////////////////////////////
//                          Typedefs
/////////////////////////////////////////////////////////
type TreeNodeRef = Rc<RefCell<TreeNode>>;
type TreeBranchRef = Rc<RefCell<TreeBranch>>;

/////////////////////////////////////////////////////////
//                     Utility functions
/////////////////////////////////////////////////////////
fn get_ord_float(val: f64) -> OrdFloat {
    Float::with_val(FLOAT_PRECISION, val).into()
}

/////////////////////////////////////////////////////////
//                      Struct definitions
/////////////////////////////////////////////////////////

#[derive(Debug)]
struct TreeNode {
    id: OrdFloat,
    parent: Option<TreeNodeRef>,
    children: Vec<TreeBranchRef>,
}

#[derive(Debug)]
struct TreeBranch {
    dst: TreeNodeRef,
    weigth: OrdFloat,
}

#[derive(Debug)]
struct TreeBranchRaw {
    src: OrdFloat,
    dst: OrdFloat,
    weigth: OrdFloat,
}

#[derive(Debug)]
struct Tree {
    root: TreeNodeRef,
    nodes: HashMap<OrdFloat, TreeNodeRef>,
}

struct TreeIterator {
    visit_list: VecDeque<TreeNodeRef>,
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
            id: get_ord_float(0.),
            parent: None,
            children: vec![],
        }));
        Tree {
            root: root.clone(),
            nodes: [(get_ord_float(0.), root.clone())].into_iter().collect(),
        }
    }

    fn add_child(
        &mut self,
        parent_id: OrdFloat,
        child: TreeNode,
        weigth: OrdFloat,
    ) -> anyhow::Result<()> {
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
            visit_list: VecDeque::from(vec![self.root.clone()]),
            iteration_kind: TreeIteratorKind::Dfs,
        }
    }
    fn iter_bfs(&self) -> TreeIterator {
        TreeIterator {
            visit_list: VecDeque::from(vec![self.root.clone()]),
            iteration_kind: TreeIteratorKind::Bfs,
        }
    }
    fn from_branch_list(branch_list_raw: &[TreeBranchRaw]) -> Self {
        let node_map: HashMap<OrdFloat, TreeNodeRef> = branch_list_raw
            .iter()
            .map(|branch_raw| {
                [
                    (
                        branch_raw.src.clone(),
                        Rc::new(RefCell::new(TreeNode {
                            id: branch_raw.src.clone(),
                            parent: None,
                            children: vec![],
                        })),
                    ),
                    (
                        branch_raw.dst.clone(),
                        Rc::new(RefCell::new(TreeNode {
                            id: branch_raw.dst.clone(),
                            parent: None,
                            children: vec![],
                        })),
                    ),
                ]
            })
            .flatten()
            .collect();

        for branch_raw in branch_list_raw.iter() {
            if node_map
                .get(&branch_raw.dst)
                .unwrap()
                .borrow_mut()
                .parent
                .is_some()
            {
                panic!("multiple branches pointing to a node");
            }

            // Assign parent
            let _ = &node_map
                .get(&branch_raw.dst)
                .unwrap()
                .borrow_mut()
                .parent
                .get_or_insert_with(|| node_map.get(&branch_raw.src).unwrap().clone());

            // Assign children
            node_map
                .get(&branch_raw.src)
                .unwrap()
                .borrow_mut()
                .children
                .push(Rc::new(RefCell::new(TreeBranch {
                    dst: node_map.get(&branch_raw.dst).unwrap().clone(),
                    weigth: branch_raw.weigth.clone(),
                })));
        }

        // Create tree and place root
        let mut tree = Tree::new();
        let mut root: Option<TreeNodeRef> = None;
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

    fn insert_bst(&mut self, val: &OrdFloat) {
        let mut node_to_compare = self.root.clone();
        loop {
            match node_to_compare.borrow().id.cmp(&val) {
                std::cmp::Ordering::Equal => return,
                std::cmp::Ordering::Less => {
                    let children = &node_to_compare.borrow().children;
                    if let Some(first_child) = children.first() {
                        node_to_compare = first_child.borrow().dst.clone();
                        continue;
                    } else {
                    }
                }
                std::cmp::Ordering::Greater => {
                    if let Some(last_child) = node_to_compare.borrow().children.last() {}
                }
            }
        }
    }
}

/////////////////////////////////////////////////////////
//                   Trait Implementations
/////////////////////////////////////////////////////////

impl Iterator for TreeIterator {
    type Item = TreeNodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.visit_list.pop_front().map(|node_to_visit| {
            match self.iteration_kind {
                TreeIteratorKind::Bfs => {
                    node_to_visit
                        .borrow()
                        .children
                        .iter()
                        .for_each(|branch| self.visit_list.push_back(branch.borrow().dst.clone()));
                }
                TreeIteratorKind::Dfs => {
                    node_to_visit
                        .borrow()
                        .children
                        .iter()
                        .for_each(|branch| self.visit_list.push_front(branch.borrow().dst.clone()));
                }
            }
            node_to_visit
        })
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

    fn get_simple_tree() -> Tree {
        let branches: Vec<TreeBranchRaw> = vec![
            TreeBranchRaw {
                src: get_ord_float(0.),
                dst: get_ord_float(1.),
                weigth: get_ord_float(1.),
            },
            TreeBranchRaw {
                src: get_ord_float(0.),
                dst: get_ord_float(2.),
                weigth: get_ord_float(1.),
            },
        ];

        Tree::from_branch_list(&branches)
    }

    #[test]
    fn example_tree() {
        get_simple_tree();
    }

    #[test]
    fn add_child() {
        let mut tree = Tree::new();

        let child = TreeNode {
            id: get_ord_float(69.),
            parent: Some(Rc::clone(&tree.root)),
            children: vec![],
        };

        let root_id = tree.root.borrow().id.clone();
        tree.add_child(root_id, child, get_ord_float(1.)).unwrap();
    }

    #[test]
    fn iterate_tree() {
        let tree = get_simple_tree();

        for node in tree.iter_dfs() {
            //dbg!(node.borrow().id);
            drop(node);
        }

        for node in tree.iter_bfs() {
            //dbg!(node.borrow().id);
            drop(node);
        }
    }
}
