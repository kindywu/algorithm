fn main() {
    let arr = vec!["noon", "no1on", "abcd", "1234554321"];
    for s in arr {
        let flag = palindrome(s);
        println!("{} is palindrome? {}", s, flag);
    }
}

fn palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mid = len / 2;
    for i in 0..mid {
        if bytes[i] != bytes[len - i - 1] {
            return false;
        }
    }
    true
}
