#include <iostream>
#include "Body.hpp"

class RecursiveCall{
private:
    Body *body;
    const std::string INDICATOR;
    int n_repeat;

public:
    RecursiveCall(int repeat);
    void print_trace();
};
