#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T>(first_list: &[T], second_list: &[T]) -> Comparison
where
    T: PartialEq,
{
    let is_sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|sub_second_list| sub_second_list == first_list);

    let is_superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|sub_first_list| sub_first_list == second_list);

    match (is_sublist, is_superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
