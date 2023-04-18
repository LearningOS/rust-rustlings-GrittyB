// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}
// 不要想着改变传入的这个vec里面的元素, 因为它声明的时候就是不可变的, 所以只能创建新的vec
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut vec = self.clone();
        vec.push("Bar".to_string());
        vec
    }
}
//TODO: Add your code here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
