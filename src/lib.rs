pub fn str_to_owned(src: &str, dst: &mut String) {
    *dst = src.to_owned();
}

pub fn str_to_string(src: &str, dst: &mut String) {
    *dst = src.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_owned() {
        let fns = vec![str_to_owned, str_to_string];
        for f in fns {
            let src = "hello";
            let mut dst = String::new();
            f(&src, &mut dst);

            assert_eq!(dst, "hello")
        }
    }
}
