//@ run-pass
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
// Test how resolving a projection interacts with inference.  In this
// case, we were eagerly unifying the type variable for the iterator
// type with `I` from the where clause, ignoring the in-scope `impl`
// for `ByRef`. The right answer was to consider the result ambiguous
// until more type information was available.

#![feature(lang_items)]
#![no_implicit_prelude]

use std::marker::Sized;
use std::option::Option::{None, Some, self};

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

trait IteratorExt: Iterator + Sized {
    fn by_ref(&mut self) -> ByRef<'_, Self> {
        ByRef(self)
    }
}

impl<I> IteratorExt for I where I: Iterator {}

struct ByRef<'a, I: 'a + Iterator>(&'a mut I);

impl<'a, I: Iterator> Iterator for ByRef<'a, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option< <I as Iterator>::Item > {
        self.0.next()
    }
}

fn is_iterator_of<A, I: Iterator<Item=A>>(_: &I) {}

fn test<A, I: Iterator<Item=A>>(mut it: I) {
    is_iterator_of::<A, _>(&it.by_ref());
}

fn test2<A, I1: Iterator<Item=A>, I2: Iterator<Item=I1::Item>>(mut it: I2) {
    is_iterator_of::<A, _>(&it)
}

fn main() { }
