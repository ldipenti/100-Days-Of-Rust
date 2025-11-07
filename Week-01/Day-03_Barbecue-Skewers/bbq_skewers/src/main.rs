fn main() {
    let grills = [
        [
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ],
        [
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ],
        [
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ]
    ];

    for (grill_nr, grill) in grills.into_iter().enumerate() {
        let [veg, non_veg] = analyze_grill(grill);
        println!("Grill #{} contents: {} veg skewers, {} non-veg skewers", grill_nr, veg, non_veg)
    }
}

fn analyze_grill(_grill:[&str; 5]) -> [u8;2] {
    [0,0]
}