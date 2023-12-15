fn main() {
    let inp = std::fs::read_to_string("../input/15").unwrap().trim().to_string();

    let mut ans = 0;

    let mut boxes: Vec<Vec<String>> = vec![vec![]; 256];

    for line in inp.split(',') {
        let mut hash = 0usize;

        let mut to_hash = Vec::new();

        if let Some((label, _)) = line.split_once("=") {
            to_hash = label.chars().collect::<Vec<_>>();
        } else {
            to_hash = line.split('-').collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        }

        for ch in to_hash {
            hash += (ch as u8) as usize;
            hash = (hash * 17) % 256;
        }

        println!("{hash}");

        if let Some((label, n)) = line.split_once("=") {
            let n = n.parse::<usize>().unwrap();
            let mut rm = None;

            for i in 0..boxes[hash].len() {
                if boxes[hash][i].contains(&label.to_string()) {
                    rm = Some(i);
                    break;
                }
            }

            if let Some(i) = rm {
                boxes[hash][i] = format!("{} {}", label.to_string(), n);
            } else {
                boxes[hash].push(format!("{} {}", label.to_string(), n));
            }
        } else {
            let rst = line.split('-').collect::<Vec<_>>()[0];
            let mut rm = None;

            for j in 0..boxes[hash].len() {
                if boxes[hash][j].contains(rst) {
                    rm = Some(j);
                    break;
                }
            }

            if let Some(j) = rm {
                boxes[hash].remove(j);
            }
        }

    }

    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            let (la, n) = boxes[i][j].split_once(" ").unwrap();

            let n = n.parse::<usize>().unwrap();

            ans += n * (i + 1) * (j + 1);
        }
    }
    
    println!("{ans}");
}
