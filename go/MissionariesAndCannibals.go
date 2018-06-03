package main

import "fmt"

type State [][]string
type Op []string
// type MissionariesAndCannibals struct{
//     var State State
//     var Op Op
//     var opAll []Op{nil}
// }

func move(from []string, to []string, op Op) State {
    var from1 = difference(from, op)  // 乗船者を取り除いた残り
    if (len(from1) == len(from) - len(op)) {
        return append(from1, append(op, to...))
    } else {
        return append(from, to)
    }
}

// difference returns the elements in a that aren't in b
func difference(a, b []string) []string {
    mb := map[string]bool{}
    for _, x := range b {
        mb[x] = true
    }
    ab := []string{}
    for _, x := range a {
        if _, ok := mb[x]; !ok {
            ab = append(ab, x)
        }
    }
    return ab
}

func solve(st State, ops []Op, boat int, history []State) []State {  // 移動記録を返す
    // もう試すパターンがないので失敗
    if (ops==nil) {
        return nil
    // 移動記録作成（方向、新たな状態）
    } else if (ops==append(op, opTail)) {
        if (st==append(l, r)) {
            if (boat == -1) {
                dir, stNew = []string{'>'}, move(l, r, op)
            } else {
                dir, stNew = []string{'>'}, move(r, l, op)
            }

            if (goal(stNew)) {
                return append(append(op, dir, stNew), history)  // ゴールなら成功
            } else if (stNew==st || !safe(stNew) ||  // 無変化と移動不可は失敗
                     history[-1:]==append(dir, stNew)) {  // 過去状態に戻ると失敗
                return solve(st, opTail, boat, history)  // 残りの操作を試す
            } else {  // 移動成功
                var ret = solve(stNew, opAll, -boat,
                                append(append(op, dir, stNew), history))  //新たな状態から進める
                if (ret != Nil) {
                    ret
                } else {
                    solve(st, opTail, boat, history)  // 失敗なら残り操作試す
                }
            }
        }
    }
}

func goal(st State) {  // この移動結果はゴールか？
    return len(st) == 0  // 左岸が空
}

func safe(st State) {  // この移動結果は安全な状態か？
    cnt_s := 0
    cnt_m := 0
    for _, x:= range(st) {
        cnt_s += count_map('s', x)
        cnt_m += count_map('m', x)
    }
    return cnt_s == 0 || cnt_s >= cnt_m
}

func count_map(target string, s []string) int {
    cnt := 0
    for _, x := range s {
        if x == target {
            cnt += 1
        }
    }
    return cnt
}

func start() {
    opAll := [][]string  // 乗船パターン
    opAll[0] = []string{'s', 's'}
    opAll[1] = []string{'s', 's'}
    opAll[2] = []string{'m', 'm'}
    opAll[3] = []string{'s'}
    opAll[3] = []string{'m'}
    st := [][]string
    st[0] = []string{'s', 's', 's', 'm', 'm', 'm'}  // 岸の初期状態
    history := [][]string
    history[0] = []string{nil, '<', 's', 's', 's', 'm', 'm', 'm'}  // 移動記録初期状態
    var solution = solve(st, opAll, -1, history)  // 問題解決
    fmt.Println("移動者\t\t移動方向\t\t結果状態（左岸）\t結果状態（右岸）")
    for _, x := range(reverse(solution)) {
        fmt.Sprintf("%v", x)  // 移動記録表示
    }
}

func reverse(input [][]string) [][]string {
    if len(input) == 0 {
        return input
    }
    return append(reverseInts(input[1:]), input[0])
}


func main() {
    start()
}
