use crate::oflag;

#[derive(Clone, Debug)]
pub struct VideoCardInfo {
    pub path: String,
    pub fd: i32
}

pub fn get_default_video_card_info() -> Option<VideoCardInfo> {
    let validated_card_pathes = std::fs::read_dir("/dev/dri").unwrap()
    .map(|x| {
        let os_path = x.as_ref().unwrap().path();
        let card_path = String::from(os_path.to_str().unwrap());

        let fd = get_fd(&card_path);
        print_debug!("path: {:#?}, fd: {:#?}", card_path, fd);
    

        let is_name_contains = x.as_ref().unwrap().file_name().to_str().unwrap().contains("card");
        let is_validated = drm_rs::core::is_validated_handle(fd);

        match is_name_contains && is_validated  {
            true => Some(VideoCardInfo{path: card_path, fd}),
            false => Option::None,
        }

    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<VideoCardInfo>>();

    if validated_card_pathes.is_empty() == false {
        Some(validated_card_pathes.first().unwrap().clone())
    }
    else {
        Option::None
    }
}

pub fn get_fd(device_path: &str) -> libc::c_int {
    let mut path = device_path.bytes().collect::<Vec<libc::c_char>>();
    path.push(b'\0');
    unsafe {
        libc::open(path.as_ptr(), oflag::OFlag::ReadWrite as _)
    }
}