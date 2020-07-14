#![feature(test)]
extern crate lazy_static;
extern crate rand;
extern crate test;

use test::Bencher;

extern crate indexmap;

use indexmap::IndexMap;

use std::collections::HashMap;
use std::iter::FromIterator;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::hash::{Hash, Hasher};

use std::borrow::Borrow;
use std::ops::Deref;

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct OneShot<T: ?Sized>(pub T);

impl Hash for OneShot<str> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        h.write(self.0.as_bytes())
    }
}

impl<'a, S> From<&'a S> for &'a OneShot<str>
where
    S: AsRef<str>,
{
    fn from(s: &'a S) -> Self {
        let s: &str = s.as_ref();
        unsafe { &*(s as *const str as *const OneShot<str>) }
    }
}

impl Hash for OneShot<String> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        h.write(self.0.as_bytes())
    }
}

impl Borrow<OneShot<str>> for OneShot<String> {
    fn borrow(&self) -> &OneShot<str> {
        <&OneShot<str>>::from(&self.0)
    }
}

impl<T> Deref for OneShot<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn shuffled_keys<I>(iter: I) -> Vec<I::Item>
where
    I: IntoIterator,
{
    let mut v = Vec::from_iter(iter);
    let mut rng = thread_rng();
    v.shuffle(&mut rng);
    v
}

#[bench]
fn insert_hashmap_string_10_000(b: &mut Bencher) {
    let c = 10_000;
    b.iter(|| {
        let mut map = HashMap::with_capacity(c);
        for x in 0..c {
            map.insert(x.to_string(), ());
        }
        map
    });
}

#[bench]
fn insert_hashmap_string_oneshot_10_000(b: &mut Bencher) {
    let c = 10_000;
    b.iter(|| {
        let mut map = HashMap::with_capacity(c);
        for x in 0..c {
            map.insert(OneShot(x.to_string()), ());
        }
        map
    });
}

#[bench]
fn insert_orderedmap_string_10_000(b: &mut Bencher) {
    let c = 10_000;
    b.iter(|| {
        let mut map = IndexMap::with_capacity(c);
        for x in 0..c {
            map.insert(x.to_string(), ());
        }
        map
    });
}

#[bench]
fn lookup_hashmap_10_000_exist_string(b: &mut Bencher) {
    let c = 10_000;
    let mut map = HashMap::with_capacity(c);
    let keys = shuffled_keys(0..c);
    for &key in &keys {
        map.insert(key.to_string(), 1);
    }
    let lookups = (5000..c).map(|x| x.to_string()).collect::<Vec<_>>();
    b.iter(|| {
        let mut found = 0;
        for key in &lookups {
            found += map.get(key).is_some() as i32;
        }
        found
    });
}

#[bench]
fn lookup_hashmap_10_000_exist_string_oneshot(b: &mut Bencher) {
    let c = 10_000;
    let mut map = HashMap::with_capacity(c);
    let keys = shuffled_keys(0..c);
    for &key in &keys {
        map.insert(OneShot(key.to_string()), 1);
    }
    let lookups = (5000..c)
        .map(|x| OneShot(x.to_string()))
        .collect::<Vec<_>>();
    b.iter(|| {
        let mut found = 0;
        for key in &lookups {
            found += map.get(key).is_some() as i32;
        }
        found
    });
}

#[bench]
fn lookup_ordermap_10_000_exist_string(b: &mut Bencher) {
    let c = 10_000;
    let mut map = IndexMap::with_capacity(c);
    let keys = shuffled_keys(0..c);
    for &key in &keys {
        map.insert(key.to_string(), 1);
    }
    let lookups = (5000..c).map(|x| x.to_string()).collect::<Vec<_>>();
    b.iter(|| {
        let mut found = 0;
        for key in &lookups {
            found += map.get(key).is_some() as i32;
        }
        found
    });
}

#[bench]
fn lookup_ordermap_10_000_exist_string_oneshot(b: &mut Bencher) {
    let c = 10_000;
    let mut map = IndexMap::with_capacity(c);
    let keys = shuffled_keys(0..c);
    for &key in &keys {
        map.insert(OneShot(key.to_string()), 1);
    }
    let lookups = (5000..c)
        .map(|x| OneShot(x.to_string()))
        .collect::<Vec<_>>();
    b.iter(|| {
        let mut found = 0;
        for key in &lookups {
            found += map.get(key).is_some() as i32;
        }
        found
    });
}
