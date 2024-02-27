use directories::ProjectDirs;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn todo() {
    let proj_dirs = ProjectDirs::from("com", "Red Hat", PKG_NAME)
        .expect("Failed to find a home directory.");
    let conf_dir = proj_dirs.config_dir();
    let conf_file = conf_dir.join(format!("{PKG_NAME}.toml"));
    println!("Configuration file:  {}", conf_file.display());
}
