#include <iostream>
#include "Body.hpp"
using namespace std;

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
