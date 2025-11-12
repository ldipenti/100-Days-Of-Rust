fn main() {
    let socks = [
        "AA",
        "ABABC",
        "CABBACCC",
        ""
    ];

    for (test_nr, sock_set) in socks.iter().enumerate() {
        println!("Sock set #{} has {} sock pairs", test_nr, sock_pairs(sock_set));
    }
}

fn sock_pairs(_sock_set: &str) -> u32 {
    0
}
