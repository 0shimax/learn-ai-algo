#[derive(Clone, Debug)]
struct Queens {
    n_queens: usize,
    board: Vec<Vec<usize>>,
}

impl Queens {
    fn new(n: usize) -> Queens {
        Queens {
            n_queens: n,
            board: vec![vec![0; n]; n],
        }
    }

    fn print_matrix(&self) {
        print!("\n");
        for i_r_b in 0..self.n_queens {
            for i_c_b in 0..self.n_queens {
                print!("{}\t", &self.board[i_r_b][i_c_b])
            }
            print!("\n");
        }
        print!("\n");
    }

    fn get_marked_col(&self, row: usize) -> usize {
        let r_b = &self.board[row].clone();
        for (i, b) in r_b.iter().enumerate() {
            if b == &1 { return i }
        }
        return u8::max_value() as usize
    }

    fn feasible(&self, row: usize, col: &usize) -> bool {
        for i in 0..self.n_queens {
            let tcol: i32 = *&self.get_marked_col(i).clone() as i32;
            let c: i32 = col.clone( as i32;
            let r: i32 = row.clone() as i32;

            if &c == &tcol || ((&r - (i as i32)) as i32).abs() == (&c - &tcol).abs() {
                return false
            }
        }
        return true
    }

    fn nqueen(&mut self, row: &usize) {
        if row < &self.n_queens {
            for i in 0..self.n_queens {
                if self.feasible(*row, &i) {
                    self.board[*row][i] = 1;
                    &self.nqueen(&(row + 1));
                    self.board[*row][i] = 0;
                }
            }
        } else {
            print!("\nThe solution is:- ");
            &self.print_matrix();
        }
    }

}


fn main() {
    print!("\nEnter the no. of queens:- ");

    let mut q = Queens::new(8);

    q.nqueen(&0);
}
