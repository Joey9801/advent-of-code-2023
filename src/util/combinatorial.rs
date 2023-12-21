pub fn pair_indices(len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..len).flat_map(move |i| (i + 1..len).map(move |j| (i, j)))
}

pub fn pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    pair_indices(slice.len()).map(move |(i, j)| (&slice[i], &slice[j]))
}
