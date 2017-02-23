package ex01;

object RecursiveCall extends App {
    var level = 0;  // 現在の再帰レベル
    val indicator = "- ";  //

    // 再帰トレース
    def trace[T](fname: String, arg: Any*)(body: => T): T = {
        val args = arg.mkString(",");
        println((indicator*level) + level + ":" + fname + "(" + args + ")");
        level += 1;
        val ret = body;
        level -= 1;
        println((indicator*level) + level + ":" + fname + "=" + ret);
        ret;
    }

    def fact(x: Int): Int = {  // factを再帰トレース
        trace("fact", x) {  // 使用法: trace(関数名, 引数, 引数...) {処理}
            if (x==1) 1
            else x * fact(x - 1)
        }
    }

    println("階差=" + fact(5))
}
