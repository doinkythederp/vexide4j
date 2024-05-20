use std::{fs::remove_dir_all, path::Path, process::Command};

use copy_dir::copy_dir;
fn main() {
    println!("cargo:rerun-if-changed=*.gradle.kts");
    println!("cargo:rerun-if-changed=src/main");
    println!("cargo:rerun-if-changed=gradlew*");
    println!("cargo:rerun-if-changed=rt_dependencies.txt");

    let gradlew = Command::new("./gradlew")
        .arg("build")
        .status()
        .expect("Failed to build project");
    assert!(gradlew.success());

    let app_class_path = Path::new("build/classes/java/main");
    let rt_class_path = Path::new("lib");
    let out_dir = Path::new("build/classpath");
    remove_dir_all(out_dir).unwrap();
    copy_dir(app_class_path, out_dir).unwrap();

    let dependencies = std::fs::read_to_string("rt_dependencies.txt").unwrap();
    for dep in dependencies.lines() {
        if dep.is_empty() || dep.starts_with('#') {
            continue;
        }
        let dep_path = rt_class_path.join(dep);
        let dep_out = out_dir.join(dep);
        std::fs::create_dir_all(dep_out.parent().unwrap()).unwrap();
        copy_dir(dep_path, dep_out).unwrap();
    }
}
