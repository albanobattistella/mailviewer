use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{env, fs};

const APP_ID: &str = "io.github.alescdb.mailviewer";

#[allow(dead_code)]
struct Config {
  name: String,
  out_dir: PathBuf,
  project: PathBuf,
  src: PathBuf,
  config_in: PathBuf,
  config_rs: PathBuf,
  version: String,
  target: PathBuf,
}

fn main() {
  let profile: String = env::var("PROFILE").unwrap();
  let meson: String = env::var("MESON_BUILD").unwrap_or("0".to_string());

  if meson == "1" || profile != "debug" {
    println!("cargo:warning=build.rs disabled (profile {}, meson {})", profile, meson);
    return;
  }
  println!("cargo:warning=build.rs started (profile == debug)");

  let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
  let project: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
  let src: PathBuf = project.join("src");
  let cfg = Config {
    out_dir: out_dir,
    project: project.clone(),
    src: src.clone(),
    config_in: src.clone().join("config.rs.in"),
    config_rs: src.clone().join("config.rs"),
    name: env::var("CARGO_PKG_NAME").unwrap(),
    version: env::var("CARGO_PKG_VERSION").unwrap(),
    target: project.clone().join("target").join(&profile),
  };
  //for (key, value) in env::vars() {
  //  println!("cargo:warning={} => {}", key, value);
  //}
  println!("cargo:warning=out_dir => {:?}", &cfg.out_dir);
  // println!("cargo:warning=project => {:?}", &cfg.project);
  // println!("cargo:warning=src: => {:?}", &cfg.src);
  // println!("cargo:warning=config_in => {:?}", &cfg.config_in);
  // println!("cargo:warning=config_rs => {:?}", &cfg.config_rs);
  // println!("cargo:warning=version => {:?}", &cfg.version);
  // println!("cargo:warning=target => {:?}", &cfg.target);

  println!("cargo::rerun-if-changed=build.rs");
  println!("cargo::rerun-if-changed=src/window.ui");
  println!("cargo::rerun-if-changed=src/preferences.ui");
  println!("cargo::rerun-if-changed=src/mailviewer.gresource.xml");
  println!("cargo::rerun-if-changed=data/io.github.alescdb.mailviewer.gschema.xml");
  println!(
    "cargo::rerun-if-changed={}",
    cfg.config_in.to_str().unwrap()
  );
  println!(
    "cargo::rerun-if-changed={}",
    cfg.config_rs.to_str().unwrap()
  );

  config(&cfg);
  glib_compile_resources(&cfg);
  glib_compile_schemas(&cfg);
  icons(&cfg);
}

fn icons(cfg: &Config) {
  let _output = Command::new("cp")
    .arg("-av")
    .arg("data/icons")
    .arg(format!("{}/", cfg.out_dir.to_str().unwrap()))
    .output()
    .expect("Failed to copy icons");
  if !_output.status.success() {
    println!("cargo:warning=cp icons => {:?}", &_output);
    exit(5);
  }
  let _output = Command::new("gtk4-update-icon-cache")
    .arg("-t")
    .arg("-f")
    .arg(format!("{}", cfg.out_dir.to_str().unwrap()))
    .output()
    .expect("Failed to copy icons");
  if !_output.status.success() {
    println!("cargo:warning=gtk4-update-icon-cache => {:?}", &_output);
    exit(5);
  }
  println!(
    "cargo:rustc-env=XDG_DATA_DIRS={}:{}",
    env::var("XDG_DATA_DIRS").unwrap(),
    cfg.out_dir.to_str().unwrap()
  );
}
fn config(cfg: &Config) {
  let config_in =
    fs::read_to_string(cfg.config_in.to_str().unwrap()).expect("Failed to read config.rs.in");
  let config_out = config_in
    .replace("@APP_ID@", &format!("\"{}\"", APP_ID))
    .replace("@APP_NAME@", &format!("\"{}\"", &cfg.name))
    .replace("@VERSION@", &format!("\"{}\"", &cfg.version))
    .replace(
      "@GETTEXT_PACKAGE@",
      &format!("\"{}\"", cfg.out_dir.to_str().unwrap()),
    )
    .replace(
      "@LOCALEDIR@",
      &format!("\"{}\"", cfg.out_dir.to_str().unwrap()),
    )
    .replace(
      "@PKGDATADIR@",
      &format!("\"{}\"", cfg.out_dir.to_str().unwrap()),
    );
  fs::write(cfg.config_rs.to_str().unwrap(), config_out).expect("Failed to write config.rs");
}

fn glib_compile_resources(cfg: &Config) {
  let dest_path = Path::new(&cfg.out_dir).join("mailviewer.gresource");

  eprintln!("dest_path : {:?}", dest_path);
  let _output: std::process::Output = Command::new("glib-compile-resources")
    .arg("--sourcedir=src")
    .arg(format!("--target={}", dest_path.to_str().unwrap()))
    .arg(format!(
      "{}/mailviewer.gresource.xml",
      cfg.src.to_str().unwrap()
    ))
    .output()
    .expect("Failed to build schema");

  if !_output.status.success() {
    println!("cargo:warning=glib_compile_resources => {:?}", &_output);
    exit(5);
  }
}

fn glib_compile_schemas(cfg: &Config) {
  let _output = Command::new("glib-compile-schemas")
    .arg(format!("--targetdir={}", cfg.out_dir.to_str().unwrap()))
    .arg("data")
    .output()
    .expect("Failed to build schema");

  println!(
    "cargo:rustc-env=GSETTINGS_SCHEMA_DIR={}",
    cfg.out_dir.to_str().unwrap()
  );

  if !_output.status.success() {
    println!("cargo:warning=glib_compile_resources => {:?}", &_output);
    exit(5);
  }
}
