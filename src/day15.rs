use crate::Solution;

#[derive(Default)]
pub struct Day15 {}

impl Solution for Day15 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day15");
        hash_csv(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day15");
        collect_lenses(input)
    }
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, ch| {
        let code = ch as usize;
        ((acc + code) * 17) % 256
    })
}

fn hash_csv(s: &str) -> usize {
    s.trim().split(',').map(hash).sum()
}

type Lens = (&'static str, usize);
type Boxes = Vec<Vec<Lens>>;

fn collect_lenses(s: &'static str) -> usize {
    let mut boxes: Boxes = vec![vec![]; 256];

    s.trim().split(',').for_each(|instruction| {
        if instruction.contains('=') {
            let (label, focal_length) = instruction.split_once('=').unwrap();
            let lens = (label, focal_length.parse().unwrap());
            add_lens(&mut boxes, lens);
        } else {
            let label = instruction.strip_suffix('-').unwrap();
            rm_lens(&mut boxes, label);
        }
    });

    let mut focusing_power = 0;

    for b in 0..boxes.len() {
        for s in 0..boxes[b].len() {
            let focal_length = boxes[b][s].1;
            focusing_power += (b + 1) * (s + 1) * focal_length;
        }
    }

    focusing_power
}

fn add_lens(boxes: &mut Boxes, lens: Lens) {
    let box_index = hash(lens.0);
    if let Some(existing_lens_index) = boxes[box_index]
        .iter()
        .position(|(label, _)| *label == lens.0)
    {
        boxes[box_index][existing_lens_index] = lens;
    } else {
        boxes[box_index].push(lens);
    }
}

fn rm_lens(boxes: &mut Boxes, label_to_remove: &'static str) {
    let box_index = hash(label_to_remove);
    if let Some(existing_lens_index) = boxes[box_index]
        .iter()
        .position(|(label, _)| *label == label_to_remove)
    {
        boxes[box_index].remove(existing_lens_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1() {
        assert_eq!(hash_csv(INPUT_1), 1320);
    }

    #[test]
    fn part_2() {
        assert_eq!(collect_lenses(INPUT_1), 145);
    }
}
