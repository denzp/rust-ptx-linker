use std::mem::replace;

pub trait IteratorExt: Sized + Iterator {
    fn keep_last_pair(self, needle: Self::Item) -> KeepLastPairIter<Self> {
        KeepLastPairIter {
            needle,
            last: PairState::None,
            iter: self,
        }
    }
}

impl<T> IteratorExt for T where T: Iterator {}

enum PairState<T> {
    None,
    Complete(T, T),
    Partial(T),
}

pub struct KeepLastPairIter<I: Iterator> {
    needle: I::Item,
    last: PairState<I::Item>,
    iter: I,
}

impl<I> Iterator for KeepLastPairIter<I>
where
    I: Iterator,
    I::Item: Clone + PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(head) => {
                if head == self.needle {
                    match self.iter.next() {
                        Some(next) => {
                            self.last = PairState::Complete(head, next);
                            self.iter.next()
                        }

                        None => Some(head),
                    }
                } else {
                    Some(head)
                }
            }

            None => match replace(&mut self.last, PairState::None) {
                PairState::Complete(first, second) => {
                    self.last = PairState::Partial(second);
                    Some(first)
                }

                PairState::Partial(second) => {
                    self.last = PairState::None;
                    Some(second)
                }

                PairState::None => None,
            },
        }
    }
}
