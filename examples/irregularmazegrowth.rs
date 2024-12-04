use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = one![W:*BB / WBB / *BB > *** / WW* / ***];
    visualize::runner(&mut rules);
}