use serde_json;

fn main() {
    let s = quote(vec!["foo, bar"]);
    println!("{}", s);
}

fn quote(args: Vec<&str>) -> String {
    let mut s = String::new();
    for arg in args {
        let arg = serde_json::to_string(arg).
            unwrap_or_else(|_| { format!("\"{}\"", arg) });
        s.push_str(&arg);
    }
    s
}

#[cfg(test)]
mod test {
    use crate::quote;

    #[test]
    fn test_quote() {
        assert_eq!(quote(vec!["foo, bar"]), "\"foo, bar\"")
    }
}
