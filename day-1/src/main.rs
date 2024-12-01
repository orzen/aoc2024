use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result};

fn distance(left: &[u32], right: &[u32]) -> u32 {
    let combined: u32 = left
        .iter()
        .zip(right.iter())
        .fold(vec![], |mut acc, (x, y)| {
            let distance = if x > y { x - y } else { y - x };

            acc.push(distance);

            acc
        })
        .into_iter()
        .sum();

    combined
}

fn similarity_score(left: &[u32], right: &[u32]) -> u32 {
    let mut map = HashMap::new();

    right.iter().for_each(|x| match map.get(x) {
        Some(v) => {
            map.insert(x, *v + 1);
        }
        None => {
            map.insert(x, 1);
        }
    });

    left.iter().fold(0, |mut acc, x| {
        if let Some(right_score) = map.get(x) {
            acc += x * right_score;
            acc
        } else {
            acc
        }
    })
}

fn file_to_lists(file: &Path) -> Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open(file).context("open file")?;

    let input = BufReader::new(file);

    let lists = input.lines().fold(
        (Vec::<u32>::new(), Vec::<u32>::new()),
        |(mut a1, mut a2), line| {
            if let Ok(l) = line {
                if l.is_empty() {
                    return (a1, a2);
                }

                let mut split = l.split_whitespace();

                let left = match split.next().unwrap_or_default().parse::<u32>() {
                    Ok(v) => v,
                    Err(e) => {
                        println!("failed to convert left to int, error: {e}");
                        0
                    }
                };

                let right = match split.next().unwrap_or_default().parse::<u32>() {
                    Ok(v) => v,
                    Err(e) => {
                        println!("failed to convert left to int, error: {e}");
                        0
                    }
                };

                println!("line: {l}, left: {left}, right: {right}");

                a1.push(left);
                a2.push(right);

                (a1, a2)
            } else {
                println!("bad line");
                (a1, a2)
            }
        },
    );

    Ok(lists)
}

fn main() -> Result<()> {
    let file = Path::new("./input.txt");

    let (left, right) = file_to_lists(file)?;

    let result = distance(&left, &right);
    println!("distance: {result}");

    let result = similarity_score(&left, &right);
    println!("similarity score: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{distance, similarity_score};

    #[test]
    fn test_distance() {
        let mut left: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let mut right: Vec<u32> = vec![4, 3, 5, 3, 9, 3];

        left.sort();
        right.sort();

        let r = distance(&left, &right);

        assert_eq!(11, r);
    }

    #[test]
    fn test_similarity_score() {
        let mut left: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let mut right: Vec<u32> = vec![4, 3, 5, 3, 9, 3];

        left.sort();
        right.sort();

        let r = similarity_score(&left, &right);

        assert_eq!(31, r);
    }
}
