build:
    tup

exec year day: build
    ./build/{{year}}/day{{day}}/day{{day}} < {{year}}/day{{day}}/input.txt
