
fn main() {
    println!("{}", where_is_nemo("I am finding Nemo !".to_string()));
}

fn where_is_nemo(sentence: String) -> String {
    let nemo = "Nemo ";
    match sentence.as_str().rfind(nemo) {
        Some(val) => {
            return format!("I found Nemo at {}", val+1).to_string();
        },
        None => {
            return "I can't find Nemo :(".to_string();
        }
    }
}