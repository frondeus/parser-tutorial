use std::fmt::Display;
use std::io::Write;
use std::path::{PathBuf, Path};
use std::fs::{File, read_to_string, create_dir_all, remove_file};
use std::env::{current_dir, set_current_dir};

pub fn snap(actual_result: impl Display, file: &str, test_case_name: &str) {
    let actual_result = format!("{}", actual_result);
    println!("{}", actual_result);

    let file_path = PathBuf::from(file);

    goto_workdir(&file_path);

    let mut dir_path = file_path.clone();
    dir_path.set_extension("");
    let file_name = dir_path.file_stem().expect("File_name");

    let mut snap_dir_path: PathBuf = file_path.parent().expect("Parent directory").into();
    snap_dir_path.push("snaps");
    snap_dir_path.push(file_name);

    let snap_path: PathBuf = snap_dir_path.join(format!("{}.snap", test_case_name));
    let new_snap_path: PathBuf = snap_dir_path.join(format!("{}.snap.new", test_case_name));

    if !snap_path.exists() {
        save_new_snap(&snap_dir_path, &new_snap_path, &actual_result);

        panic!("Couldn't find snap. Created new one");
    } else {
        let expected_result = read_to_string(&snap_path).expect("Couldn't read expected snap");

        if expected_result != actual_result {
            save_new_snap(&snap_dir_path, &new_snap_path, &actual_result);

            assert_eq!(expected_result, actual_result);
        } else if new_snap_path.exists() {
            remove_file(&new_snap_path).expect("Couldn't remove new snap");
        }
    }
}

fn goto_workdir(file_path: impl AsRef<Path>) {
    let file_path = file_path.as_ref();
    let mut path = current_dir().expect("Current dir");
    loop {
        if file_path.exists() {
            break;
        }
        path = path.parent().expect("Couldn't go up").into();

        set_current_dir(&path).expect("Couldn't go up");
    }
}

fn save_new_snap(snap_dir_path: &Path, new_snap_path: &Path, result: &str) {
    let _r = create_dir_all(&snap_dir_path);
    File::create(&new_snap_path)
        .and_then(|mut file| file.write_all(result.as_bytes()))
        .expect("Couldn't save snap");
}

