#!/usr/bin/env python3

import time
import subprocess


def arg_parser():
    import argparse

    parser = argparse.ArgumentParser(description="Pomodoro Timer")
    parser.add_argument("-t", "--time", type=int, default=30, help="Time in minutes")
    parser.add_argument(
        "-s", "--short-break", type=int, default=5, help="Short break time in minutes"
    )
    return parser.parse_args()


class Notification:
    def __init__(self):
        self.title = ""
        self.message = ""

    def send(self):
        result = subprocess.check_output(
            [
                "notify-send",
                self.title,
                self.message,
                "-A",
                "OK",
                "-t",
                str(3600 * 1000),
                "-w",
            ]
        )


def main():
    args = arg_parser()

    work_time = args.time
    rest_time = args.short_break

    work_notification = Notification()
    work_notification.title = "Work time!"
    work_notification.message = f"Focus on your work for {work_time} minutes"

    rest_notification = Notification()
    rest_notification.title = "Short break!"
    rest_notification.message = f"Take a break for {rest_time} minutes"

    while True:
        work_notification.send()
        print(f"Focus on your work for {work_time} minutes")
        time.sleep(args.time * 60)

        rest_notification.send()
        print(f"Take a break for {rest_time} minutes")
        time.sleep(rest_time * 60)


if __name__ == "__main__":
    main()
