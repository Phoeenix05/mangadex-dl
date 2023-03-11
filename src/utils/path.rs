pub const DL_PATH: &str = "~/mdx-dl";

pub fn get_path(file_name: &str) -> String {
  format!("{DL_PATH}/{file_name}")
}