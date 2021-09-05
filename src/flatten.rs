pub struct Flatten<T>
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    outer: T::IntoIter,
    front_inner: Option<<T::Item as IntoIterator>::IntoIter>,
    back_inner: Option<<T::Item as IntoIterator>::IntoIter>,
}

impl<T> Iterator for Flatten<T>
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    type Item = <T::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(front) = &mut self.front_inner {
                if let Some(next) = front.next() {
                    return Some(next);
                }
            }
            if let Some(outer) = self.outer.next() {
                self.front_inner = Some(outer.into_iter());
            } else {
                // If there's no next outer, check if there is there is one in back
                if let Some(back) = &mut self.back_inner {
                    if let Some(next) = back.next() {
                        return Some(next);
                    }
                }
                return None;
            }
        }
    }
}

impl<T> DoubleEndedIterator for Flatten<T>
where
    T: IntoIterator,
    T::Item: IntoIterator,
    T::IntoIter: DoubleEndedIterator,
    <T::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(back) = &mut self.back_inner {
                if let Some(next) = back.next_back() {
                    return Some(next);
                }
            }
            if let Some(outer) = self.outer.next_back() {
                self.back_inner = Some(outer.into_iter());
            } else {
                // If there's no next outer, check if there is there is one in front
                if let Some(front) = &mut self.front_inner {
                    if let Some(next) = front.next_back() {
                        return Some(next);
                    }
                }
                return None;
            }
        }
    }
}

pub fn flatten<T>(i: T) -> Flatten<T>
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    Flatten {
        outer: i.into_iter(),
        front_inner: None,
        back_inner: None,
    }
}

#[test]
fn two_vecs() {
    assert_eq!(
        flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]).collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn empty_middle_vec() {
    assert_eq!(
        flatten(vec![vec![1, 2, 3], vec![], vec![4, 5, 6]]).collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn empty_starting_vec() {
    assert_eq!(
        flatten(vec![vec![], vec![4, 5, 6]]).collect::<Vec<_>>(),
        vec![4, 5, 6]
    );
}

#[test]
fn empty() {
    let mut iter: Flatten<Vec<Vec<usize>>> = flatten(vec![vec![]]);

    let my_vec: Vec<usize> = vec![];
    let my_vec2: Vec<usize> = vec![];
    let wide = vec![my_vec, my_vec2];

    let mut flattened = flatten(wide);

    assert_eq!(flattened.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn next_back() {
    let mut iter = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]);

    assert_eq!(iter.next_back(), Some(6));
    assert_eq!(iter.next_back(), Some(5));
    assert_eq!(iter.next_back(), Some(4));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn both_ways() {
    let mut iter = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]);

    assert_eq!(iter.next_back(), Some(6));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next_back(), Some(5));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), None);
}
