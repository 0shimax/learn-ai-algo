#include <iostream>
// #include "RecursiveCall.hpp"
using namespace std;

class Body{
public:
    void pre_trace(string fname, string indicator, int x) {
        string s = "";
        for(int i=0;i<x;i++) s += indicator;

        cout << s << x << ":" << fname << "(" << x << ")" << endl;
    }

    void post_trace(string fname, int ret, string indicator, int x) {
        string s = "";
        for(int i=0;i<x;i++) s += indicator;

        cout << s << x << ":" << fname << "=" << ret << endl;
    }

    int fact(int x, string indicator) {
        Body::pre_trace("fact", indicator, x);
        int ret = 0;
        if (x==1) {
            ret = 1;
        } else {
            ret = x * Body::fact(x - 1, indicator);
        }
        Body::post_trace("fact", ret, indicator, x);
        return ret;
    }
};

class RecursiveCall
{
private:
    Body *body;
    const string INDICATOR;
    int n_repeat;

public:
    RecursiveCall(int repeat): body(new Body), INDICATOR("- ") {
        n_repeat = repeat;
    }
    void print_trace() {
        int ret = RecursiveCall::body->fact(
                RecursiveCall::n_repeat, RecursiveCall::INDICATOR);

        cout << "Difference=" << ret;
    }
};

int main(){
    RecursiveCall *rc = new RecursiveCall(5);
    rc->print_trace();
    return 0;
}
