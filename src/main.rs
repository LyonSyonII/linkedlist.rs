use std::{alloc::{Layout, alloc, dealloc}, ptr::NonNull, marker::PhantomData};

type NodePtr<T> = Option<NonNull<Node<T>>>;

#[derive(Clone)]
struct LinkedList<T> {
    head: NodePtr<T>,
    current: NodePtr<T>,
    tail: NodePtr<T>,
    len: usize
}

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: NodePtr<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, current: None, tail: None, len: 0 }
    }

    pub fn next(&mut self) -> Option<&T> {
        if let Some(p) = self.current {
            let next = unsafe { p.as_ref().next };
            self.current = next;
            next.map(|p| unsafe { &p.as_ref().value })
        } else {
            self.current = self.head;
            self.current.map(|p| unsafe { &p.as_ref().value })
        }
    }

    pub fn next_mut(&mut self) -> Option<&mut T> {
        if let Some(p) = self.current {
            let next = unsafe { p.as_ref().next };
            self.current = next;
            return next.map(|mut p| unsafe { &mut p.as_mut().value });
        }
        
        None
    }

    fn next_ptr(&mut self) -> NodePtr<T> {
        let current = self.current;
        self.current = current.and_then(|p| unsafe { p.as_ref().next() });
        current
    }
    
    #[allow(clippy::wrong_self_convention)]
    pub fn to_start(&mut self) -> &mut Self {
        self.current = None;
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_end(&mut self) -> &mut Self {
        self.current = self.tail;
        self
    }

    pub fn push(&mut self, value: T) {
        if let Some(end) = &mut self.tail {
            let new = Some(Node::new(value));
            unsafe { end.as_mut().next = new; }
            self.tail = new;
        } else {
            let new = Some(Node::new(value));
            self.head = new;
            self.current = new;
            self.tail = new;
        }
        self.len += 1;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(v) = self.next_ptr() {
            unsafe { dealloc(v.as_ptr().cast(), Layout::for_value(v.as_ref())) }
        }
    }
}

impl<T> Node<T> {
    fn new(value: T) -> NonNull<Node<T>> {
        let node = Node { value, next: None };
        let alloc = unsafe { alloc(Layout::for_value(&node)) } as *mut Node<T>;
        unsafe { *alloc = node; }
        NonNull::new(alloc).unwrap()
    }

    fn next(&self) -> NodePtr<T> {
        self.next
    }
}

impl<T: std::fmt::Debug + Clone> std::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut clone = self.clone();
        clone.to_start();

        if let Some(v) = clone.next() {
            write!(f, "{v:?}")?;
        }
        
        while let Some(v) = clone.next() {
            write!(f, ", {v:?}")?;
        }

        write!(f, "]")?;

        Ok(())
    }
}



fn main() {
    let mut list = LinkedList::new();
    list.push(5);
    list.push(75);
    list.push(93);
    println!("{list:?}");
}
