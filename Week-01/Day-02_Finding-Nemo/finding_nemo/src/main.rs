
fn main() {
    println!("{}", where_is_nemo("I am finding Nemo !".to_string()));
    println!("{}", where_is_nemo("Nemo is me".to_string()));
    println!("{}", where_is_nemo("I Nemo am".to_string()));
}

fn where_is_nemo(sentence: String) -> String {
    let nemo = "Nemo";
    let mut pos = 0;
    let mut nemo_pos = pos;
    for word in sentence.split(' ') {
        if word == nemo {
            nemo_pos = pos+1;
            break;
        }
        pos += 1;
    }

    if nemo_pos > 0 {
        return format!("I found Nemo at {}!", nemo_pos).to_string();
    } else {
        return "I can't find Nemo :(".to_string();
    }
}