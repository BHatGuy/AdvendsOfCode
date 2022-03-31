FQBN        ?= arduino:avr:uno
BUILD_DIR  := build/

SRC        := $(wildcard *.ino)
CPP        := $(wildcard *.cpp) 
HDRS       := $(wildcard *.hpp)
BIN        := $(BUILD_DIR)/$(SRC).bin
ELF        := $(BUILD_DIR)/$(SRC).elf
SERIAL_DEV = /dev/ttyUSB0

all: $(ELF) upload
.PHONY: all

compile: $(ELF)
.PHONY: compile

$(ELF): $(SRC) $(HDRS) $(CPP)
	arduino-cli compile -b $(FQBN) --build-path $(BUILD_DIR)

upload: 
	arduino-cli upload -b $(FQBN) --input-dir $(BUILD_DIR) -p $(SERIAL_DEV)

serial:
	picocom $(SERIAL_DEV)
.PHONY: serial

clean:
	rm -r $(BUILD_DIR)