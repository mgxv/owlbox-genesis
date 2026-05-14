fn main() {
    println!("cargo:rerun-if-changed=../injected");
    tauri_build::build()
}
