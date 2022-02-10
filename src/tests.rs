#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use crate::main;

    #[test]
    fn check() {
        let updll = "UnityPlayer.dll";
        let upmdll = "UnityPlayer_Mono.dll";
        let upbdll = "UnityPlayer.bak.dll";
        let files_names = [updll, upmdll, upbdll];
        for file_name in files_names {
            fs::remove_file(file_name).ok();
            let mut file = fs::OpenOptions::new().write(true).create(true).open(file_name).unwrap();
            file.write_all(file_name.as_bytes()).expect("Unable to write file");
            println!("made file {}", file_name)
        }
        main()
    }
}