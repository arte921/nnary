

fn main() {
    for i in 0..=400 {

        println!("\"normaal\": {}, mayaans {}", i, naar_ntallig(i, 0, 20).0);
    }
}

fn naar_ntallig (n: u32, macht: u32, radix: u32) -> (String, u32) {
    let mut huidige_x = n;
    let mut huidige_string = "".to_string();

    if n >= radix.pow(macht + 1) {
        let waarde = naar_ntallig(n, macht + 1, radix);
        huidige_string = waarde.0;
        huidige_x = waarde.1;
    }

    let huidigemacht = radix.pow(macht);
    let aantal = huidige_x / huidigemacht;
    let rest = huidige_x % huidigemacht;

    let berekend_symbool = symbool(aantal);
    let complete_tekst_uitvoer = format!("{}{}", huidige_string, berekend_symbool);

    (complete_tekst_uitvoer, rest)

}

fn symbool (n: u32) -> String {
    ["𝋠", "𝋡", "𝋢", "𝋣", "𝋤", "𝋥", "𝋦", "𝋧", "𝋨", "𝋩", "𝋪", "𝋫", "𝋬", "𝋭", "𝋮", "𝋯", "𝋰", "𝋱", "𝋲", "𝋳"]
        [n as usize]
        .to_string()
}

/*
        if n < 10 {
            48
        } else {
            55
        }*/
    