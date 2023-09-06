wai_bindgen_rust::export!("rusty-sorter.wai");

pub struct RustySorter;

impl rusty_sorter::RustySorter for RustySorter {
    fn bubble_sort(a:Vec<i32>,) -> Vec<i32> {
        let mut unsorted_array = a.clone();
        for i in 0..a.len() {
            for j in 0..(a.len() - i - 1) {
                if unsorted_array[j] > unsorted_array[j + 1] {
                    unsorted_array.swap(j, j+1);
                }
            }
        }

        unsorted_array
    }
}
