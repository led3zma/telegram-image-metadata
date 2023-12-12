use std::{error::Error as std_error, ffi::OsString, fs};

/// Reads a path and returns a Vec containing every file in it
/// TODO: filter to return only images
pub fn read_image_path(path: &str) -> Result<Vec<OsString>, Box<dyn std_error>> {
    Ok(fs::read_dir(path)?
        .filter(|file| file.is_ok())
        .map(|entry| {
            entry
                .map(|file| file.file_name()) // Probably should need the file path
                .expect("Should only contain OK values")
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_invalid_image_path() {
        let path = "./tests/20230224_182502.jpg";
        assert!(read_image_path(path).is_err());
    }

    #[test]
    fn collect_only_images() {
        let path = "./tests/";
        assert!(
            read_image_path(path)
                .unwrap()
                .iter()
                .filter(|&file| {
                    let file = &file.to_str().unwrap();
                    !file.contains(".jpg") || !file.contains(".png") || !file.contains(".jpeg")
                })
                .collect::<Vec<_>>()
                .len()
                == 0
        );
    }
}
