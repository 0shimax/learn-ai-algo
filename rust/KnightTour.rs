#[derive(Clone, Debug)]
struct Knights {
    N: usize,
    bd: Vec<Vec<usize>>,
    pat: Vec<(usize, usize)>,
}

impl Knights {
    fn new(n: usize) -> Knights {
        Knights {
            N: n,
            bd: vec![vec![0; n]; n],
            pat: ((1,2), (2,1)).iter().map(
                    |t| {for b in (1, -1) {
                            for c in (1, -1) {
                                (t.0*b, t.1*c)
                            }
                        }
                    }
            ).collect::<Vec<(usize, usize)>>(),
        }
    }

    fn knight(&mut self, r: usize, c: usize, cnt: usize, route: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        if r>=0 && c>=0 && r<N && c<n && self.bd[r][c]==0 {
            self.bd[r][c] = cnt;

            if cnt == N*N { return route.insert(0, (r,c)) }
            for p in pat {
                let rt = knight(r+p0, c+p1, cnt+1, route.insert(0, (r,c)));
                if rt != None { return rt }
            }
            self.bd[r][c] = 0
        }
        None
    }

    fn start(&mut self, r: usize, c: usize) {
        println!("{}", self.knight(r,c))
        let str_bd = self.bd.iter()
                .map(|row| {
                        let tmp_bd = row.iter()
                                        .fold("", |s, cell| [s, cell.to_string()].concat(" "));
                        [tmp_bd, "\n"]
                    })
        println!("{}", str_bd)
    }
}

fn main() {
    let knight = Knights::new(5);
    knight.start(0, 0);
}
