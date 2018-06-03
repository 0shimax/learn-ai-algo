package ex03

// 宣教師とモンスターの川渡り問題クラス
class MissionariesAndCannibals {
    type State = List[List[Char]]  // 現在の状態のState型の定義
    type Op = List[Char]  // 操作パターンのOp型の定義
    var opAll: List[Op] = Nil  // 可能な乗船パターン

    def move(from: List[Char], to: List[Char], op: Op): State = {
        val from1 = from.diff(op)  // 乗船者を取り除いた残り
        if (from1.length == from.length - op.length)
            // `:::`はList同士の結合演算子
            List(from1, op:::to)
        else List(from, to)
    }

    def solve(st: State, ops: List[Op], boat: Int,
              history: List[State]): List[State] = {  // 移動記録を返す
        ops match {
            case Nil => Nil  // もう試すパターンがないので失敗
            case op::opTail => {
                val (dir, stNew) = st match {  // 移動記録作成（方向、新たな状態）
                    case List(l, r) =>
                        if (boat == -1) (List('>'), move(l, r, op).map(_.sorted))
                        else (List('<'), move(r, l, op).reverse.map(_.sorted))
                }

                // `::`はListへの要素の追加
                if (goal(stNew)) (op::dir::stNew)::history  // ゴールなら成功
                else if (stNew==st || !safe(stNew) ||  // 無変化と移動不可は失敗
                         history.exists(_.tail==dir::stNew))  // 過去状態に戻ると失敗
                    solve(st, opTail, boat, history)  // 残りの操作を試す
                else {  // 移動成功
                    var ret = solve(stNew, opAll, -boat,
                                    (op::dir::stNew)::history)  //新たな状態から進める
                    if (ret != Nil) ret
                    else solve(st, opTail, boat, history)  // 失敗なら残り操作試す
                }
            }
        }
    }

    def goal(st: State) = {  // この移動結果はゴールか？
        st.head == Nil  // 左岸が空
    }

    def safe(st: State) = {  // この移動結果は安全な状態か？
        st.forall(x => x.count(_ == 's') == 0 ||
                  x.count(_ == 's') >= x.count(_ == 'm'))
    }

    def start() {
        opAll = List(List('s', 's'), List('s', 'm'), List('m', 'm'),
                     List('s'), List('m')).map(_.sorted)  // 乗船パターン
        val st = List(List('s', 's', 's', 'm', 'm', 'm'),
                      List()).map(_.sorted)  // 岸の初期状態
        val history = List(Nil::List('<')::st)  // 移動記録初期状態
        val solution = solve(st, opAll, -1, history)  // 問題解決
        println("移動者\t\t移動方向\t\t結果状態（左岸）\t結果状態（右岸）")
        solution.reverse.foreach(x =>
            println(x.map(_.mkString).mkString("\t\t")))  // 移動記録表示
    }
}

object MissionariesAndCannibalsApp extends App {
    new MissionariesAndCannibals().start
}
