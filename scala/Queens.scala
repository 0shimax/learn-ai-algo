package ex021


class Queens(val n: Int) {
    def check(r: Int, c: Int, pat: List[(Int, Int)]): Boolean = {
        // check placements whether overlap
        pat.forall(p => c != p._2 && r-p._1 != math.abs(c-p._2))
    }

    def queen(r: Int): List[List[(Int, Int)]] = {  // placements list
        if (r==0) List(Nil)
        else for (p <- queen(r-1); c <- 1 to n if check(r, c, p))
            yield (r, c)::p
    }

    def start() {
        queen(n).foreach(pat => println(pat.map(p =>
            "+"*(p._2-1) + "Q" + "+"*(n-p._2)+"\n").mkString))
    }
}
object QueensApp extends App {
    new Queens(8).start
}
