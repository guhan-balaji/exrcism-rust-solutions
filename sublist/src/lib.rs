#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let sublist_exists = match first_list.len() <= second_list.len() {
        true => _sublist(first_list, second_list),
        false => _sublist(second_list, first_list),
    };

    if sublist_exists {
        match first_list.len() > second_list.len() {
            _ if first_list.len() == second_list.len() => Comparison::Equal,
            true => Comparison::Superlist,
            false => Comparison::Sublist,
        }
    } else {
        Comparison::Unequal
    }
}

// NOTE: Initial call MUST pass the smaller slice (in tems of lentgth) as first argument.
fn _sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if b.len() < a.len() {
        return false;
    };

    if a[..] == b[..a.len()] {
        return true;
    } else {
        _sublist(a, &b[1..])
    }
}
