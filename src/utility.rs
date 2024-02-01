use crate::oflag;

#[derive(Clone, Debug)]
pub struct VideoCardInfo {
    pub path: String,
    pub fd: i32,
}

pub fn get_video_card_info(path: Option<&str>) -> Option<VideoCardInfo> {
    match path {
        Some(str_path) => {
            let card_infos = get_available_video_card_infos();
            match card_infos {
                Some(infos) => {
                    let selected_cards = infos
                        .into_iter()
                        .filter(|x| x.path == str_path)
                        .collect::<Vec<_>>();
                    let selected_card = selected_cards.first();
                    match selected_card {
                        Some(selected_card) => Some(selected_card.clone()),
                        None => Option::None,
                    }
                }
                None => Option::None,
            }
        }
        None => get_default_video_card_info(),
    }
}

fn get_default_video_card_info() -> Option<VideoCardInfo> {
    let validated_card_pathes = std::fs::read_dir("/dev/dri")
        .unwrap()
        .map(|x| {
            let os_path = x.as_ref().unwrap().path();
            let card_path = String::from(os_path.to_str().unwrap());

            let fd = get_fd(&card_path);
            print_debug!("path: {:?}, fd: {:?}", card_path, fd);

            let is_name_contains = x
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .contains("card");
            let is_validated = drm_rs::core::is_validated_handle(fd);

            match is_name_contains && is_validated {
                true => Some(VideoCardInfo {
                    path: card_path,
                    fd,
                }),
                false => Option::None,
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    if validated_card_pathes.is_empty() == false {
        Some(validated_card_pathes.first().unwrap().clone())
    } else {
        Option::None
    }
}

fn get_available_video_card_infos() -> Option<Vec<VideoCardInfo>> {
    let validated_card_pathes = std::fs::read_dir("/dev/dri")
        .unwrap()
        .map(|x| {
            let os_path = x.as_ref().unwrap().path();
            let card_path = String::from(os_path.to_str().unwrap());

            let fd = get_fd(&card_path);
            print_debug!("path: {:?}, fd: {:?}", card_path, fd);

            let is_validated = drm_rs::core::is_validated_handle(fd);

            match is_validated {
                true => Some(VideoCardInfo {
                    path: card_path,
                    fd,
                }),
                false => Option::None,
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    if validated_card_pathes.is_empty() == false {
        Some(validated_card_pathes)
    } else {
        Option::None
    }
}

pub fn get_fd(device_path: &str) -> libc::c_int {
    let mut path = device_path.bytes().collect::<Vec<_>>();
    path.push(b'\0');
    unsafe { libc::open(path.as_ptr(), oflag::OFlag::ReadWrite as _) }
}
