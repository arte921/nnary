use std::str;

fn main() {
    for i in 0..=20 {

        println!("i = {}, result = {}", i, naar_ntallig(i, 0, 20).0);
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

fn symbool (n: u32) -> str {
    let zero = '\u{1D2E}';
    let total = (zero as u32) + n;
    
}

/*
        if n < 10 {
            48
        } else {
            55
        }*/
    