pub struct Flatten<T>
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    outer: T::IntoIter,
    inner: Option<<T::Item as IntoIterator>::IntoIter>,
}

impl<T> Iterator for Flatten<T>
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    type Item = <T::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = &mut self.inner {
                if let Some(next) = inner.next() {
                    return Some(next);
                }
            }
            if let Some(outer) = self.outer.next() {
                self.inner = Some(outer.into_iter());
            } else {
                return None;
            }
        }
    }
}

pub fn flatten<T: IntoIterator>(i: T) -> Flatten<T>
where
    T::Item: IntoIterator,
{
    Flatten {
        outer: i.into_iter(),
        inner: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(iter.next(), None);
    }
}
