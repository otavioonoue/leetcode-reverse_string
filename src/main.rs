fn main() {
    Solution::reverse_string(
        &mut Vec::from("hello")
            .iter()
            .map(|c| *c as char)
            .collect()
    );
}

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();

        for i in 0..len {
            if len / 2 == i {
                break;
            }
            let ptr1: *mut char = &mut s[i];
            let ptr2: *mut char = &mut s[len - 1 - i];
            let temp: char = unsafe { *ptr1 };
            unsafe { *ptr1 = *ptr2; }
            unsafe { *ptr2 = temp; }
        }

        println!("{:#?}", s);
    }
}