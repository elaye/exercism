pub fn raindrops(n: usize) -> String {
    let mut drops = Vec::new();

    if n % 3 == 0 {
        drops.push("Pling");
    }

    if n % 5 == 0 {
        drops.push("Plang");
    }

    if n % 7 == 0 {
        drops.push("Plong");
    }

    if drops.len() > 0 {
        return drops.join("");
    }

    format!("{}", n)
}
