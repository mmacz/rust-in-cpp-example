#include <iostream>
#include "rust_lib.h"

int main(int argc, char **argv)
{
    hello_from_rust();

    Rectangle rect;
    rect.width = 10;
    rect.height = 15;
    std::cout << "Area of Rectangle calculated in Rust: " << calculate_area(&rect) << std::endl;

    return 0;
}