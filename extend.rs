use std::collections::HashSet;

fn main() {
    let mut set1: HashSet<i32> = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set1.insert(3);  //duplicate, will not be inserted
    set1.insert(4);
    println!("{:?}", set1);  //{1, 2, 3, 4}

    let set2: HashSet<i32> = HashSet::from([3, 4, 5, 3]);
    println!("{:?}", set2); //{3, 4, 5}

    // Using extend() to add all elements of set2 into set1.
    // extend() ensures uniqueness, so it doesn't insert duplicates.
    set1.extend(set2);

    println!("{:?}", set1); //{1, 2, 3, 4, 5}
}
