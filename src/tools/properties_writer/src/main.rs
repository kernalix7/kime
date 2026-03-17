use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    configurations: Vec<Configuration>,
    version: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Configuration {
    name: String,
    intelli_sense_mode: String,
    include_path: HashSet<String>,
    compiler_path: String,
    c_standard: String,
    cpp_standard: String,
    browse: Browse,
    configuration_provider: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Browse {
    path: Vec<String>,
    limit_symbols_to_included_headers: bool,
    database_filename: String,
}

fn main() {
    let cc = std::env::var("CC").unwrap_or("gcc".into());

    let ret = Command::new("whereis")
        .arg(&cc)
        .output()
        .expect("Failed to run whereis")
        .stdout;
    let ret = String::from_utf8(ret).expect("whereis output is not valid UTF-8");
    let cc_path = ret.split(' ').nth(1).unwrap_or("/usr/bin/gcc");

    let ret = Command::new("pkg-config")
        .arg("--list-all")
        .output()
        .expect("Failed to run pkg-config; is it installed?")
        .stdout;
    let ret = String::from_utf8(ret).expect("pkg-config output is not valid UTF-8");

    let mut include_path = HashSet::new();

    let packages = ret.lines().filter_map(|l| l.split(' ').next());

    for package in packages {
        let output = match Command::new("pkg-config")
            .arg("--cflags-only-I")
            .arg(package)
            .output()
        {
            Ok(o) => o,
            Err(_) => continue,
        };
        let ret = String::from_utf8_lossy(&output.stdout);
        let paths = ret.split(' ');

        for path in paths {
            if !path.starts_with("-I") {
                continue;
            }

            include_path.insert(path.split_at(2).1.trim().into());
        }
    }

    include_path.insert("${workspaceFolder}".into());
    include_path.insert("${workspaceFolder}/src/engine/capi".into());

    let config = Configuration {
        name: "include paths".into(),
        intelli_sense_mode: "clang-x64".into(),
        include_path,
        compiler_path: cc_path.into(),
        c_standard: "c11".into(),
        cpp_standard: "c++17".into(),
        configuration_provider: "ms-vscode.cmake-tools".into(),
        browse: Browse {
            path: vec!["${workspaceFolder}/**".into()],
            limit_symbols_to_included_headers: true,
            database_filename: String::new(),
        },
    };

    let root = Root {
        version: 4,
        configurations: vec![config],
    };

    println!("{}", serde_json::to_string_pretty(&root).unwrap());
}
