// http://blog.muuny-blue.info/115c51eb37365df2d4f4e2482b964822.html
// http://qiita.com/yasuyuky/items/8894f731da9a4e8cac4c
// http://stackoverflow.com/questions/38812874/no-method-found-for-type-t-in-the-current-scope-when-wrapping-a-type

trait Body: Sized {
    fn new() -> Self;

    fn pre_trace(&self, fname: &str, indicator: &str, x: &usize) {
        let args: String = x.to_string();
        println!("{}",
            (vec![indicator; *x]).join("")
                + args.as_str() + ":" + fname + "(" + args.as_str() + ")");
    }

    fn post_trace(&self, fname: &str, ret: &usize, indicator: &str, x: &usize) {
        let args: String = x.to_string();
        println!("{}",
            (vec![indicator; *x]).join("")
                + args.as_str() + ":" + fname + "=" + ret.to_string().as_str());
    }

    fn fact(&self, x: usize, indicator: &str) -> usize {
        &self.pre_trace("fact", indicator, &x);
        let ret: usize;
        if x==1 {
            ret = 1;
        } else {
            ret = x * &self.fact(x - 1, indicator);
        }
        &self.post_trace("fact", &ret, indicator, &x);
        ret
    }
}

#[derive(Clone, Debug)]
struct B;

impl Body for B {
    fn new() -> B {
        B
    }
}

#[derive(Clone, Debug)]
struct RecursiveCall<T:Body>{
    indicator: String,
    n_repeat: usize,
    body: T,
}

impl<T: ?Sized> RecursiveCall<T>
    where T:Body
{
    fn new(n_repeat: usize) -> RecursiveCall<T> {
        RecursiveCall {
            indicator: "- ".to_string(),
            n_repeat: n_repeat,
            body: <T as Body>::new(),
        }
    }

    fn print_trace(&mut self) {
        let ret = &self.body.fact(self.n_repeat, &self.indicator);

        println!("Difference={}", &ret.to_string().as_str());
    }
}

fn main(){
    let mut rc = RecursiveCall::<B>::new(5);
    rc.print_trace();
}
