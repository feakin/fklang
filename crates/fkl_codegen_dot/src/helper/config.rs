pub static INDENT_SIZE: usize = 2;

pub fn indent(depth: usize) -> String {
    return " ".repeat(INDENT_SIZE * depth)
}
