pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    if list.is_empty() {
        return result;
    }
    let mut i = 1;
    while i < list.len() {
        result.push_str(&format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]));
        i += 1;
    }
    result.push_str(&format!("And all for the want of a {}.", list[0]));

    return result;
}
