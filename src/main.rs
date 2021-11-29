mod html;

fn main() {
    let mut p = html::parser::Parser::new();
    let ans = p.parse("");
    println!("{}", format!(">>> Hello, world! {}", ans));
}
