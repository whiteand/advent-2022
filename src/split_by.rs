pub struct SplitBy<Iter, Pred>
where
    Iter: Iterator,
{
    iter: Iter,
    group: Vec<Iter::Item>,
    predicate: Pred,
}

pub trait SplitByTrait<Iter>
where
    Iter: Iterator,
{
    fn split_by<Pred>(self, predicate: Pred) -> SplitBy<Iter, Pred>
    where
        Pred: Fn(&Iter::Item) -> bool;
}

impl<Iter, Pred> Iterator for SplitBy<Iter, Pred>
where
    Iter: Iterator,
    Pred: Fn(&Iter::Item) -> bool,
{
    type Item = Vec<Iter::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_entry = self.iter.next();
        match next_entry {
            Some(value) => {
                if (self.predicate)(&value) {
                    let mut result = Vec::new();
                    std::mem::swap(&mut result, &mut self.group);
                    Some(result)
                } else {
                    self.group.push(value);
                    self.next()
                }
            }
            None if !self.group.is_empty() => {
                let mut result = Vec::new();
                std::mem::swap(&mut result, &mut self.group);
                Some(result)
            }
            None => None,
        }
    }
}

impl<Iter> SplitByTrait<Iter> for Iter
where
    Iter: Iterator,
{
    fn split_by<Pred>(self, predicate: Pred) -> SplitBy<Iter, Pred>
    where
        Pred: Fn(&Iter::Item) -> bool,
    {
        SplitBy {
            iter: self,
            group: Vec::new(),
            predicate,
        }
    }
}
