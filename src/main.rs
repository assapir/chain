mod chain;
mod node;

use crate::chain::Chain;

fn main() {
    let mut chain = Chain::new();
    chain.commit(1);
    chain.commit(2);
    println!("{:#?}", chain.get_head());
    println!("{:#?}", chain.get_older(1));
    println!("{:#?}", chain.get_older(3));
}
