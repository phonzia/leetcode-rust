use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn get_balance_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<u32> {
    match root {
        Some(root) => {
            let ld: u32;
            let rd: u32;

            if let Some(left_depth) = get_balance_depth(&root.borrow().left) {
                ld = left_depth;
            } else {
                return None;
            }

            if let Some(right_depth) = get_balance_depth(&root.borrow().right) {
                rd = right_depth;
            } else {
                return None;
            }

            match rd > ld {
                true => match rd - ld > 1 {
                    true => return None,
                    false => return Some(rd + 1),
                },
                false => match ld - rd > 1 {
                    true => return None,
                    false => return Some(ld + 1),
                },
            }
        }
        None => return Some(0),
    }
}

fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(_depth) = get_balance_depth(&root) {
        return true;
    }

    return false;
}

fn main() {
    let tree: Option<Rc<RefCell<TreeNode>>> = None;
    println!("{}", is_balanced(tree));
}
