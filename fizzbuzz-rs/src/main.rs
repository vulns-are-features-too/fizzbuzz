use std::ops::Range;

fn main() {
    // funny magic notations :)
    let r#mod = |i| Box::new(move |x| x % i == 0);
    FizzBuzz::new()
        // use a straight closure
        .add(&|x| x % 3 == 0, "fizz")
        // or make your own Fn builder
        .add(&r#mod(5), "buzz")
        // you can generate() multiple times from a single rule set,
        // but I wanted to do a single chain for fun :)
        .generate(0..100)
        // could just use a for loop, but again, nice chain :)
        .iter()
        .for_each(|i| println!("{}", i));
}

// use whatever int type
type Int = u32;

// can also use <'static>
struct Entry<'a> {
    pub func: &'a dyn Fn(Int) -> bool,
    pub value: &'a str,
}

// why make a new struct with just a single member :)
type FizzBuzz<'a> = Vec<Entry<'a>>;

// Using Hungarian notation, but not because I write Windows stuff (I use Arch btw)
trait TFizzBuzz<'a> {
    fn add(self, func: &'a dyn Fn(Int) -> bool, value: &'a str) -> Self;
    fn generate(&self, range: Range<Int>) -> Vec<String>;
}

impl<'a> TFizzBuzz<'a> for FizzBuzz<'a> {
    fn add(mut self, func: &'a dyn Fn(Int) -> bool, value: &'a str) -> Self {
        self.push(Entry { func, value });
        self
    }

    fn generate(&self, range: Range<Int>) -> Vec<String> {
        let mut result = vec![];
        for i in range {
            let mut s = String::new();
            for f in self.iter() {
                if (f.func)(i) {
                    s.push_str(f.value);
                }
            }
            if s.is_empty() {
                s = i.to_string();
            }
            result.push(s);
        }
        result
    }
}

// It's not like I wanted to be noticed by the Haskell devs or anything... BAKA!
