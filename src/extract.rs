use std::fs::{metadata, DirEntry};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use users::get_user_by_uid;

pub struct FileInfo {
    pub filename: String,
    pub file_size: String,
    pub dir_symbol: String,
    pub permissions: String,
    pub owner: String,
}

impl FileInfo {
    pub fn new(direntry: &DirEntry) -> Result<FileInfo, &'static str> {
        let filename = extract_filename(&direntry);
        let file_size = human_size(extract_file_size(&direntry));
        let dir_symbol = extract_dir_symbol(&direntry);
        let permissions = extract_permissions_string(&direntry);
        let owner = extract_username(&direntry);

        Ok(FileInfo {
            filename,
            file_size,
            dir_symbol,
            permissions,
            owner,
        })
    }
}

pub fn extract_filename(direntry: &DirEntry) -> String {
    direntry.file_name().into_string().unwrap()
}

pub fn extract_permissions_string(direntry: &DirEntry) -> String {
    let mode = metadata(direntry.path()).unwrap().permissions().mode() & 511;

    let mode_o = mode >> 6;
    let mode_g = (mode >> 3) & 7;
    let mode_a = mode & 7;

    let s_o = convert_octal_mode(mode_o);
    let s_g = convert_octal_mode(mode_g);
    let s_a = convert_octal_mode(mode_a);

    [s_o, s_g, s_a].join("")
}

pub fn convert_octal_mode(mode: u32) -> String {
    let rwx = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];
    String::from(rwx[(mode & 7 as u32) as usize])
}

pub fn extract_username(direntry: &DirEntry) -> String {
    let meta = metadata(direntry.path());
    let owner_id: u32 = meta.unwrap().uid();
    let user = get_user_by_uid(owner_id).unwrap();
    String::from(user.name().to_str().unwrap())
}

pub fn extract_dir_symbol(direntry: &DirEntry) -> String {
    let meta = metadata(direntry.path());
    if meta.unwrap().is_dir() {
        String::from("d")
    } else {
        String::from(".")
    }
}

pub fn extract_file_size(direntry: &DirEntry) -> u64 {
    direntry.path().metadata().unwrap().len()
}

pub fn human_size(bytes: u64) -> String {
    let size = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];
    let factor = (bytes.to_string().chars().count() as u64 - 1) / 3 as u64;
    let human_size = format!(
        "{:>3}{:<1}",
        bytes / (1204 as u64).pow(factor as u32),
        size[factor as usize]
    );
    human_size
}
