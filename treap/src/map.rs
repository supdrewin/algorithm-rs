use crate::node::TreapNode;
use std::{
    borrow::Borrow,
    collections::VecDeque,
    mem,
    ops::{Bound, Index, RangeBounds},
};

/// An ordered map based on a [Treap].
///
/// [Treap]: https://en.wikipedia.org/wiki/Treap
///
/// # Examples
///
/// ```
/// use treap::TreapMap;
///
/// // type inference lets us omit an explicit type signature (which
/// // would be `BTreeMap<&str, &str>` in this example).
/// let mut movie_reviews = TreapMap::new();
///
/// // review some movies.
/// movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
/// movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
/// movie_reviews.insert("The Godfather",      "Very enjoyable.");
/// movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");
///
/// // check for a specific one.
/// if !movie_reviews.contains_key("Les Misérables") {
///     println!("We've got {} reviews, but Les Misérables ain't one.",
///              movie_reviews.len());
/// }
///
/// // oops, this review has a lot of spelling mistakes, let's delete it.
/// movie_reviews.remove("The Blues Brothers");
///
/// // look up the values associated with some keys.
/// let to_find = ["Up!", "Office Space"];
/// for movie in &to_find {
///     match movie_reviews.get(movie) {
///        Some(review) => println!("{movie}: {review}"),
///        None => println!("{movie} is unreviewed.")
///     }
/// }
///
/// // Look up the value for a key (will panic if the key is not found).
/// println!("Movie review: {}", movie_reviews["Office Space"]);
///
/// // iterate over everything.
/// for (movie, review) in &movie_reviews {
///     println!("{movie}: \"{review}\"");
/// }
/// ```
///
/// A `TreapMap` with a known list of items can be initialized from an array:
///
/// ```
/// use treap::TreapMap;
///
/// let solar_distance = TreapMap::from([
///     ("Mercury", 0.4),
///     ("Venus", 0.7),
///     ("Earth", 1.0),
///     ("Mars", 1.5),
/// ]);
/// ```
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreapMap<K, V> {
    root: Option<Box<TreapNode<K, V>>>,
}

impl<K, V> TreapMap<K, V> {
    /// Makes a new, empty `TreapMap`.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    ///
    /// // entries can now be inserted into the empty map
    /// map.insert(1, "a");
    /// ```
    pub const fn new() -> Self {
        Self { root: None }
    }

    /// Clears the map, removing all elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut a = TreapMap::new();
    /// a.insert(1, "a");
    /// a.clear();
    /// assert!(a.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        Some(&self.root.as_ref()?.get(key)?.value)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        let node = self.root.as_ref()?.get(key)?;
        Some((&node.key, &node.value))
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.get(key).is_some()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        Some(&mut self.root.as_mut()?.get_mut(key)?.value)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the
    /// old value is returned. The key is not updated, though; this matters
    /// for types that can be `==` without being identical.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        match &mut self.root {
            Some(node) => node.insert(key, value),
            None => {
                self.root = TreapNode::new(key, value);
                None
            }
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        Some(
            match self.root.as_mut()?.remove(key) {
                Err(()) => *mem::replace(&mut self.root, None)?,
                Ok(node) => node?,
            }
            .value,
        )
    }

    /// Removes a key from the map, returning the stored key and value if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove_entry(&1), Some((1, "a")));
    /// assert_eq!(map.remove_entry(&1), None);
    /// ```
    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        let node = match self.root.as_mut()?.remove(key) {
            Err(()) => *mem::replace(&mut self.root, None)?,
            Ok(node) => node?,
        };
        Some((node.key, node.value))
    }

    /// Constructs a double-ended iterator over a sub-range of
    /// elements in the map.
    /// The simplest way is to use the range syntax `min..max`, thus
    /// `range(min..max)` will yield elements from min (inclusive) to
    /// max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so
    /// for example `range((Excluded(4), Included(10)))` will yield
    /// a left-exclusive, right-inclusive range from 4 to 10.
    ///
    /// # Panics
    ///
    /// Panics if range `start > end`.
    /// Panics if range `start == end` and both bounds are `Excluded`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::ops::Bound::Included;
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(3, "a");
    /// map.insert(5, "b");
    /// map.insert(8, "c");
    /// for (&key, &value) in map.range((Included(&4), Included(&8))) {
    ///     println!("{key}: {value}");
    /// }
    /// assert_eq!(Some((&5, &"b")), map.range(4..).next());
    /// ```
    pub fn range<T: ?Sized, R>(&self, range: R) -> Range<K, V>
    where
        T: Ord,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        Range::new(self.root.as_ref(), range)
    }

    /// Constructs a mutable double-ended iterator over a sub-range of
    /// elements in the map.
    /// The simplest way is to use the range syntax `min..max`, thus
    /// `range(min..max)` will yield elements from min (inclusive) to
    /// max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so
    /// for example `range((Excluded(4), Included(10)))` will yield
    /// a left-exclusive, right-inclusive range from 4 to 10.
    ///
    /// # Panics
    ///
    /// Panics if range `start > end`.
    /// Panics if range `start == end` and both bounds are `Excluded`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map: TreapMap<&str, i32> =
    ///     [("Alice", 0), ("Bob", 0), ("Carol", 0), ("Cheryl", 0)].into();
    /// for (_, balance) in map.range_mut("B".."Cheryl") {
    ///     *balance += 100;
    /// }
    /// for (name, balance) in &map {
    ///     println!("{name} => {balance}");
    /// }
    /// ```
    pub fn range_mut<T: ?Sized, R>(&mut self, range: R) -> RangeMut<K, V>
    where
        T: Ord,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        RangeMut::new(self.root.as_mut(), range)
    }

    /// Gets an iterator over the entries of the map, sorted by key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::new();
    /// map.insert(3, "c");
    /// map.insert(2, "b");
    /// map.insert(1, "a");
    ///
    /// for (key, value) in map.iter() {
    ///     println!("{key}: {value}");
    /// }
    ///
    /// let (first_key, first_value) = map.iter().next().unwrap();
    /// assert_eq!((*first_key, *first_value), (1, "a"));
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter::new(self.root.as_ref())
    }

    /// Gets a mutable iterator over the entries of the map, sorted by key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut map = TreapMap::from([
    ///    ("a", 1),
    ///    ("b", 2),
    ///    ("c", 3),
    /// ]);
    ///
    /// // add 10 to the value if the key isn't "a"
    /// for (key, value) in map.iter_mut() {
    ///     if key != &"a" {
    ///         *value += 10;
    ///     }
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut::new(self.root.as_mut())
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut a = TreapMap::new();
    /// assert_eq!(a.len(), 0);
    /// a.insert(1, "a");
    /// assert_eq!(a.len(), 1);
    /// ```
    pub const fn len(&self) -> usize {
        match &self.root {
            Some(node) => node.len(),
            None => 0,
        }
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let mut a = TreapMap::new();
    /// assert!(a.is_empty());
    /// a.insert(1, "a");
    /// assert!(!a.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, K, V> IntoIterator for &'a TreapMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self.root.as_ref())
    }
}

impl<'a, K, V> IntoIterator for &'a mut TreapMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self.root.as_mut())
    }
}

impl<K, V> IntoIterator for TreapMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.root)
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for TreapMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut this = TreapMap::new();
        iter.into_iter().for_each(|(key, val)| {
            this.insert(key, val);
        });
        this
    }
}

impl<K: Ord, V> Extend<(K, V)> for TreapMap<K, V> {
    #[inline]
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |(key, val)| {
            self.insert(key, val);
        });
    }

    #[inline]
    fn extend_one(&mut self, (k, v): (K, V)) {
        self.insert(k, v);
    }
}

impl<'a, K: Ord + Copy, V: Copy> Extend<(&'a K, &'a V)> for TreapMap<K, V> {
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &val)| (key, val)));
    }

    #[inline]
    fn extend_one(&mut self, (&k, &v): (&'a K, &'a V)) {
        self.insert(k, v);
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for TreapMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `TreapMap`.
    #[inline]
    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl<K: Ord, V, const N: usize> From<[(K, V); N]> for TreapMap<K, V> {
    /// Converts a `[(K, V); N]` into a `TreapMap<(K, V)>`.
    ///
    /// ```
    /// use treap::TreapMap;
    ///
    /// let map1 = TreapMap::from([(1, 2), (3, 4)]);
    /// let map2: TreapMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_ne!(map1, map2);
    /// ```
    fn from(items: [(K, V); N]) -> Self {
        let mut this = TreapMap::new();
        items.into_iter().for_each(|(key, val)| {
            this.insert(key, val);
        });
        this
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<'a, K, V> {
    inner: VecDeque<&'a Box<TreapNode<K, V>>>,
}

impl<'a, K: Ord, V> Range<'a, K, V> {
    pub fn new<T: ?Sized, R>(root: Option<&'a Box<TreapNode<K, V>>>, range: R) -> Self
    where
        T: Ord,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        let mut stack = VecDeque::new();
        let mut inner = VecDeque::new();
        let mut current = root;
        while match current {
            Some(node) => {
                match range.start_bound() {
                    Bound::Excluded(start) if node.key.borrow().le(start) => {
                        current = node.right.as_ref()
                    }
                    Bound::Included(start) if node.key.borrow().lt(start) => {
                        current = node.right.as_ref()
                    }
                    _ => {
                        current = node.left.as_ref();
                        stack.push_back(node);
                    }
                }
                true
            }
            None => match stack.pop_back() {
                Some(node) => {
                    match range.end_bound() {
                        Bound::Excluded(end) if node.key.borrow().ge(end) => current = None,
                        Bound::Included(end) if node.key.borrow().gt(end) => current = None,
                        _ => {
                            current = node.right.as_ref();
                            inner.push_back(node);
                        }
                    }
                    true
                }
                None => false,
            },
        } {}
        Self { inner }
    }
}

impl<'a, K, V> Iterator for Range<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_front()?;
        Some((&node.key, &node.value))
    }
}

impl<K, V> DoubleEndedIterator for Range<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_back()?;
        Some((&node.key, &node.value))
    }
}

impl<K, V> ExactSizeIterator for Range<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangeMut<'a, K, V> {
    inner: VecDeque<&'a mut Box<TreapNode<K, V>>>,
}

impl<'a, K: Ord, V> RangeMut<'a, K, V> {
    pub fn new<T: ?Sized, R>(root: Option<&'a mut Box<TreapNode<K, V>>>, range: R) -> Self
    where
        T: Ord,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        let mut stack = VecDeque::new();
        let mut inner = VecDeque::new();
        let mut current = root.and_then(|node| Some(node as *mut Box<TreapNode<K, V>>));
        unsafe {
            while match current {
                Some(node) => {
                    match range.start_bound() {
                        Bound::Excluded(start) if (*node).key.borrow().le(start) => {
                            current = (*node).right.as_mut().and_then(|node| Some(node as *mut _));
                        }
                        Bound::Included(start) if (*node).key.borrow().lt(start) => {
                            current = (*node).right.as_mut().and_then(|node| Some(node as *mut _));
                        }
                        _ => {
                            current = (*node).left.as_mut().and_then(|node| Some(node as *mut _));
                            stack.push_back(node);
                        }
                    }
                    true
                }
                None => match stack.pop_back() {
                    Some(node) => {
                        match range.end_bound() {
                            Bound::Excluded(end) if (*node).key.borrow().ge(end) => current = None,
                            Bound::Included(end) if (*node).key.borrow().gt(end) => current = None,
                            _ => {
                                current =
                                    (*node).right.as_mut().and_then(|node| Some(node as *mut _));
                                inner.push_back(node.as_mut().unwrap());
                            }
                        }
                        true
                    }
                    None => false,
                },
            } {}
        }
        Self { inner }
    }
}

impl<'a, K, V> Iterator for RangeMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_front()?;
        Some((&node.key, &mut node.value))
    }
}

impl<K, V> DoubleEndedIterator for RangeMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_back()?;
        Some((&node.key, &mut node.value))
    }
}

impl<K, V> ExactSizeIterator for RangeMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Iter<'a, K, V> {
    inner: VecDeque<&'a Box<TreapNode<K, V>>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub fn new(root: Option<&'a Box<TreapNode<K, V>>>) -> Self {
        let mut stack = VecDeque::new();
        let mut inner = VecDeque::new();
        let mut current = root;
        while match current {
            Some(node) => {
                current = node.left.as_ref();
                stack.push_back(node);
                true
            }
            None => match stack.pop_back() {
                Some(node) => {
                    current = node.right.as_ref();
                    inner.push_back(node);
                    true
                }
                None => false,
            },
        } {}
        Self { inner }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_front()?;
        Some((&node.key, &node.value))
    }
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_back()?;
        Some((&node.key, &node.value))
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IterMut<'a, K, V> {
    inner: VecDeque<&'a mut Box<TreapNode<K, V>>>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    pub fn new(root: Option<&'a mut Box<TreapNode<K, V>>>) -> Self {
        let mut stack = VecDeque::new();
        let mut inner = VecDeque::new();
        let mut current = root.and_then(|node| Some(node as *mut Box<TreapNode<K, V>>));
        unsafe {
            while match current {
                Some(node) => {
                    current = (*node).left.as_mut().and_then(|node| Some(node as *mut _));
                    stack.push_back(node);
                    true
                }
                None => match stack.pop_back() {
                    Some(node) => {
                        current = (*node).right.as_mut().and_then(|node| Some(node as *mut _));
                        inner.push_back(node.as_mut().unwrap());
                        true
                    }
                    None => false,
                },
            } {}
        }
        Self { inner }
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_front()?;
        Some((&node.key, &mut node.value))
    }
}

impl<K, V> DoubleEndedIterator for IterMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_back()?;
        Some((&node.key, &mut node.value))
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntoIter<K, V> {
    inner: VecDeque<Box<TreapNode<K, V>>>,
}

impl<K, V> IntoIter<K, V> {
    pub fn new(root: Option<Box<TreapNode<K, V>>>) -> Self {
        let mut stack = VecDeque::new();
        let mut inner = VecDeque::new();
        let mut current = root;
        while match current {
            Some(mut node) => {
                current = mem::replace(&mut node.left, None);
                stack.push_back(node);
                true
            }
            None => match stack.pop_back() {
                Some(mut node) => {
                    current = mem::replace(&mut node.right, None);
                    inner.push_back(node);
                    true
                }
                None => false,
            },
        } {}
        Self { inner }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_front()?;
        Some((node.key, node.value))
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let node = self.inner.pop_back()?;
        Some((node.key, node.value))
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
