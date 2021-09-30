mod html;

fn main() {
    let mut p = html::html_parser::Parser::new();
    let ans = p.parse("");
    println!("{}", format!(">>> Hello, world! {}", ans));
}
