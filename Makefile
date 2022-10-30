PROJ    := test
TARGET  := $(PROJ)

.PHONY : build clean

build: $(TARGET).gba

$(TARGET).gba :
	@cargo build --release
	@arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/agb_play $@
	@gbafix $@

clean : 
	@rm -fv *.gba
	@rm -fv *.sav
