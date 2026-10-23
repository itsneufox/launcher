fn main() {
  build_ompwine();
  tauri_build::build()
}

// ompwine.dll (ompwine/ompwine.c), which the launcher embeds and injects into
// games under Wine (src/wine_overlay.rs). Built 32-bit like the game.
fn build_ompwine() {
  let source = "ompwine/ompwine.c";
  println!("cargo:rerun-if-changed={}", source);

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let out = out_dir.join("ompwine.dll");
  let target = std::env::var("TARGET").unwrap();
  if !target.contains("windows") {
    std::fs::write(&out, []).unwrap();
    return;
  }

  // GTA San Andreas is 32-bit whatever the launcher is
  let compiler = cc::Build::new()
    .target(&target.replacen("x86_64", "i686", 1))
    .cargo_metadata(false)
    .get_compiler();
  let mut command = compiler.to_command();
  if compiler.is_like_msvc() {
    command
      .args(["/nologo", "/O2", "/MT", "/LD", source])
      .arg(format!("/Fe{}", out.display()))
      .arg(format!("/Fo{}\\", out_dir.display()))
      .args(["/link", "user32.lib", "gdi32.lib"]);
  } else {
    command
      .args(["-O2", "-Wall", "-shared", "-static-libgcc", "-s", "-o"])
      .arg(&out)
      .arg(source)
      .args(["-lgdi32", "-luser32"]);
  }
  let status = command
    .status()
    .unwrap_or_else(|e| panic!("cannot compile {}: {}", source, e));
  assert!(status.success(), "compiling {} failed", source);
}
