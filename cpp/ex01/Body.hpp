#include <iostream>

class Body
{
public:
    int fact(int x, std::string indicator);
    void pre_trace(std::string fname, std::string indicator, int x);
    void post_trace(std::string fname, int ret, std::string indicator, int x);
};
