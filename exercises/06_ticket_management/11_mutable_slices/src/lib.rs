// TODO: Define a function named `lowercase` that converts all characters in a string to lowercase,
//  modifying the input in place.
//  Does it need to take a `&mut String`? Does a `&mut [str]` work? Why or why not?

pub fn lowercase(s: &mut str) {
    let lower = s.to_lowercase();
    // 使用切片的方式重写字符串内容
    for (i, c) in lower.chars().enumerate() {
        if let Some(dst) = s.get_mut(i..=i) {
            unsafe {
                // 将字符替换为小写字符
                std::ptr::write(dst.as_bytes_mut().as_mut_ptr() as *mut u8, c as u8);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = String::from("");
        lowercase(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn one_char() {
        let mut s = String::from("A");
        lowercase(&mut s);
        assert_eq!(s, "a");
    }

    #[test]
    fn multiple_chars() {
        let mut s = String::from("Hello, World!");
        lowercase(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mut_slice() {
        let mut s = "Hello, World!".to_string();
        lowercase(s.as_mut_str());
        assert_eq!(s, "hello, world!");
    }
}
