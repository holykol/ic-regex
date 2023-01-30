use blake3;
use ic_kit::prelude::*;
use regex::{Captures, Regex};
use std::collections::HashMap;

#[derive(Default)]
struct Cache {
    precompiled: HashMap<blake3::Hash, Regex>,
}

impl Cache {
    /// runs callback with precompiled pattern or compiles it in place
    fn with<F, T>(&self, pattern: String, cb: F) -> T
    where
        F: FnOnce(&Regex) -> T,
    {
        let hash = blake3::hash(pattern.as_bytes());

        match self.precompiled.get(&hash) {
            Some(r) => cb(r),
            None => {
                let re = Regex::new(&pattern).expect("failed to compile regex");
                cb(&re)
            }
        }
    }
}

#[update]
fn precompile(c: &mut Cache, patterns: Vec<String>) {
    c.precompiled.reserve(patterns.len());

    for re in patterns {
        let hash = blake3::hash(re.as_bytes());
        let compile = || Regex::new(&re).expect("failed to compile regex");
        c.precompiled.entry(hash).or_insert_with(compile);
    }
}

#[update]
fn purge_cache(cache: &mut Cache) {
    cache.precompiled.clear();
    cache.precompiled.shrink_to_fit();
}

#[query]
fn is_match(cache: &Cache, pattern: String, text: String) -> bool {
    cache.with(pattern, |re| re.is_match(&text))
}

#[query]
fn is_match_batch(cache: &Cache, pattern: String, texts: Vec<String>) -> Vec<bool> {
    cache.with(pattern, |re| {
        texts.into_iter().map(|s| re.is_match(&s)).collect()
    })
}

#[derive(Debug, PartialEq, Eq, CandidType, Serialize, Deserialize)]
struct Match {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[query]
fn captures(cache: &Cache, pattern: String, text: String) -> Vec<Option<Match>> {
    let captures = cache.with(pattern, |r| r.captures(&text));
    map_captures(captures)
}

#[query]
fn captures_batch(cache: &Cache, pattern: String, texts: Vec<String>) -> Vec<Vec<Option<Match>>> {
    cache.with(pattern, |re| {
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

#[derive(KitCanister)]
#[candid_path("regex.did")]
pub struct RegexCanister;

#[cfg(test)]
mod tests {
    use super::*;

    #[kit_test]
    async fn test_basic(replica: Replica) {
        let r = replica.add_canister(RegexCanister::anonymous());

        let pattern = "hel.";
        r.new_call("precompile")
            .with_arg(vec![pattern])
            .perform()
            .await
            .assert_ok();

        let matches = r
            .new_call("is_match")
            .with_args((pattern, "hell"))
            .perform()
            .await
            .decode_one::<bool>()
            .unwrap();

        assert!(matches);

        let matches = r
            .new_call("is_match")
            .with_args((pattern, "world"))
            .perform()
            .await
            .decode_one::<bool>()
            .unwrap();

        assert!(!matches);
    }

    #[kit_test]
    async fn test_match_batch(replica: Replica) {
        let r = replica.add_canister(RegexCanister::anonymous());

        let pattern = "^hel.$";

        let matches = r
            .new_call("is_match_batch")
            .with_args((pattern, vec!["hell", "hello", "helm"]))
            .perform()
            .await
            .decode_one::<Vec<bool>>()
            .unwrap();

        assert_eq!(matches, vec![true, false, true]);
    }

    #[kit_test]
    async fn test_captures(replica: Replica) {
        let r = replica.add_canister(RegexCanister::anonymous());

        let pattern = r"hello(\d+)?";

        let captures = r
            .new_call("captures_batch")
            .with_args((pattern, vec!["hello42", "hello"]))
            .perform()
            .await
            .decode_one::<Vec<Vec<Option<Match>>>>()
            .unwrap();

        assert_eq!(
            captures,
            vec![
                vec![
                    Some(Match {
                        text: "hello42".to_owned(),
                        start: 0,
                        end: 7,
                    }),
                    Some(Match {
                        text: "42".to_owned(),
                        start: 5,
                        end: 7,
                    })
                ],
                vec![
                    Some(Match {
                        text: "hello".to_owned(),
                        start: 0,
                        end: 5,
                    }),
                    None,
                ],
            ]
        );
    }
}
