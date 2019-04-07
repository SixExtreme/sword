mod array;


use std::collections::HashMap;


fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    for (i, x) in nums.iter().enumerate() {
        println!("> {} {}", i, x);
    }
}
