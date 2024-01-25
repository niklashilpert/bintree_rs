mod tree;

use tree::BinaryTree;

fn main() {
    let mut tree: BinaryTree<i32> = BinaryTree::new();

    tree.print_pre_order();

    tree.insert(5);
    tree.insert(3);
    tree.insert(2);
    tree.insert(4);
    tree.insert(1);

    tree.insert(8);
    tree.insert(7);
    tree.insert(6);
    tree.insert(9);
    tree.print_pre_order();
    tree.print_in_order();
    tree.print_post_order();
}
