#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub struct Atom<T: Clone> {
    v: T
}

pub type CLVoid = Atom<()>;

#[allow(dead_code)]
pub impl<T: Clone> Atom<T> {
    fn new(v: T) -> Self {
        Atom {v: v}
    }

    fn evaluate(&self) -> Self {
        self.clone()
    }
}

#[allow(unused_macros)]
pub macro_rules! atom {
    ($v:expr) => {{Atom::new($v)}};
}

#[allow(unused_macros)]
pub macro_rules! clist {
    () => {{
        atom!(Vec<Atom<_>>::new())
    }};

    ($($key:expr),* $(,)?) => {{
        let mut list: Atom<Vec<Atom<_>>> = atom!(Vec::new());

        $( list.v.push(atom!($key)); )*
        list
    }};
}

#[allow(unused_macros)]
pub macro_rules! chash {
    () => {{
        atom!(HashMap<Atom<_>, Atom<_>>::new())
    }};

    ($($key:expr => $value:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut list: Atom<HashMap<Atom<_>, Atom<_>>> = atom!(HashMap::new());
        $( list.v.insert(atom!($key), atom!($value)); )*
        list
    }};
}

#[allow(unused_macros)]
pub macro_rules! cons {
    ($left:expr, $right:expr) => {{atom!([atom!($left), atom!($right)])}};
}

#[allow(dead_code, type_alias_bounds)]
pub type CLFunc<T: Clone,R: Clone> = fn(args: &Vec<Atom<T>>) -> Atom<R>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct SEpression<T: Clone, R: Clone> {
    args: Vec<Atom<T>>,
    func: CLFunc<T, R>,
}

pub impl<T: Clone, R: Clone> SEpression<T, R> {
    fn new(cb: CLFunc<T, R>, args: Vec<Atom<T>>) -> Self {
        SEpression {func: cb, args: args}
    }

    fn evaluate(&self) -> Atom<R> {
        (self.func)(&self.args)
    }
}

pub macro_rules! sexpr {
    ( $func:expr, w args $($arg:expr),* $(,)? ) => {{
        let mut v: Vec<Atom<_>> = Vec::new();
        $( v.push($arg); )*
        SEpression::new($func, v).evaluate()
    }};
}

pub mod builtins {
    use super::*;
    use std::any::Any;
    pub fn ADD(nums: &Vec<Atom<i32>>) -> Atom<Result<i32, String>> {
        if nums.len() < 2 {return atom!(Err("Can't be less than 2".to_string()));}
        let mut total: i32 = nums[0].v;
        for n in &nums[1..] {
            total += n.v;
        }
        atom!(Ok(total))
    }
    pub fn SUB(nums: &Vec<Atom<i32>>) -> Atom<Result<i32, String>> {
        if nums.len() < 2 { return atom!(Err("Can't be less than 2".to_string())); }
        let mut total = nums[0].v;
        for n in &nums[1..] {
            total -= n.v;
        }
        atom!(Ok(total))
    }
    pub fn MUL(nums: &Vec<Atom<i32>>) -> Atom<Result<i32, String>> {
        if nums.len() < 2 { return atom!(Err("Can't be less than 2".to_string())); }
        let mut total = nums[0].v;
        for n in &nums[1..] {
            total *= n.v;
        }
        atom!(Ok(total))
    }
    pub fn DIV(nums: &Vec<Atom<i32>>) -> Atom<Result<i32, String>> {
        if nums.len() < 2 { return atom!(Err("Can't be less than 2".to_string())); }
        let mut total = nums[0].v;
        for n in &nums[1..] {
            total -= n.v;
        }
        atom!(Ok(total))
    }
    pub fn WRITE(nums: &Vec<Atom<&dyn Any>>) -> Atom<std::io::Result<()>> {
        use std::fs;
        use std::io::Result;
        if nums.len() == 2 {
            let mut f: fs::File = nums[1].v;
            let bytes: Vec<u8> = nums[0].v;
            atom!(f.write_all(&bytes)?);
        } else if nums.len() == 1 {
            let v: String = nums[0];
            print!(v);
        }
        atom!(Result::Ok(()))
    }
    pub fn FORMAT(vals: &Vec<Atom<&dyn Any>>) -> Atom<Option<String>> {
        let mut result: String = "".to_string();
        let toParse: String = vals[1].v
        let args = vals[2..];
        let mut isVerb = false;

        for ch in toParse.chars() {
            if(ch == '~') {
                isVerb = true;
                continue;
            }
            if(isVerb) {
                match ch {
                    'A' => {
                        result.push_str(vals.p);
                    }
                }
            }
        }

        if vals[0].v {
            print!(result.as_str());
            atom!(None)
        } else {
            atom!(Some(result))
        }
    }
}

fn OptionHandler<T, R>(thing: Option<T>,someCb: F, noneCb: G) -> R
where F: FnOnce(T) -> R, G: FnOnce() -> R {
    return match thing {
        Some(x) => someCb(x),
        None => noneCb()
    };
}

fn main() {}