#![feature(pattern)]
use std::str::pattern::{Pattern, ReverseSearcher};

trait Split2 {
    fn split2<'a, P>(&'a self, pat: P) -> Option<(&str, &str)>
    where P: Pattern<'a>;
}

trait RSplit2 {
    fn rsplit2<'a, P>(&'a self, pat: P) -> Option<(&str, &str)>
    where P: Pattern<'a>,
        P::Searcher: ReverseSearcher<'a>;
}

impl Split2 for &str {
    fn split2<'a, P>(&'a self, pat: P) -> Option<(&str, &str)>
    where P: Pattern<'a>
    {
        let mut iter = self.splitn(2, pat);
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            Some((a,b))
        } else {
            None
        }
    }
}

impl RSplit2 for &str {
    fn rsplit2<'a, P>(&'a self, pat: P) -> Option<(&str, &str)>
    where P: Pattern<'a>,
        P::Searcher: ReverseSearcher<'a>
    {
        let mut iter = self.rsplitn(2, pat);
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            Some((a,b))
        } else {
            None
        }
    }
}

fn main() {
    let url = "https://foo.bar/zot?boop=beep";
    if let Some((a, rest)) = url.split2(':') {
        println!("a = {}    the rest = {}", a, rest);
    }
    if let Some((a, rest)) = url.split2("://") {
        println!("a = {}    the rest = {}", a, rest);
    }
    if let Some((a, rest)) = url.split2('@') {  // this is false
        println!("a = {}    the rest = {}", a, rest);
    }
    if let Some((rest, a)) = url.rsplit2('=') {
        println!("the rest = {}    a = {}", a, rest);
    }
    if let Some((a, rest)) = "foo:".split2(':') {
        println!("a = {}    the rest = {}", a, rest);
    }
    if let Some((a, rest)) = ":foo".split2(':') {
        println!("a = {}    the rest = {}", a, rest);
    }
    if let Some((a, rest)) = ":".split2(':') {
        println!("a = {}    the rest = {}", a, rest);
    }
}
