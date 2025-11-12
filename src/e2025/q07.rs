fn parse_input(input: &str) -> (Vec<&str>, [[u8; 128]; 128]) {
    let mut lines = input.lines();
    let names = lines.next().unwrap().split(',').collect::<Vec<&str>>();
    lines.next();
    let mut rules = [[0u8; 128]; 128];
    for l in lines {
        let c = l.as_bytes()[0];
        let r = l[4..].split(',').collect::<Vec<&str>>();
        for s in r {
            let d = s.as_bytes()[0];
            rules[c as usize][d as usize] = 1;                        
        }
    }
    (names, rules)
}

fn is_valid(name: &str, rules: &[[u8; 128]]) -> bool {
    let bytes = name.as_bytes();
    for i in 0..bytes.len() - 1 {
        let a = bytes[i];
        let b = bytes[i + 1];
        if rules[a as usize][b as usize] == 0 {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> String {
    let (names, rules) = parse_input(input);
    for name in names.iter() {
        if is_valid(name, &rules) {
            return name.to_string();
        }
    }
    "".to_string()
}

pub fn part2(input: &str) -> String {
    let (names, rules) = parse_input(input);
    let mut ret = 0;
    for (pos, name) in names.iter().enumerate() {
        if is_valid(name, &rules) {
            ret += pos + 1;
        }
    }
    ret.to_string()
}

fn count_dfs(
    rules: &[[u8; 128]],
    cache: &mut [[u64; 128]; 12],
    node: u8,
    len: usize,
) -> u64 {
    let mut total = 0u64;
    if cache[len][node as usize] > 0 {
        return cache[len][node as usize];
    }
    if len >= 7 {
        total += 1;
    }
    if len < 11 {
        for dest in 64..128 {
            if rules[node as usize][dest] == 1 {
                total += count_dfs(rules, cache, dest as u8, len + 1);
            }
        }        
    }
    cache[len][node as usize] = total;
    total
}

pub fn part3(input: &str) -> String {
    let (mut names, rules) = parse_input(input);
    let mut ret = 0;
    names.sort();
    // remove duplicates
    for i in 0..names.len() {
        let name = names[i];
        if name.is_empty() {
            continue;
        }
        for j in  i+1..names.len() {
            let other = names[j];
            if other.starts_with(name) {
                names[j] = "";
            }
        }
    }    
    names.retain(|name| !name.is_empty());
    let mut cache = [[0u64; 128]; 12];
    for name in names {
        let bytes = name.as_bytes();
        if !is_valid(name, &rules) {
            continue
        }
        let last = bytes[bytes.len() - 1];
        ret += count_dfs(&rules, &mut cache, last as u8, bytes.len());
    }
    ret.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Oronris,Urakris,Oroneth,Uraketh\n\n\
        r > a,i,o\ni > p,w\nn > e,r\no > n,m\nk > f,r\na > k\nU > r\ne > t\nO > r\nt > h";
        assert_eq!(part1(input), "Oroneth");
    }

    #[test]
    fn test_part2() {
        let input = "Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax\n\n\
        r > v,e,a,g,y\na > e,v,x,r\ne > r,x,v,t\nh > a,e,v\ng > r,y\ny > p,t\ni > v,r\nK > h\n\
        v > e\nB > r\nt > h\nN > e\np > h\nH > e\nl > t\nz > e\nX > a\nn > v\nx > z\nT > i";
        assert_eq!(part2(input), "23");
    }

    #[test]
    fn test_part3() {
        let input = "Khara,Xaryt,Noxer,Kharax\n\n\
        r > v,e,a,g,y\na > e,v,x,r,g\ne > r,x,v,t\nh > a,e,v\ng > r,y\ny > p,t\ni > v,r\nK > h\n\
        v > e\nB > r\nt > h\nN > e\np > h\nH > e\nl > t\nz > e\nX > a\nn > v\nx > z\nT > i";
        assert_eq!(part3(input), "1154");
    }


}
