#![feature(test)]

extern crate spelling_corrector;
extern crate test;

use test::Bencher;
use std::io::prelude::*;
use std::fs::File;

#[bench]
fn bench_creation(b: &mut Bencher) {
    let mut f = File::open("big.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    b.iter(|| {
        spelling_corrector::SimpleSpellChecker::new(&s);
    });
}

#[bench]
fn bench_correction(b: &mut Bencher) {
    let mut f = File::open("big.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let spell_checker = spelling_corrector::SimpleSpellChecker::new(&s);

    b.iter(|| {
        spell_checker.correct("speling");
    });
}