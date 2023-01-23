#![feature(local_key_cell_methods)]

use blake3;
use candid::{CandidType, Deserialize};
use ic_cdk::*;
use regex::{Captures, Regex};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// compiled regexes by their blake3 hash
    pub static PRECOMPILED_REGEXES: RefCell<HashMap<blake3::Hash, Regex>> = Default::default();
}

#[update]
fn precompile(patterns: Vec<String>) {
    PRECOMPILED_REGEXES.with_borrow_mut(|c| {
        c.reserve(patterns.len());

        for re in patterns {
            let hash = blake3::hash(re.as_bytes());
            let compile = || Regex::new(&re).expect("failed to compile regex");
            c.entry(hash).or_insert_with(compile);
        }
    })
}

#[update]
fn purge_cache() {
    PRECOMPILED_REGEXES.with_borrow_mut(|c| {
        c.clear();
        c.shrink_to_fit();
    })
}

#[query]
fn is_match(pattern: String, text: String) -> bool {
    with_regex(pattern, |re| re.is_match(&text))
}

#[query]
fn batch_is_match(pattern: String, texts: Vec<String>) -> Vec<bool> {
    with_regex(pattern, |re| {
        texts.into_iter().map(|s| re.is_match(&s)).collect()
    })
}

#[derive(CandidType, Deserialize)]
struct Match {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[query]
fn captures(pattern: String, text: String) -> Vec<Option<Match>> {
    let captures = with_regex(pattern, |r| r.captures(&text));
    map_captures(captures)
}

#[query]
fn batch_captures(pattern: String, texts: Vec<String>) -> Vec<Vec<Option<Match>>> {
    with_regex(pattern, |re| {
        texts
            .into_iter()
            .map(|s| map_captures(re.captures(&s)))
            .collect()
    })
}

fn map_captures(captures: Option<Captures>) -> Vec<Option<Match>> {
    match captures {
        Some(c) => c.iter().map(|m| m.map(map_match)).collect(),
        None => Vec::new(),
    }
}

fn map_match<'a>(m: regex::Match<'a>) -> Match {
    Match {
        text: m.as_str().to_owned(),
        start: m.start(),
        end: m.end(),
    }
}

// fun fact: this function took an embarrasing amount of time to figure out
fn with_regex<F, T>(pattern: String, cb: F) -> T
where
    F: FnOnce(&Regex) -> T,
{
    let hash = blake3::hash(pattern.as_bytes());

    PRECOMPILED_REGEXES.with_borrow(|h| match h.get(&hash) {
        Some(r) => cb(r),
        None => {
            let re = Regex::new(&pattern).expect("failed to compile regex");
            cb(&re)
        }
    })
}
