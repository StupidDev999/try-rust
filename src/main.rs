use extabs::Expand;

fn main() {
    let s = String::from("Hewwo\tWorld!");
    let expanded = s.expandtabs(4);

    println!("{}", expanded);
}
