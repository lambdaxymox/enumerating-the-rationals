pub enum Tree<T> {
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

pub fn foldt<F, T>(f: F, tree: &Tree<T>) -> T where F: for<'r, 's> Fn<(T, &'r Tree<T>, &'s Tree<T>)> -> T {
    match tree {
        &Tree::Node(val, left, right) => {
            f(val, foldt(f, &*left), foldt(f, right))
        }
    }
}