use id_tree::InsertBehavior::*;
use id_tree::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Pair {
    key: String,
    val: u32,
}

fn sort_value_occurence(occurence: &mut Vec<Pair>) {
    occurence.sort_by(|a, b| a.val.cmp(&b.val));
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

fn build_up_tree<'a>(tree: &'a mut Tree<Pair>, occurence: &'a mut Vec<Pair>) -> &'a Tree<Pair> {
    if occurence.len() == 1 {
        return tree;
    }
    let pair: Vec<Pair> = occurence.drain(0..2).collect();
    let new_pair = Pair {
        key: pair[0].key.clone() + &pair[1].key,
        val: pair[0].val + pair[1].val,
    };
    let root_id = tree.root_node_id().unwrap();
    let new_node = Node::new(new_pair.clone());
    let node = tree_find(tree, &new_pair.key);
    match node {
        Some(node) => {
            let new_id = tree.insert(new_node, UnderNode(&root_id)).unwrap();
            tree.swap_nodes(&root_id, &new_id, SwapBehavior::LeaveChildren)
                .unwrap();
        }
        None => {}
    };
    tree.insert(
        Node::new(Pair {
            key: pair[0].key.clone(),
            val: pair[0].val,
        }),
        UnderNode(&root_id),
    )
    .unwrap();
    tree.insert(
        Node::new(Pair {
            key: pair[1].key.clone(),
            val: pair[1].val,
        }),
        UnderNode(&root_id),
    )
    .unwrap();
    occurence.push(new_pair);
    sort_value_occurence(occurence);
    build_up_tree(tree, occurence)
}

fn main() {
    let string_test = "A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    let mut occurence: Vec<Pair> = Vec::new();
    for it in string_test.chars() {
        match occurence.iter_mut().find(|x| x.key == it.to_string()) {
            Some(x) => {
                x.val += 1;
            }
            None => {
                occurence.push(Pair {
                    key: it.to_string(),
                    val: 1,
                });
            }
        }
    }
    sort_value_occurence(&mut occurence);
    let mut tree: Tree<Pair> = TreeBuilder::new().with_node_capacity(5).build();
    build_up_tree(&mut tree, &mut occurence);
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
