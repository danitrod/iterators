pub struct Cycle<T>
where
    T: IntoIterator + Clone,
{
    copy: T,
    iter: T::IntoIter,
}

impl<T> Iterator for Cycle<T>
where
    T: IntoIterator + Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            return Some(next);
        }
        self.iter = self.copy.clone().into_iter();
        // Now we can safely return the next, if it's None the inner iterator is empty.
        self.iter.next()
    }
}

pub fn cycle<T>(iterable: T) -> Cycle<T>
where
    T: IntoIterator + Clone,
{
    Cycle {
        copy: iterable.clone(),
        iter: iterable.into_iter(),
    }
}

#[test]
fn cycle_infinitely() {
    let mut iter = cycle(vec![1, 2, 3]);

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(1));
}

#[test]
fn does_not_loop_infinitely_on_empty() {
    let mut iter = cycle(std::iter::empty::<usize>());

    assert_eq!(iter.next(), None);
}

#[test]
fn std_cycle_does_not_loop_infinitely() {
    let iterable = std::iter::empty::<usize>();

    assert_eq!(iterable.cycle().next(), None);
}
