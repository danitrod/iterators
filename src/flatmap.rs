pub struct FlatMap<T, I, F>
where
    T: IntoIterator,
    I: Iterator,
    F: FnMut(T::Item) -> I,
{
    outer: T::IntoIter,
    inner: Option<I>,
    mapper: F,
}

impl<T, I, F> Iterator for FlatMap<T, I, F>
where
    T: IntoIterator,
    I: Iterator,
    F: FnMut(T::Item) -> I,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = &mut self.inner {
                if let Some(next) = inner.next() {
                    return Some(next);
                }
            }
            if let Some(outer) = self.outer.next() {
                self.inner = Some((self.mapper)(outer));
            } else {
                return None;
            }
        }
    }
}

pub fn flat_map<T, I, F>(i: T, f: F) -> FlatMap<T, I, F>
where
    T: IntoIterator,
    I: Iterator,
    F: FnMut(T::Item) -> I,
{
    FlatMap {
        outer: i.into_iter(),
        inner: None,
        mapper: f,
    }
}

#[test]
fn collect_chars() {
    assert_eq!(
        flat_map(vec!["hello", " ", "world"], |s| s.chars()).collect::<String>(),
        "hello world"
    );
}

#[test]
fn next() {
    let mut iter = flat_map(vec![vec![1, 2, 3], vec![1, 2, 3]], |v| v.into_iter());

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
}
