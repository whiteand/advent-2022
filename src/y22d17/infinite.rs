pub fn infinite<'i, T>(res: &'i [T]) -> impl Iterator<Item = &T> + 'i {
    std::iter::successors(Some(0), |&ind| Some((ind + 1) % res.len())).map(|ind| &res[ind])
}
