pub fn vectors() {
    // Iterators can be collected into vectors
    let mut collected_iterator: Vec<int> = range(0i, 10).collect();
    println!("Collected range(0, 10) into: {}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i, 2, 3];
    println!("Initial vector: {}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector")
    xs.push(4);
    println!("Vector: {}", xs);

    // Error! Immutable vectors can't grow -- fixed by declaring collected_iterator mut
    collected_iterator.push(0);
    println!("collected_iterator size: {}", collected_iterator.len());

    // The `len` method yields the current size of the vector
    println!("Vector size: {}", xs.len());

    // Indexing is done using the `get` method (indexing starts at 0)
    println!("Second element: {}", collected_iterator.get(1));

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {}", xs.pop());

    // Out of bounds indexing yields a task failure
    println!("Fourth element: {}", xs.get(3));
}

pub trait Trait {
}