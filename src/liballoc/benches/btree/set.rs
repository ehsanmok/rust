use std::collections::BTreeSet;
use std::collections::btree_set::Intersection;

use rand::{thread_rng, Rng};
use test::{black_box, Bencher};

fn random(n1: usize, n2: usize) -> [BTreeSet<usize>; 2] {
    let mut rng = thread_rng();
    let mut sets = [BTreeSet::new(), BTreeSet::new()];
    for i in 0..2 {
        while sets[i].len() < [n1, n2][i] {
            sets[i].insert(rng.gen());
        }
    }
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

fn stagger(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
    let n2 = n1 * factor;
    let mut sets = [BTreeSet::new(), BTreeSet::new()];
    for i in 0..(n1 + n2) {
        let b = i % (factor + 1) != 0;
        sets[b as usize].insert(i as u32);
    }
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

fn neg_vs_pos(n1: usize, n2: usize) -> [BTreeSet<i32>; 2] {
    let mut neg = BTreeSet::new();
    let mut pos = BTreeSet::new();
    for i in -(n1 as i32)..=-1 {
        neg.insert(i);
    }
    for i in 1..=(n2 as i32) {
        pos.insert(i);
    }
    assert_eq!(neg.len(), n1);
    assert_eq!(pos.len(), n2);
    [neg, pos]
}

fn pos_vs_neg(n1: usize, n2: usize) -> [BTreeSet<i32>; 2] {
    let mut sets = neg_vs_pos(n2, n1);
    sets.reverse();
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

fn intersection_search<T>(sets: &[BTreeSet<T>; 2]) -> Intersection<T>
    where T: std::cmp::Ord
{
    Intersection::Search {
        a_iter: sets[0].iter(),
        b_set: &sets[1],
    }
}

fn intersection_stitch<T>(sets: &[BTreeSet<T>; 2]) -> Intersection<T>
    where T: std::cmp::Ord
{
    Intersection::Stitch {
        a_iter: sets[0].iter(),
        b_iter: sets[1].iter(),
    }
}

macro_rules! intersection_bench {
    ($name: ident, $sets: expr) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = sets[0].intersection(&sets[1]).count();
                black_box(x);
            })
        }
    };
    ($name: ident, $sets: expr, $intersection_kind: ident) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            // setup
            let sets = $sets;
            assert!(sets[0].len() >= 1);
            assert!(sets[1].len() >= sets[0].len());

            // measure
            b.iter(|| {
                let x = $intersection_kind(&sets).count();
                black_box(x);
            })
        }
    };
}

intersection_bench! {intersect_100_neg_vs_100_pos,      neg_vs_pos(100, 100)}
intersection_bench! {intersect_100_neg_vs_10k_pos,      neg_vs_pos(100, 10_000)}
intersection_bench! {intersect_100_pos_vs_100_neg,      pos_vs_neg(100, 100)}
intersection_bench! {intersect_100_pos_vs_10k_neg,      pos_vs_neg(100, 10_000)}
intersection_bench! {intersect_10k_neg_vs_100_pos,      neg_vs_pos(10_000, 100)}
intersection_bench! {intersect_10k_neg_vs_10k_pos,      neg_vs_pos(10_000, 10_000)}
intersection_bench! {intersect_10k_pos_vs_100_neg,      pos_vs_neg(10_000, 100)}
intersection_bench! {intersect_10k_pos_vs_10k_neg,      pos_vs_neg(10_000, 10_000)}
intersection_bench! {intersect_random_100_vs_100_actual,random(100, 100)}
intersection_bench! {intersect_random_100_vs_100_search,random(100, 100), intersection_search}
intersection_bench! {intersect_random_100_vs_100_stitch,random(100, 100), intersection_stitch}
intersection_bench! {intersect_random_100_vs_10k_actual,random(100, 10_000)}
intersection_bench! {intersect_random_100_vs_10k_search,random(100, 10_000), intersection_search}
intersection_bench! {intersect_random_100_vs_10k_stitch,random(100, 10_000), intersection_stitch}
intersection_bench! {intersect_random_10k_vs_10k_actual,random(10_000, 10_000)}
intersection_bench! {intersect_random_10k_vs_10k_search,random(10_000, 10_000), intersection_search}
intersection_bench! {intersect_random_10k_vs_10k_stitch,random(10_000, 10_000), intersection_stitch}
intersection_bench! {intersect_stagger_100_actual,      stagger(100, 1)}
intersection_bench! {intersect_stagger_100_search,      stagger(100, 1), intersection_search}
intersection_bench! {intersect_stagger_100_stitch,      stagger(100, 1), intersection_stitch}
intersection_bench! {intersect_stagger_10k_actual,      stagger(10_000, 1)}
intersection_bench! {intersect_stagger_10k_search,      stagger(10_000, 1), intersection_search}
intersection_bench! {intersect_stagger_10k_stitch,      stagger(10_000, 1), intersection_stitch}
intersection_bench! {intersect_stagger_1_actual,        stagger(1, 1)}
intersection_bench! {intersect_stagger_1_search,        stagger(1, 1), intersection_search}
intersection_bench! {intersect_stagger_1_stitch,        stagger(1, 1), intersection_stitch}
intersection_bench! {intersect_stagger_diff1_actual,    stagger(100, 1 << 1)}
intersection_bench! {intersect_stagger_diff1_search,    stagger(100, 1 << 1), intersection_search}
intersection_bench! {intersect_stagger_diff1_stitch,    stagger(100, 1 << 1), intersection_stitch}
intersection_bench! {intersect_stagger_diff2_actual,    stagger(100, 1 << 2)}
intersection_bench! {intersect_stagger_diff2_search,    stagger(100, 1 << 2), intersection_search}
intersection_bench! {intersect_stagger_diff2_stitch,    stagger(100, 1 << 2), intersection_stitch}
intersection_bench! {intersect_stagger_diff3_actual,    stagger(100, 1 << 3)}
intersection_bench! {intersect_stagger_diff3_search,    stagger(100, 1 << 3), intersection_search}
intersection_bench! {intersect_stagger_diff3_stitch,    stagger(100, 1 << 3), intersection_stitch}
intersection_bench! {intersect_stagger_diff4_actual,    stagger(100, 1 << 4)}
intersection_bench! {intersect_stagger_diff4_search,    stagger(100, 1 << 4), intersection_search}
intersection_bench! {intersect_stagger_diff4_stitch,    stagger(100, 1 << 4), intersection_stitch}
intersection_bench! {intersect_stagger_diff5_actual,    stagger(100, 1 << 5)}
intersection_bench! {intersect_stagger_diff5_search,    stagger(100, 1 << 5), intersection_search}
intersection_bench! {intersect_stagger_diff5_stitch,    stagger(100, 1 << 5), intersection_stitch}
intersection_bench! {intersect_stagger_diff6_actual,    stagger(100, 1 << 6)}
intersection_bench! {intersect_stagger_diff6_search,    stagger(100, 1 << 6), intersection_search}
intersection_bench! {intersect_stagger_diff6_stitch,    stagger(100, 1 << 6), intersection_stitch}
