package ex02

class KnightsTour(n: Int) {
    val bd = Array.fill(n,n)(0)  // chess board(n*n)
    val pat = for(a <- List((1,2), (2,1));  // movin pattern of knight
                b <- List(1,-1); c <- List(1,-1)) yield (a._1*b, a._2*c)

    def knight(r: Int, c: Int, cnt: Int=1, route: List[(Int, Int)]=Nil): List[(Int, Int)] = {
        // println("r, c: %d, %d".format(r, c))
        // println("route:")
        // println(route)

        // try to move to (r, c)
        if (r>=0 && c>=0 && r<n && c<n && bd(r)(c)==0) {
            bd(r)(c) = cnt  // write

            if (cnt == n*n) return (r,c)::route  // reach final position. return result.
            for (p<-pat) {
                val rt = knight(r+p._1, c+p._2, cnt+1, (r,c)::route)  // move to next
                if (rt != Nil) return rt  // success of searching route. return result.
            }
            bd(r)(c) = 0  // roll back to 0.
        }
        Nil  // return Nil, because of failing.
    }

    def start(r: Int, c: Int) {
        println(knight(r,c))
        println(bd.map(_.map("%02d ".format(_)).mkString).mkString("\n"))
    }
}

object KnightsTourApp extends App {
    new KnightsTour(5).start(0,0)  // board size, initial location.
}
