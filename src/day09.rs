use itertools::{repeat_n, Itertools};

super::solve!("09");

fn parse(input: &str) -> Vec<Option<usize>> {
    expand_disk(input)
}

fn part_1(disk: &[Option<usize>]) -> usize {
    let mut disk = disk.to_vec();
    let mut seen = 0;
    while let Some(free) = disk.iter().skip(seen).position(Option::is_none) {
        disk[seen + free] = disk.pop().unwrap();
        seen += free;
        while disk.last().unwrap().is_none() {
            disk.pop();
        }
    }
    checksum(&disk)
}

fn part_2(disk: &[Option<usize>]) -> usize {
    let mut disk = disk.to_vec();
    let mut free_spaces = spaces(&disk);
    let mut head = disk.len() - 1;
    while head > 0 {
        if let Some(file_id) = disk[head] {
            let required = disk[..=head]
                .iter()
                .rev()
                .take_while(|&x| *x == Some(file_id))
                .count();
            if let Some(space) = free_spaces.iter_mut().find(|(offset, available)| {
                *offset < head.saturating_sub(required) && *available >= required
            }) {
                disk[space.0..space.0 + required].fill(Some(file_id));
                disk[head + 1 - required..=head].fill(None);
                space.0 += required;
                space.1 -= required;
            }
            head = head.saturating_sub(required);
        } else {
            head -= 1;
        }
    }
    checksum(&disk)
}

fn expand_disk(s: &str) -> Vec<Option<usize>> {
    s.chars()
        .take_while(|c| c.is_ascii_digit())
        .enumerate()
        .flat_map(|(i, c)| {
            let elem = if i % 2 == 0 { Some(i / 2) } else { None };
            repeat_n(elem, c.to_digit(10).unwrap() as usize)
        })
        .collect_vec()
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, f)| f.map(|f| f * i))
        .sum()
}

fn spaces(disk: &[Option<usize>]) -> Vec<(usize, usize)> {
    let mut rv = vec![];
    let mut offset = 0;
    while offset < disk.len() {
        if disk[offset].is_none() {
            let size = disk[offset..].iter().take_while(|x| x.is_none()).count();
            rv.push((offset, size));
            offset += size;
        } else {
            offset += 1;
        }
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 1928);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 2858);
    }
}
