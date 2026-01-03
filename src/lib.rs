pub mod utils {
    use std::env;

    pub fn load_data() -> Result<String, std::io::Error> {
        let args: Vec<String> = env::args().collect();

        let path: String;
        if args.len() > 1 {
            path = args[1].clone();
        } else {
            let t = env::current_exe().unwrap();
            let exe = t.file_stem().unwrap().to_str().unwrap();
            path = format!("input/{exe}/input.txt");
        }

        println!("[INFO] Loading content of \"{path}\"");
        return std::fs::read_to_string(path);
    }
}
