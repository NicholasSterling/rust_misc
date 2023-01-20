struct Person<'a> {
    name: &'static str,
    children: Vec<&'a Person<'a>>,
}

struct PersonIter<'a> {
    person: &'a Person<'a>,
    next: &'a Person<'a>,
    iter_stack: Vec<std::slice::Iter<'a, &'a Person<'a>>>,
}
impl<'a> Person<'a> {
    fn iter(&self) -> PersonIter {
        PersonIter {
            person: self,
            next: &self,
            iter_stack: vec![vec![self].iter()]
        }
    }
}

impl<'a> Iterator for PersonIter<'a> {
    type Item = &'a Person<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mut iter) = self.iter_stack.pop() {
                let opt_p: Option<&&'a Person> = iter.next();
                if let Some(&p) = opt_p {
                    if !p.children.is_empty() {
                        self.iter_stack.push(p.children.iter());
                    }
                    return Some(p);
                } else {
                    self.iter_stack.pop();
                }
            } else {  // iter_stack is empty
                return None;
            }
        }
    }
}

fn main() {
    //let mut iter_stack: Vec<std::slice::Iter<Person>> = Vec::new();
    //let v = vec![Person {}, Person {}];
    //iter_stack.push(v.iter());
    let c1 = Person { name: "c1", children: vec![] };
    let c2 = Person { name: "c2", children: vec![] };
    let p1 = Person { name: "p1", children: vec![&c1, &c2] };
    for p in p1.iter() {
        println!("{}", &p.name);
    }
}