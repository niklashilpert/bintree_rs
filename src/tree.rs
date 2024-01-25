use std::fmt::Display;


enum Element<T: PartialEq + PartialOrd + Display> {
    Node(T, Box<Element<T>>, Box<Element<T>>),
    End,
}

pub struct BinaryTree<T: PartialEq + PartialOrd + Display> {
    start: Element<T>,
}



impl<T: PartialEq + PartialOrd + Display> BinaryTree<T> {
    pub fn new() -> Self { 
        return BinaryTree { start: Element::End } 
    }
}

impl<T: PartialEq + PartialOrd + Display> Element<T> {
    fn from(t: T) -> Self { 
        Element::Node(t, Box::from(Element::End), Box::from(Element::End)) 
    }
}


impl<T: PartialEq + PartialOrd + Display> BinaryTree<T> {
    pub fn contains(&self, t: T) -> bool {
        return self.start.contains(t);
    }

    pub fn insert(&mut self, t: T) {
        if let Element::Node(..) = self.start {
            self.start.insert(t);
        } else {
            self.start = Element::from(t);
        }
    }
}

impl<T: PartialEq + PartialOrd + Display> Element<T> {

    fn contains(&self, t: T) -> bool {
        match self {
            Element::Node(data, left, right) => {
                return if *data == t {
                    true
                } else if t < *data {
                    left.contains(t)
                } else {
                    right.contains(t)
                }
            },
            Element::End => false
        }        
    }

    fn insert(&mut self, t: T) {
        match self {
            Element::Node(data, left, right) => {
                if t < *data {
                    if let Element::Node(..) = **left {
                        left.insert(t);
                    } else {
                        *left = Box::from(Element::from(t));
                    }
                } else if t > *data {
                    if let Element::Node(..) = **right {
                        right.insert(t);
                    } else {
                        *right = Box::from(Element::from(t));
                    }
                }
            },
            _ => {println!("This should never be printed!");}
        }
    }

}


/*
 *  The following implementations definde printing functions
 */

impl<T: PartialEq + PartialOrd + Display> BinaryTree<T> {
    pub fn print_pre_order(&self) {
        println!("Tree in PreOrder:");
        self.start.print_pre_order(0);
    }

    pub fn print_in_order(&self) {
        println!("Tree in Order:");
        self.start.print_in_order();
    }

    pub fn print_post_order(&self) {
        println!("Tree in PostOrder:");
        self.start.print_post_order(0);
    }

}
impl<T: PartialEq + PartialOrd + Display> Element<T> {
    fn print_pre_order(&self, depth: u32) {
        match self {
            Element::Node(data, left, right) => {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("{}", data);
                left.print_pre_order(depth+1);
                right.print_pre_order(depth+1);
            } 
            _ => {}
        }
    }

    fn print_in_order(&self) {
        match self {
            Element::Node(data, left, right) => {
                
                left.print_in_order();
                println!("{}", data);
                right.print_in_order();
            } 
            _ => {}
        }
    }

    fn print_post_order(&self, depth: u32) {
        match self {
            Element::Node(data, left, right) => {
                left.print_post_order(depth+1);
                right.print_post_order(depth+1);
                for _ in 0..depth {
                    print!("  ");
                }
                println!("{}", data);
            } 
            _ => {}
        }
    }
}