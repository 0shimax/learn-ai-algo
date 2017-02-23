#include <iostream>
#include <string>
using namespace std;

class Body{
public:
    void pre_trace(string fname, string indicator, int x) {
        char cstr[x];
        strncpy(cstr, indicator.c_str(), x);

        cout << cstr << x << ":" << fname << "(" << x << ")" << endl;
    }

    void post_trace(string fname, int ret, string indicator, int x) {
        char cstr[x];
        strncpy(cstr, indicator.c_str(), x);

        cout << cstr << x << ":" << fname << "=" << ret << endl;
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
