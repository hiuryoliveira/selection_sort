fn main() {
    let mut data = vec![4, 2, 3, 19, 29, 14, 7, 84, 9, 3, 77, 45, 37, 2, 84, 88,30, 1];
    sortable::selection_sort(&mut data);
    println!("{:?}", data);
}

mod sortable {
    pub(crate) fn selection_sort(data : &mut Vec<i32>) {
        let n = data.len();
        for i in 0..n {
            let mut min = i;
            for j in i+1..n {
                if data[j] < data[min] {
                    min = j;
                }
            }
            data.swap(i, min);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut data = vec![4, 2, 3, 19, 29, 14, 7, 84, 9, 3, 77, 45, 37, 2, 84, 88, 30, 1];
        sortable::selection_sort(&mut data);
        assert_eq!(data, vec![1, 2, 2, 3, 3, 4, 7, 9, 14, 19, 29, 30, 37, 45, 77, 84, 84, 88]);
    }
}
