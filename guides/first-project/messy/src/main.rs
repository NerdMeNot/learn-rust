use std::collections::HashMap;
fn main(){
let mut stock=HashMap::new();
    stock.insert("apple",3);stock.insert("pear",5);
  let total:i32=stock.values().sum();
    if total>7{println!("{total} items, no restock")}else{
println!("restock");
    }
}
