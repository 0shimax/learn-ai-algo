#include <algorithm>
#include<iostream>
#include<vector>
using namespace std;


class Queens {
public:
    const int N;
    vector<vector<int>> Board;

public:
    Queens(int n): N(n), Board(n, vector<int>(n)) {
        vector<int> work(n);
        generate(work.begin(), work.end(), []() -> int {return 0;});
        for (auto i=0; i<n; ++i) {
            Board[i] = work;
        }
    }

    void printMatrix() {
       cout << "\n";
       for (auto i = 0; i < this->N; i++) {
          for (auto j = 0; j < this->N; j++)
             cout << this->Board[i][j] << "\t";
          cout << "\n\n";
       }
    }

    int getMarkedCol(int row) {
       for (auto i = 0; i < this->N; i++) {
          if (this->Board[row][i] == 1) {
             return i;
          }
        }
        return 0x7f800000;
    }

    int feasible(int row, int col) {
        for (auto i = 0; i < this->N; i++) {
            int tcol = this->getMarkedCol(i);
            if (col == tcol || abs(row - i) == abs(col - tcol))
                return 0;
        }
        return 1;
    }

    void nqueen(int row) {
        if (row < this->N) {
            for (auto i = 0; i < this->N; i++) {
                if (this->feasible(row, i)) {
                    this->Board[row][i] = 1;
                    this->nqueen(row + 1);
                    this->Board[row][i] = 0;
                 }
            }
        } else {
            cout << "\nThe solution is:- ";
            this->printMatrix();
       }
    }
};

int main(void) {
    int n = 8;
    Queens *q = new Queens(n);

    q->nqueen(0);
    return 0;
}
