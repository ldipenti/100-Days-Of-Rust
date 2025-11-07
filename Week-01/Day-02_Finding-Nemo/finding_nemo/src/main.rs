
fn main() {
    println!("{}", where_is_nemo("I am finding Nemo !".to_string()));
    println!("{}", where_is_nemo("Nemo is me".to_string()));
    println!("{}", where_is_nemo("I Nemo am".to_string()));
    println!("{}", where_is_nemo("I am Doris , and you ?".to_string()));
}

fn where_is_nemo(sentence: String) -> String {
    let nemo = "Nemo";
    for (item, word) in sentence.split(' ').enumerate() {
        if word == nemo {
            return format!("I found Nemo at {}!", item+1).to_string();
        }
    }
    return "I can't find Nemo :(".to_string();
}