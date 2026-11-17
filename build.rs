use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src-svelte");
    println!("cargo:rerun-if-changed=static");
    println!("cargo:rerun-if-changed=vite.config.ts");
    println!("cargo:rerun-if-changed=svelte.config.js");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=yarn.lock");
    println!("cargo:rerun-if-changed=.yarnrc.yml");

    let status = Command::new("yarn")
        .arg("build")
        .status()
        .expect("failed to execute yarn build");

    if !status.success() {
        panic!("failed to build SvelteKit dashboard");
    }
}
