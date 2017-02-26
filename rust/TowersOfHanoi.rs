use std::iter::repeat;


struct TowersOfHanoi {
    _n: usize,
}

impl TowersOfHanoi {
    fn new (n: usize) -> TowersOfHanoi {
        TowersOfHanoi {
            _n: n,
        }
    }

    fn hanoi(&self, m: &usize, from: usize, to: usize, work: usize, s:&mut Vec<Vec<usize>>) {
        if m.eq(&1) {
            // println!("pre:{:?}", &s[from]);
            // don't use swap_remove, because of swap_remove behave strange.
            let head: usize = *s[from].first().unwrap();
            s[from].remove(0);
            // println!("post:{:?}", &s[from]);
            s[to].insert(0, head);  //v.slice(1, 3);
            // println!("post:{:?}", &s);
            let disp_vec = s.iter()
                                .map(|v| {
                                    let mut tmp = v.clone();
                                    tmp.reverse();
                                    tmp})
                                .collect::<Vec<Vec<usize>>>();
            println!("{}", &self.disp(disp_vec.clone()));
        } else {
            &self.hanoi(&(m-1), from, work, to, s);
            &self.hanoi(&1, from, to, work, s);
            &self.hanoi(&(m-1), work, to, from, s);
        }
    }

    fn disp(&self, a: Vec<Vec<usize>>) -> String {
        // println!("a:{:?}", &a);
        if a[0].is_empty() && a[1].is_empty() && a[2].is_empty() {
            // println!("disp\n");
            // println!("{:?}", a);
            return (vec!["-"; &self._n*2*3]).join("") + "\n"
        } else {
            let str_disp = a.iter()
                            .map(|x|
                                Some(x).and_then(
                                    |tmp| if tmp.len()==0 {Some(vec![])}
                                        // skip(k) skips k elements, not k-th element.
                                        else {Some(x.iter().skip(1).map(|e| *e).collect::<Vec<usize>>())}
                                ).unwrap())
                            .collect::<Vec<Vec<usize>>>();
            // println!("str_disp:{:?}", str_disp);

            let tail = &self.disp(str_disp);

            let head = a.iter()
                        .map(|x| {
                            Some(x).and_then(|tmp|
                                if tmp.len()==0 {Some(0)}
                                else {Some(*tmp.first().unwrap())}).unwrap()})
                        .map(|x| {
                            let s1 = (vec![" "; &self._n-x]).join("");
                            let s2 = (vec!["■■"; x]).join("");
                            let s3 = (vec![" "; &self._n-x]).join("");
                            return s1 + s2.as_str() + s3.as_str() + ""})
                        .collect::<Vec<String>>();
            return tail.clone() + "\n" + (head.join("").as_str())
        }
    }

    fn start(&self){
        let mut s: Vec<Vec<usize>> = repeat(vec![]).take(3).collect();
        s[0] = (1..&self._n+1).collect();
        &self.hanoi(&self._n, 0, 2, 1, &mut s);
    }
}


fn main() {
    let toh = TowersOfHanoi::new(4);
    toh.start();
}
