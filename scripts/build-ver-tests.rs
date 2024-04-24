// build.rs
use std::cfg;

fn main() {
    build::main();
}

mod build {
    use std::fs;
    use std::io::{self, BufRead, Write};
    use std::path::Path;

    pub fn main() {
        write_test_files_for_version_1_70("tests/trybuild_tests_rt.rs");
        write_test_files_for_version_1_70("tests/trybuild_tests.rs");
        let original_runtime_path = "trybuild_tests/runtime";
        let new_runtime_path = "trybuild_tests/1_70/runtime";
        fs::create_dir_all(new_runtime_path).expect("Unable to create directory");

        let original_tests_path = "trybuild_tests";
        let new_tests_path = "trybuild_tests/1_70";
        fs::create_dir_all(new_tests_path).expect("Unable to create directory");

        // Copy all .rs files from original_tests_path to new_tests_path
        for (old_test_dir, new_test_dir) in [original_runtime_path, original_tests_path]
            .iter()
            .zip([new_runtime_path, new_tests_path])
        {
            for entry in fs::read_dir(old_test_dir).expect("Unable to read directory") {
                let entry = entry.expect("Unable to get entry");
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        let new_path = Path::new(new_test_dir)
                            .join(path.file_name().expect("Unable to get file name"));
                        fs::copy(&path, new_path).expect("Unable to copy file");
                    }
                }
            }
        }
    }

    fn generate_new_file_path(original_file_path: &str, suffix: &str) -> String {
        let file_name = Path::new(original_file_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let new_file_name = format!("{}_{}", file_name, suffix);
        let new_file_path = Path::new(original_file_path)
            .with_file_name(new_file_name)
            .with_extension("rs");
        new_file_path.to_str().unwrap().to_string()
    }

    fn write_test_files_for_version_1_70(original_tests_dir_file_path: &str) {
        let suffix = "1_70";
        let new_file_path = generate_new_file_path(original_tests_dir_file_path, suffix);

        let file = fs::File::open(original_tests_dir_file_path).expect("Unable to open file");
        let new_file = fs::File::create(new_file_path).expect("Unable to create file");
        let reader = io::BufReader::new(file);
        let mut writer = io::BufWriter::new(new_file);

        let first_line = "#[rustversion::stable(1.70.0)]";
        writeln!(writer, "{}", first_line).expect("Unable to write line");

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            if line == "#[rustversion::not(stable(1.70.0))]".to_owned() || line.is_empty() {
                continue;
            }
            let modified_line = line.replace(
                "trybuild_tests",
                format!("trybuild_tests/{suffix}").as_str(),
            );
            writeln!(writer, "{}", modified_line).expect("Unable to write line");
        }
    }
}
