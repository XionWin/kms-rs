use egl_rs::def::SurfaceType;

use crate::{vertical_synchronize::VerticalSynchronizeTrait, utility};

pub struct KMS {
    drm: drm_rs::Drm,
    gbm: gbm_rs::Gbm,
    context: egl_rs::Context,
    width: libc::c_int,
    height: libc::c_int
}

impl KMS {
    pub fn new(device: Option<&str>, surface_type: SurfaceType) -> Self {
        let selected_video_card_info = match utility::get_video_card_info(device) {
            Some(card_info) => card_info,
            None => panic!("Video card not found"),
        };
        print_debug!(
            "selected_video_card_info: {:?}, fd: {:?}",
            selected_video_card_info.path,
            selected_video_card_info.fd
        );
    
        let fd = selected_video_card_info.fd;
        let drm = drm_rs::core::Drm::new(fd, |conn| {
            conn.get_connection_status() == drm_rs::ConnectionStatus::Connected
        });
        let mode = drm.get_mode();
        print_debug!(
            "actived_mode: {:?} type: {}",
            mode.get_name(),
            mode.get_mode_type()
                .iter_names()
                .map(|x| x.0)
                .collect::<Vec<_>>()
                .join(" ")
        );
    
        let (width, height) = (drm.crtc.get_width(), drm.crtc.get_height());
    
        let mut gbm = gbm_rs::Gbm::new(
            fd,
            width,
            height,
            gbm_rs::def::SurfaceFormat::ARGB8888,
            vec![gbm_rs::def::FormatModifier::DRM_FORMAT_MOD_LINEAR],
        );
    
        let supported_surface_format = gbm_rs::def::SurfaceFormat::iter()
            .into_iter()
            .filter(|format| {
                gbm.get_surface()
                    .get_device()
                    .is_format_supported(*format, gbm_rs::def::SurfaceFlags::Linear)
            })
            .collect::<Vec<_>>();
    
        print_debug!(
            "supported_surface_formats: {}",
            supported_surface_format
                .into_iter()
                .map(|format| format!("{:?} ", format))
                .collect::<Vec<_>>()
                .join(" ")
        );
    
        let context: egl_rs::Context = egl_rs::Context::new(
            gbm.get_surface().get_handle(),
            gbm.get_surface().get_device().get_handle(),
            surface_type,
            width,
            height,
            true,
        );
        context.init_double_buffer(&mut gbm, &drm);
        
        Self {
            drm,
            gbm,
            context,
            width,
            height
        }
    }

    pub fn wait_vertical_synchronize(&mut self) {
        self.context.wait_vertical_synchronize(&mut self.gbm, &self.drm);
    }

    pub fn get_width(&self) -> libc::c_int {
        self.width
    }

    pub fn get_height(&self) -> libc::c_int {
        self.height
    }
}

#[macro_export]
macro_rules! begin_render {
    ($init:ident, $update:ident, $kms:expr) => {
        let mut graphic = $init($kms);
        loop {
            $update($kms, &mut graphic);
            $kms.wait_vertical_synchronize();
        }
    };
}
