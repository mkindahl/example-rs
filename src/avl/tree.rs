// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you
// may not use this file except in compliance with the License.  You
// may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.  See the License for the specific language governing
// permissions and limitations under the License.

use std::cmp::max;
use std::fmt::Display;
use std::result;

#[derive(PartialEq, Debug)]
pub enum Error {
    NotFound,
}

pub type Result<T> = result::Result<T, Error>;

struct Node<K, V> {
    key: K,
    value: V,
    height: isize,
    left: Option<Inner<K, V>>,
    right: Option<Inner<K, V>>,
}

/// Convenience declaration of an inner subtree
type Inner<K, V> = Box<Node<K, V>>;

/// Helper function to compute height of an inner tree, which can be
/// optional.
fn height<K, V>(node: &Option<Inner<K, V>>) -> isize {
    node.as_ref().map_or(0, |n| n.height)
}

//     (4)           (2)
//     / \           / \
//   (2) [5]  ==>  [1] (4)
//   / \               / \
// [1] [3]           [3] [5]
fn rotate_right<K, V>(mut root: Inner<K, V>) -> Inner<K, V>
where
    K: PartialOrd,
{
    let mut new_root = root.left.take().expect("no left subtree");
    root.left = new_root.right.take();
    root.update_height();
    new_root.right = Some(root);
    new_root.update_height();
    new_root
}

//    (2)              (4)
//    / \              / \
//  [1] (4)    ==>   (2) [5]
//      / \          / \
//    [3] [5]      [1] [3]
fn rotate_left<K, V>(mut root: Inner<K, V>) -> Inner<K, V>
where
    K: PartialOrd,
{
    let mut new_root = root.right.take().expect("no left subtree");
    root.right = new_root.left.take();
    root.update_height();
    new_root.left = Some(root);
    new_root.update_height();
    new_root
}

/// Rebalance an inner tree, returning the new root.
fn rebalance<K, V>(mut root: Inner<K, V>) -> Inner<K, V>
where
    K: PartialOrd,
{
    let balance = height(&root.left) - height(&root.right);
    if balance > 1 {
        let rheight = height(&root.left.as_ref().unwrap().right);
        let lheight = height(&root.left.as_ref().unwrap().left);
        if rheight > lheight {
            root.left = Some(rotate_left(root.left.take().unwrap()));
        }
        rotate_right(root)
    } else if balance < -1 {
        let rheight = height(&root.right.as_ref().unwrap().right);
        let lheight = height(&root.right.as_ref().unwrap().left);
        if rheight < lheight {
            root.right = Some(rotate_right(root.right.take().unwrap()));
        }
        rotate_left(root)
    } else {
        root
    }
}

/// Take out the smallest node from an inner subtree.
///
/// Returns the smallest node and the new tree resulting from removing
/// the smallest node. The smallest node will always be defined since
/// the tree is non-empty, but the resulting tree can become an empty
/// tree.
fn take_smallest<K, V>(mut root: Inner<K, V>) -> (Option<Inner<K, V>>, Inner<K, V>) {
    if let Some(top) = root.left.take() {
        let (new_root, node) = take_smallest(top);
        root.left = new_root;
        (Some(root), node)
    } else {
        let new_root = root.right.take();
        (new_root, root)
    }
}

impl<K, V> Node<K, V>
where
    K: std::cmp::PartialOrd,
{
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key: key,
            value: value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn update_height(&mut self) {
        self.height = max(height(&self.left), height(&self.right)) + 1;
    }
}

pub struct Tree<K, V> {
    root: Option<Inner<K, V>>,
}

impl<K, V> Tree<K, V>
where
    K: std::cmp::PartialOrd + Display,
{
    pub fn new() -> Tree<K, V> {
        Tree { root: None }
    }

    pub fn height(&self) -> isize {
        height(&self.root)
    }
    
    pub fn pretty(&self) -> String {
        self.pretty_node(&self.root, String::new(), String::new(), String::new())
    }

    fn pretty_node(
        &self,
        tree: &Option<Inner<K, V>>,
        left: String,
        mid: String,
        right: String,
    ) -> String {
        if let Some(root) = tree {
            let mut result = String::new();
            result.push_str(&self.pretty_node(
                &root.left,
                format!("{}   ", left),
                format!("{}  +", left),
                format!("{}  |", left),
            ));
            if root.left.is_some() {
                result.push_str(&format!("{}  |\n", left));
            }
            result.push_str(&format!(
                "{}--+ key={} height={}\n",
                mid, root.key, root.height
            ));
            if root.right.is_some() {
                result.push_str(&format!("{}  |\n", right));
            }
            result.push_str(&self.pretty_node(
                &root.right,
                format!("{}  |", right),
                format!("{}  +", right),
                format!("{}   ", right),
            ));
            result
        } else {
            String::new()
        }
    }

    /// Insert value into tree under the given key.
    pub fn insert(&mut self, key: K, value: V) {
        let old_root = self.root.take();
        let new_root = self.insert_node(old_root, key, value);
        self.root = Some(new_root);
    }

    /// Insert key and value into a new node, optionally performing rotations.
    ///
    /// The rotations might be done to preserve the balance between
    /// the left and right branch of the tree and the root will be
    /// returned (either the new or old root).
    fn insert_node(&self, root: Option<Inner<K, V>>, key: K, value: V) -> Inner<K, V> {
        if let Some(mut node) = root {
            if key < node.key {
                node.left = Some(self.insert_node(node.left.take(), key, value));
            } else {
                node.right = Some(self.insert_node(node.right.take(), key, value));
            };

            node.update_height();
            rebalance(node)
        } else {
            Box::new(Node::new(key, value))
        }
    }

    /// Delete a record by key from the tree.
    ///
    pub fn delete(&mut self, key: K) -> Result<()> {
        let old_root = self.root.take();
        self.root = self.delete_node(old_root, key)?;
        Ok(())
    }

    fn delete_node(&self, root: Option<Inner<K, V>>, key: K) -> Result<Option<Inner<K, V>>> {
        match root {
            None => Err(Error::NotFound),
            Some(mut node) => {
                if key < node.key {
                    node.left = self.delete_node(node.left.take(), key)?;
                    node.update_height();
                    Ok(Some(rebalance(node)))
                } else if key > node.key {
                    node.right = self.delete_node(node.right.take(), key)?;
                    node.update_height();
                    Ok(Some(rebalance(node)))
                } else {
                    // The root of the tree is the node to delete, so
                    // either the tree is empty efter the node is
                    // deleted, or we need to promote either left or
                    // right tree as the new root of the tree.
                    match (node.left.take(), node.right.take()) {
                        (None, None) => Ok(None),
                        (None, Some(right)) => Ok(Some(right)),
                        (Some(left), None) => Ok(Some(left)),
                        (Some(left), Some(right)) => {
                            let (new_root, node) = take_smallest(right);
                            if let Some(mut inner) = new_root {
                                inner.left = Some(left);
                                inner.right = Some(node);
                                Ok(Some(inner))
                            } else {
                                Ok(None)
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn find(&self, key: K) -> Option<&V> {
        self.find_node(&self.root, key)
    }

    fn find_node<'a>(&'a self, root: &'a Option<Inner<K, V>>, key: K) -> Option<&'a V> {
        root.as_ref().and_then(|node| {
            if key < node.key {
                self.find_node(&node.left, key)
            } else if key > node.key {
                self.find_node(&node.right, key)
            } else {
                Some(&node.value)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_avl<K, V>(root: &Option<Inner<K, V>>) -> Option<isize> {
        if let Some(ref node) = root {
            if let Some(lh) = is_avl(&node.left) {
                if let Some(rh) = is_avl(&node.right) {
                    if (lh - rh).abs() <= 1 {
                        return Some(max(lh, rh) + 1);
                    }
                }
            }
            None
        } else {
            Some(0)
        }
    }

    fn is_avl_tree<K, V>(tree: &Tree<K, V>) -> bool {
        if let Some(_) = is_avl(&tree.root) {
            true
        } else {
            false
        }
    }

    #[test]
    fn test_is_avl_tree() {
        let tree = Tree {
            root: Some(Box::new(Node {
                key: 1,
                value: 1,
                height: 3,
                left: Some(Box::new(Node {
                    key: 2,
                    value: 2,
                    height: 2,
                    left: Some(Box::new(Node {
                        key: 3,
                        value: 3,
                        height: 1,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
        };
        assert!(!is_avl_tree(&tree));
    }

    #[test]
    fn test_insert() {
        let mut tree = Tree::new();
        for i in 1..11 {
            tree.insert(i, i * i);
            assert!(is_avl_tree(&tree));
            tree.insert(2 * 11 - i, i * i * i);
            assert!(is_avl_tree(&tree));
        }

        for i in 1..11 {
            assert_eq!(tree.find(i), Some(&(i * i)));
            assert_eq!(tree.find(2 * 11 - i), Some(&(i * i * i)));
        }
    }

    #[test]
    fn test_delete() {
        let mut tree = Tree::new();
        for i in 1..11 {
            tree.insert(i, i * i);
            tree.insert(2 * 11 - i, i * i * i);
        }

        for i in 1..11 {
            assert_eq!(tree.find(i), Some(&(i * i)));
            assert_eq!(tree.find(2 * 11 - i), Some(&(i * i * i)));

            assert_eq!(tree.delete(i), Ok(()));
            assert!(
                is_avl_tree(&tree),
                format!("Tree was not AVL:\n{}", tree.pretty())
            );
            assert_eq!(tree.find(i), None);
            assert_eq!(tree.find(2 * 11 - i), Some(&(i * i * i)));

            assert_eq!(tree.delete(2 * 11 - i), Ok(()));
            assert!(
                is_avl_tree(&tree),
                format!("Tree was not AVL:\n{}", tree.pretty())
            );
            assert_eq!(tree.find(i), None);
            assert_eq!(tree.find(2 * 11 - i), None);
        }
    }

    #[test]
    fn test_rotate_right() {
        let root = Box::new(Node {
            key: 1,
            value: 1,
            height: 3,
            left: Some(Box::new(Node {
                key: 2,
                value: 2,
                height: 2,
                left: Some(Box::new(Node {
                    key: 3,
                    value: 3,
                    height: 1,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: None,
        });

        let after = rotate_right(root);
        assert_eq!(after.key, 2);
        assert_eq!(after.height, 2);
    }
}
