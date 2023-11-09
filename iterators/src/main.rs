use std::iter;

#[derive(Clone, Debug)]
struct Tree {
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
    value: i32,
}

impl Tree {
    fn leaf(value: i32) -> Self {
        Tree {
            left: None,
            right: None,
            value,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

// fn tree_iter_existential(tree: Tree) -> impl Iterator<Item=i32> {
//     let is_leaf = tree.is_leaf();
//     let left_iter = tree.left.into_iter().flat_map(|b| tree_iter_existential(*b));
//     let right_iter = tree.right.into_iter().flat_map(|b| tree_iter_existential(*b));
//
//    left_iter
//         .chain((if is_leaf { Some(tree.value) } else {None}).into_iter())
//         .chain(right_iter)
// }

fn tree_iter_dyn(tree: Tree) -> Box<dyn Iterator<Item=i32>> {
    let is_leaf = tree.is_leaf();
    let node_iter = |node: Option<Box<Tree>>|
        Box::new(node.into_iter().flat_map(|b| tree_iter_dyn(*b)))
        ;

    Box::new(
        node_iter(tree.left)
            .chain((if is_leaf { Some(tree.value) } else { None }).into_iter())
            .chain(node_iter(tree.right))
    )
}

fn internal_iterate<F: Fn(i32)>(tree: Tree, f: &F) {
    let is_leaf = tree.is_leaf();
    tree.left.map(|r| internal_iterate(*r, f));

    if is_leaf {
        f(tree.value);
    }

    tree.right.map(|r| internal_iterate(*r, f));
}

fn test_tree_iterator() {
    let tree = Tree {
        left: Some(Box::new(Tree::leaf(10))),
        right: Some(Box::new(Tree::leaf(11))),
        value: 0,
    };

    for i in tree_iter_dyn(tree.clone()) {
        println!("{i}");
    }


    internal_iterate(tree, &|int| { println!("{int}"); });
}





struct Filter<T, I: Iterator<Item=T>, F: Fn(&T) -> bool> {
    inner: I,
    predicate: F,
}

impl<T, I: Iterator<Item=T>, F: Fn(&T) -> bool> Iterator for Filter<T, I, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(inner_next) = self.inner.next() {
            if (self.predicate)(&inner_next) {
                return Some(inner_next);
            }
        }
        None
    }
}

trait Filterable: Iterator {
    fn my_filter<F: Fn(&Self::Item) -> bool>(self, f: F) -> Filter<Self::Item, Self, F> where Self: Sized;
}


impl<T, I: Iterator<Item=T>> Filterable for I {
    fn my_filter<F: Fn(&Self::Item) -> bool>(self, f: F) -> Filter<Self::Item, Self, F> {
        Filter {
            inner: self,
            predicate: f,
        }
    }
}

fn main() {
    for i in [1, 2, 3].into_iter().my_filter(|e| e % 2 == 1) {
        println!("{i}");
    }
}
