build:
    tup

exec15 day: build
    ./build/2015/day{{day}}/day{{day}} < 2015/day{{day}}/input.txt
