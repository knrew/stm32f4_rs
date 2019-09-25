//extern crate cc;

fn fetch_file_names(folder_path: &str) -> Result<Vec<String>, String> {
    let mut names: Vec<String> = Vec::new();

    let dirs = match std::fs::read_dir(folder_path) {
        Ok(o) => o,
        Err(e) => return Err(format!("{}", e)),
    };
    for d in dirs {
        let s = match d {
            Ok(o) => o,
            Err(e) => return Err(format!("{}", e)),
        };
        let s = format!("{}", s.path().display());
        names.push(s);
    }

    Ok(names)
}

fn main() {
    let out_dir: &str = &std::env::var("OUT_DIR").expect("cannot get OUT DIR.");
    let current_dir: &str = &std::env::current_dir().expect("failed to get current dir.").display().to_string();
    let hal_dir: &str = &format!("{}/src/stm32f4_hal/", current_dir);
    let objects_dir: &str = &format!("{}/CMakeFiles/stm32f4_hal.elf.dir/", out_dir);

    std::process::Command::new("cmake")
        .arg(hal_dir)
        .current_dir(out_dir)
        .stdout(std::process::Stdio::inherit())
        .status().expect("failed to cmake.");

    std::process::Command::new("make")
        .current_dir(&out_dir)
        .stdout(std::process::Stdio::inherit())
        .status().expect("failed to make.");

    let src_objects = fetch_file_names(&format!("{}/Src/", objects_dir)).unwrap();
    let driver_objects = fetch_file_names(&format!("{}/Drivers/STM32F4xx_HAL_Driver/Src", objects_dir)).unwrap();
    let startup_object = format!("{}/startup/startup_stm32f411xe.s.obj", objects_dir);

    std::process::Command::new("arm-none-eabi-ar")
        .args(&["curs", "libhal.a"])
        .args(&driver_objects)
        .args(&src_objects)
        .arg(&startup_object)
        .current_dir(out_dir)
        .status().unwrap();

    for e in src_objects.iter() {
        std::process::Command::new("cp")
            .arg(e)
            .arg(out_dir)
            .status().unwrap();
    }
    for e in driver_objects.iter() {
        std::process::Command::new("cp")
            .arg(e)
            .arg(out_dir)
            .status().unwrap();
    }
    std::process::Command::new("cp")
        .arg(startup_object)
        .arg(out_dir)
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hal");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/stm32f4_hal/Src/main.c");

//    println!("cargo:rerun-if-changed=src/stm32f4_hal/STM32F411RETx_FLASH.ld");
//    println!("cargo:rerun-if-changed=src/stm32f4_hal/startup/startup_stm32f411xe.s");

}

//let inc_dirs = [
//        &format!("-I{}/Drivers/CMSIS/Device/ST/STM32F4xx/Include", hal_dir),
//        &format!("-I{}/Drivers/CMSIS/Include", hal_dir),
//        &format!("-I{}/Drivers/STM32F4xx_HAL_Driver/Inc", hal_dir),
//        &format!("-I{}/Inc", hal_dir),
//    ];

//    let src = fetch_file_names(&format!("{}/Src/", hal_dir)).expect("");
//    let drivers = fetch_file_names(&format!("{}/Drivers/", hal_dir)).expect("");
//    let startup = fetch_file_names(&format!("{}/startup/", hal_dir)).expect("");

//    let definitions = [
//        "-D__weak=__attribute__((weak))",
//        "-D__packed=__attribute__((__packed__))",
//        "-DUSE_HAL_DRIVER",
//        "-DSTM32F411xE"
//    ];

//    let compile_flags = "-mcpu=cortex-m4 ${FPU_FLAGS} -mthumb -mthumb-interwork -ffunction-sections -fdata-sections -g -fno-common -fmessage-length=0 -specs=nosys.specs -specs=nano.specs";
