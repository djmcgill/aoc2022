
use d3::{p1_btreeset, p2_btreeset, REAL};
    
fn main() {
    let p1: u32 = p1_btreeset(REAL);

    let p2: u32 = p2_btreeset(REAL);

    println!("{} {}", p1, p2);
}
