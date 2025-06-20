#include <cstdio>
#include <notification.h>

#include <chrono>
#include <iostream>
#include <sys/wait.h>
#include <unistd.h>

void send_notification(const std::string &header, const std::string &msg)
{
    pid_t pid = fork();
    if (pid == 0)
    {
        // Child process
        execlp("notify-send", header.c_str(), msg.c_str(), "-a", "Pomodoro", "-t", "3600000", "-w", "-A", "ok",
               nullptr);
        std::perror("Execution of notify-send failed");
        _exit(1);
    }
    else if (pid > 0)
    {
        // Parent process
        std::cout << "Notification sent, waiting for acceptance.\n";
        auto start = std::chrono::high_resolution_clock::now();
        waitpid(pid, nullptr, 0);
        auto duration =
            std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::high_resolution_clock::now() - start);
        std::cout << std::format("Notification closed after: {} seconds\n", duration.count() / 1e6L);
    }
    else
    {
        // Fork failed
        std::perror("Creating new process failed");
    }
}
