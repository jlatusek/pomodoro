#include <format>
#include <iostream>
#include <stdexcept>
#include <string>
#include <thread>

#include "notification.h"

int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        std::cerr << "Wrong number of arguments :((\n";
        return 1;
    }
    auto args = std::span(argv, argc);
    int timeout = 0;

    try
    {
        timeout = std::stoi(args[1]);
    }
    catch (std::invalid_argument const &ex)
    {
        std::cerr << std::format("Wrong format of input argument, expected int got: {}\n", args[1]);
        return 1;
    }

    while (true)
    {
        for (auto i = timeout; i > 0; --i)
        {
            std::cout << std::format("\rTime left: {}{}", i, std::string(32, ' ')) << std::flush;
            std::this_thread::sleep_for(std::chrono::seconds(1));
        }
        std::cout << "\n";
        send_notification("Get your eyes rest", "Take a short break for 5 minutes");
    }
    return 0;
}
