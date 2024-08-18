// ANCHOR: problem-1
pub fn max_depth(s: String) -> i32 {
    let mut stack = Vec::new();
    let mut max_len = 0;
    for char in s.chars() {
        match char {
            '(' => {
                stack.push(1);
                max_len= max_len.max(stack.len());
            },
            ')' => _ = stack.pop(),
            _ => (),
        }
    }
    max_len as i32
}
// ANCHOR_END: problem-1