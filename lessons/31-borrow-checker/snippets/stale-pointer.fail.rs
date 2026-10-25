fn main() {
    let mut s = vec![1, 2, 3, 4];

    // Go: p := &s[0]; s = append(s, 5); *p = 9
    let p = &mut s[0];
    s.push(5);
    *p = 9;

    // Go: b := s[:2]; s = append(s, 6); b[0] = 7
    let b = &mut s[..2];
    s.push(6);
    b[0] = 7;
}
