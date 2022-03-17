use std::{borrow::Borrow, cmp::Ordering, mem};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreapNode<K, V> {
    pub key: K,
    pub value: V,
    priority: usize,
    length: usize,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

impl<K, V> TreapNode<K, V> {
    pub fn new(key: K, value: V) -> Option<Box<Self>> {
        Some(Box::new(Self {
            key,
            value,
            priority: random(),
            length: 1,
            left: None,
            right: None,
        }))
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        match key.cmp(self.key.borrow()) {
            Ordering::Equal => Some(&self.value),
            Ordering::Greater => self.right.as_ref().and_then(|node| node.get(key)),
            Ordering::Less => self.left.as_ref().and_then(|node| node.get(key)),
        }
    }

    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        match key.cmp(self.key.borrow()) {
            Ordering::Equal => Some((&self.key, &self.value)),
            Ordering::Greater => self.right.as_ref().and_then(|node| node.get_key_value(key)),
            Ordering::Less => self.left.as_ref().and_then(|node| node.get_key_value(key)),
        }
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        match key.cmp(self.key.borrow()) {
            Ordering::Equal => Some(&mut self.value),
            Ordering::Greater => self.right.as_mut().and_then(|node| node.get_mut(key)),
            Ordering::Less => self.left.as_mut().and_then(|node| node.get_mut(key)),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        match key.cmp(&self.key) {
            Ordering::Equal => Some(mem::replace(&mut self.value, value)),
            Ordering::Greater => match &mut self.right {
                Some(node) => node.insert(key, value).or_else(|| {
                    self.length += 1;
                    None
                }),
                None => {
                    self.right = Self::new(key, value);
                    self.length += 1;
                    if self.priority < self.right.as_ref()?.priority {
                        self.left_rotate();
                    }
                    None
                }
            },
            Ordering::Less => match &mut self.left {
                Some(node) => node.insert(key, value).or_else(|| {
                    self.length += 1;
                    None
                }),
                None => {
                    self.left = Self::new(key, value);
                    self.length += 1;
                    if self.priority < self.left.as_ref()?.priority {
                        self.right_rotate();
                    }
                    None
                }
            },
        }
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Result<Option<V>, ()>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        match key.cmp(self.key.borrow()) {
            Ordering::Equal => unsafe {
                let left = &mut self.left as *mut Option<Box<Self>>;
                let right = &mut self.right as *mut Option<Box<Self>>;
                match (&*left, &*right) {
                    (Some(left), Some(right)) => {
                        if left.priority < right.priority {
                            self.left_rotate();
                            self.left.as_mut().unwrap().remove(key)
                        } else {
                            self.right_rotate();
                            self.right.as_mut().unwrap().remove(key)
                        }
                    }
                    (Some(_), None) => Ok(Some({
                        let node = mem::replace(&mut self.left, None).unwrap();
                        mem::replace(self, *node).value
                    })),
                    (None, Some(_)) => Ok(Some({
                        let node = mem::replace(&mut self.right, None).unwrap();
                        mem::replace(self, *node).value
                    })),
                    (None, None) => Err(()),
                }
            },
            Ordering::Greater => Ok(match self.right.as_mut() {
                Some(node) => node.remove(key).unwrap_or_else(|_| {
                    let node = mem::replace(&mut self.right, None).unwrap();
                    Some(node.value)
                }),
                None => None,
            }),
            Ordering::Less => Ok(match self.left.as_mut() {
                Some(node) => node.remove(key).unwrap_or_else(|_| {
                    let node = mem::replace(&mut self.left, None).unwrap();
                    Some(node.value)
                }),
                None => None,
            }),
        }
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Result<Option<(K, V)>, ()>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        match key.cmp(self.key.borrow()) {
            Ordering::Equal => unsafe {
                let left = &mut self.left as *mut Option<Box<Self>>;
                let right = &mut self.right as *mut Option<Box<Self>>;
                match (&*left, &*right) {
                    (Some(left), Some(right)) => {
                        if left.priority < right.priority {
                            self.left_rotate();
                            self.left.as_mut().unwrap().remove_entry(key)
                        } else {
                            self.right_rotate();
                            self.right.as_mut().unwrap().remove_entry(key)
                        }
                    }
                    (Some(_), None) => Ok(Some({
                        let node = mem::replace(&mut self.left, None).unwrap();
                        let node = mem::replace(self, *node);
                        (node.key, node.value)
                    })),
                    (None, Some(_)) => Ok(Some({
                        let node = mem::replace(&mut self.right, None).unwrap();
                        let node = mem::replace(self, *node);
                        (node.key, node.value)
                    })),
                    (None, None) => Err(()),
                }
            },
            Ordering::Greater => Ok(match self.right.as_mut() {
                Some(node) => node.remove_entry(key).unwrap_or_else(|_| {
                    let node = mem::replace(&mut self.right, None).unwrap();
                    Some((node.key, node.value))
                }),
                None => None,
            }),
            Ordering::Less => Ok(match self.left.as_mut() {
                Some(node) => node.remove_entry(key).unwrap_or_else(|_| {
                    let node = mem::replace(&mut self.left, None).unwrap();
                    Some((node.key, node.value))
                }),
                None => None,
            }),
        }
    }

    pub const fn len(&self) -> usize {
        self.length
    }

    fn left_rotate(&mut self) {
        if let Some(mut node) = mem::replace(&mut self.right, None) {
            self.length -= node.length;
            mem::swap(self, &mut node);
            if let Some(left) = &self.left {
                self.length -= left.length;
                node.length += left.length;
            }
            mem::swap(&mut self.left, &mut node.right);
            self.length += node.length;
            self.left = Some(node);
        }
    }

    fn right_rotate(&mut self) {
        if let Some(mut node) = mem::replace(&mut self.left, None) {
            self.length -= node.length;
            mem::swap(self, &mut node);
            if let Some(right) = &self.right {
                self.length -= right.length;
                node.length += right.length;
            }
            mem::swap(&mut self.right, &mut node.left);
            self.length += node.length;
            self.right = Some(node);
        }
    }
}

#[rustfmt::skip]
fn random<T: Copy>() -> T {
    fn ptr<T>(r: &T) -> *const T { r }
    extern "C" { fn rand() -> usize; }
    unsafe { *ptr(&rand()).cast() }
}
