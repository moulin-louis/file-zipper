use id_tree::InsertBehavior::*;
use id_tree::*;
use std::collections::BinaryHeap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair {
    key: String,
    cost: u32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //Call reverse to implement a min-heap instead of an max-heap
        other.cost.cmp(&self.cost).reverse()
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //Call reverse to implement a min-heap instead of an max-heap
        Some(self.cmp(other).reverse())
    }
}

fn tree_find<'a>(tree: &'a Tree<Pair>, key: &String) -> Option<&'a Node<Pair>> {
    for node in tree
        .traverse_pre_order(&tree.root_node_id().unwrap())
        .unwrap()
    {
        if node.data().key == *key {
            return Some(node);
        }
    }
    return None;
}

fn build_up_tree<'a>(
    tree: &'a mut Tree<Pair>,
    occurence: &'a mut BinaryHeap<Pair>,
) -> &'a Tree<Pair> {
    if occurence.len() == 1 {
        return tree;
    }
    let first_val = occurence.pop().unwrap();
    println!("first val = {:?}", first_val);
    let second_val = occurence.pop().unwrap();
    let new_pair = Pair {
        key: first_val.key.clone() + &second_val.key,
        cost: first_val.cost + second_val.cost,
    };
    let root_id = tree.root_node_id().unwrap();
    let new_node = Node::new(new_pair.clone());
    let node = tree_find(tree, &new_pair.key);
    //match node {
    //    Some(node) => {
    //        let new_id = tree.insert(new_node, UnderNode(&root_id)).unwrap();
    //        tree.swap_nodes(&root_id, &new_id, SwapBehavior::LeaveChildren)
    //            .unwrap();
    //    }
    //    None => {}
    //};
    //tree.insert(
    //    Node::new(Pair {
    //        key: pair[0].key.clone(),
    //        val: pair[0].val,
    //    }),
    //    UnderNode(&root_id),
    //)
    //.unwrap();
    //tree.insert(
    //    Node::new(Pair {
    //        key: pair[1].key.clone(),
    //        val: pair[1].val,
    //    }),
    //    UnderNode(&root_id),
    //)
    //.unwrap();
    occurence.push(new_pair);
    //sort_value_occurence(occurence);
    build_up_tree(tree, occurence)
}

fn main() {
    let string_test = "A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    let mut occurence: Vec<Pair> = Vec::new();
    for it in string_test.chars() {
        match occurence.iter_mut().find(|x| x.key == it.to_string()) {
            Some(x) => {
                x.cost += 1;
            }
            None => {
                occurence.push(Pair {
                    key: it.to_string(),
                    cost: 1,
                });
            }
        }
    }
    let mut b_heap: BinaryHeap<Pair> = BinaryHeap::from(occurence);
    let mut tree: Tree<Pair> = TreeBuilder::new().with_node_capacity(b_heap.len()).build();
    //build_up_tree(&mut tree, &mut b_heap);
    let root_id = tree.root_node_id().unwrap();
    println!(
        "root: nbr of children = {}",
        tree.get(root_id).unwrap().children().len()
    );
    print_ascii_tree(&tree, &tree.root_node_id().unwrap(), "", true).unwrap();
}

// Print tree with ASCII art
fn print_ascii_tree<D: Debug>(
    tree: &Tree<D>,
    node_id: &NodeId,
    prefix: &str,
    is_last: bool,
) -> Result<(), NodeIdError> {
    let node = tree.get(node_id)?;
    let children = node.children();

    // Print current node
    println!(
        "{}{}── {:?}",
        prefix,
        if is_last { "└" } else { "├" },
        node.data()
    );

    // Prepare prefix for children
    let child_prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });

    // Print children
    for (i, child_id) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        print_ascii_tree(tree, child_id, &child_prefix, is_last_child)?;
    }

    Ok(())
}
