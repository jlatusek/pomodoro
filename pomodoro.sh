#!/bin/bash

# Pomodoro timer

# Set the work and rest times
work_time=30
rest_time=5

# Start the timer
while true; do
    # Work time
    notify-send "Work time!" "Focus on your work for $work_time minutes" -w -t 3600000
    sleep $((work_time * 60))

    # Rest time
    notify-send "Rest time!" "Take a break for $rest_time minutes" -w -t 3600000
    sleep $((rest_time * 60))
done
