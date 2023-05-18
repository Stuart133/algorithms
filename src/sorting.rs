fn insertion_sort<T: PartialOrd + Copy>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j] < data[j - 1] {
            swap(data, j, j - 1);
            j = j - 1;
        }
    }
}

fn swap<T: Copy>(data: &mut [T], i: usize, j: usize) {
    let tmp = data[i];
    data[i] = data[j];
    data[j] = tmp;
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    pub fn insertion_sort_sorts() {
        let mut seq = [1, 54, 1, 13, 10];

        insertion_sort(&mut seq);

        assert_eq!([1, 1, 10, 13, 54], seq);
    }
}
