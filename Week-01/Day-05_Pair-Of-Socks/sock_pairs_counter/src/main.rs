use std::collections::HashMap;

fn main() {
    let socks = ["AA", "ABABC", "CABBACCC", ""];

    for (test_nr, sock_set) in socks.iter().enumerate() {
        println!(
            "Sock set #{} has {} sock pairs",
            test_nr,
            sock_pairs(sock_set)
        );
    }
}

fn sock_pairs(sock_set: &str) -> u32 {
    let mut sock_kinds = HashMap::new();
    let mut sock_pairs = 0;
    for sock in sock_set.as_bytes().iter() {
        sock_kinds.insert(sock, sock_kinds.get(sock).unwrap_or(&0) + 1);
    }
    for sock_kind in sock_kinds.keys() {
        sock_pairs += sock_kinds.get(sock_kind).unwrap() / 2;
    }
    sock_pairs
}
