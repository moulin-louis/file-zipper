struct Node {
    value: i32,
    left: Subtree,
    right: Subtree,
}

struct Subtree(Option<Box<Node>>);

pub struct BinaryTree {
    root: Subtree,
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    key: char,
    val: u32,
}

fn main() {
    let string_test = "A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    let mut occurence: Vec<Pair> = Vec::new();
    for it in string_test.chars() {
        match occurence.iter_mut().find(|x| x.key == it) {
            Some(x) => {
                x.val += 1;
            }
            None => {
                occurence.push(Pair { key: it, val: 1 });
            }
        }
    }
    occurence.sort_by(|a, b| a.val.cmp(&b.val));
    for chunk in occurence.chunks(2) {
        if chunk.len() == 2 {
            let (first, second) = (&chunk[0], &chunk[1]);
            println!(
                "Pair: ({}, {}) = ({}, {})",
                first.key, second.key, first.val, second.val
            );
        } else {
            // Handle the case when there's an odd number of elements
            println!("Remaining unpaired element: {:?}", chunk[0]);
        }
    }
}
