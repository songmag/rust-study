pub fn bubble_sort<T>(items: &mut Vec<T>)
where
    T: Clone + std::cmp::PartialOrd + std::fmt::Display,
{
    for x in 0..items.len() {
        for y in (x + 1..items.len()).rev() {
            if items[y - 1] > items[y] {
                items.swap(y-1,y);
            }
        }
    }
}

pub fn selection_sort<T>(items: &mut Vec<T>)
where
    T: Clone + std::cmp::PartialOrd + std::fmt::Display,
{

    for x in 0..items.len() {
        let mut selection_index = x;
        for y in x..items.len() {
            if items[y] < items[selection_index] {
                 selection_index = y
            } 
        }
        items.swap(x,selection_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut v1: Vec<i16> = vec![2, 4, 5, 1];
        bubble_sort(&mut v1);

        assert_eq!(v1, &[1,2,4,5]);
    }

    #[test]
    fn selection_sort_test() {
        let mut v1: Vec<i16> = vec![2, 4, 5, 1];
        selection_sort(&mut v1);

        assert_eq!(v1, &[1,2,4,5]);
    }
}
