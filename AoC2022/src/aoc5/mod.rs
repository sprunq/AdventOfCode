pub fn p1() {
    let mut boxes = vec![Vec::<char>::new(); 9];
    include_str!("boxes.txt").lines().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, c)| {
            if c.is_ascii_alphabetic() {
                boxes[idx / 4].insert(0, c)
            }
        });
    });

    let moves = include_str!("moves.txt")
        .lines()
        .map(|a| a.split(' '))
        .map(|b| {
            let mut ret: [usize; 3] = [0; 3];
            for (idx, p) in b.enumerate() {
                if let Ok(val) = p.parse::<usize>() {
                    ret[idx / 2] = val
                }
            }
            ret
        })
        .collect::<Vec<_>>();

    for mov in moves {
        let [amount, from, to] = mov;
        for _ in 0..amount {
            let val = boxes[from - 1].pop().unwrap();
            boxes[to - 1].push(val);
        }
    }

    boxes.iter().for_each(|x| print!("{}", x.last().unwrap()));
}

pub fn p2() {
    let mut boxes = vec![Vec::<char>::new(); 9];
    include_str!("boxes.txt").lines().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, c)| {
            if c.is_ascii_alphabetic() {
                boxes[idx / 4].insert(0, c)
            }
        });
    });

    let moves = include_str!("moves.txt")
        .lines()
        .map(|a| a.split(' '))
        .map(|b| {
            let mut ret: [usize; 3] = [0; 3];
            for (idx, p) in b.enumerate() {
                if let Ok(val) = p.parse::<usize>() {
                    ret[idx / 2] = val
                }
            }
            ret
        })
        .collect::<Vec<_>>();

    for mov in moves {
        let [amount, from, to] = mov;
        let mut buffer = Vec::<char>::with_capacity(amount);
        for _ in 0..amount {
            let val = boxes[from - 1].pop().unwrap();
            buffer.push(val);
        }
        buffer.reverse();
        boxes[to - 1].extend(buffer)
    }

    boxes.iter().for_each(|x| print!("{}", x.last().unwrap()));
}
