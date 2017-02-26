#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;


class TowersOfHanoi {
private:
    const int N;

public:
    TowersOfHanoi(int n): N(n) {}

    void hanoi(int m, int from, int to, int work, vector<vector<int>> s) {
        vector<int> from_head;
        from_head.push_back(s[from][0]);

        if (m==1) {
            from_head.insert(from_head.end(), s[to].begin(), s[to].end());
            s[to].resize(from_head.size());
            s[to].swap(from_head);
            vector<int> v_tmp(s[from].begin() + 1, s[from].end());
            s[from].resize(v_tmp.size());
            s[from].swap(v_tmp);
            //
            vector<vector<int>> s_print;
            for (auto e: s) {
                vector<int> tmp;
                copy(e.begin(), e.end(), back_inserter(tmp));
                reverse(tmp.begin(), tmp.end());
                s_print.push_back(tmp);
            }

            cout << this->disp(s_print) << "\n";
        } else {
            this->hanoi(m-1, from, work, to, s);
            this->hanoi(1, from, to, work, s);
            this->hanoi(m-1, work, to, from, s);
        }
    }

    string disp(vector<vector<int> > a) {
        vector<vector<int> > all_null;
        if (a[0].empty() && a[1].empty() && a[2].empty()) {
            string ret = "";
            for (int i=0;i<this->N*2*3;i++) {
                ret += "-";
            }
            ret += "\n";
            return ret;
        } else{
            string s_disp = this->extract_tail_element(a);
            string head = this->extract_head_element(a);
            return s_disp + "\n" + head;
        }
    }

    string extract_tail_element(vector<vector<int> > a) {
        vector<vector<int> > in_disp_vec;
        for (int i=0; i<a.size(); i++) {
            if (a[i].empty()) {
                in_disp_vec.push_back({});
            } else {
                vector<int> v_tmp(a[i].begin() + 1, a[i].end());
                in_disp_vec.push_back(v_tmp);
            }
        }
        return this->disp(in_disp_vec);
    }

    string extract_head_element(vector<vector<int> > a) {
        int n = this->N;
        vector<int> res;
        for(vector<int> x : a) {
            if (x.empty()) {
                res.push_back(0);
            } else {
                res.push_back(x.front());
            }
        }

        vector<string> res2;
        for(int x : res){
            string s = "";
            for(int i=0;i<n-x;i++) s += " ";
            for(int i=0;i<x;i++) s += "■■";
            for(int i=0;i<n-x;i++) s += " ";
            s += " ";
            res2.push_back(s);
        }
        return this->join(res2);
    }

    string join(vector<string> wa) {
        string s_ret = "";
        for (string s: wa) {
            s_ret += s;
        }
        return s_ret;
    }


    void start(void) {
        vector<int> work(this->N);
        int i = 1;
        generate(work.begin(), work.end(), [&i]() -> int {return i++;});
        vector<vector<int> > vec(3, vector<int>(this->N));
        vec[0] = work;
        this->hanoi(this->N, 0, 2, 1, vec);
    }
};

int main(void) {
    TowersOfHanoi *toh = new TowersOfHanoi(4);
    toh->start();
    return 0;
}
