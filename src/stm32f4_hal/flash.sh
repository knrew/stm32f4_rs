#!bin/sh

/usr/local/openocd/bin/openocd -c "tcl_port disabled" -s /usr/local/openocd/share/openocd/scripts -c "gdb_port 3333" -c "telnet_port 4444" -f board/st_nucleo_f4.cfg -c "program \"/home/ryunosuke/Documents/rust_projects/poyo/src/stm32f4_hal/build/stm32f4_hal.elf\"" -c "init;reset init;reset run;exit;"
