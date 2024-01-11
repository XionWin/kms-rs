use crate::{initializer::EglContextOutsideInitTrait, utility, Context};

pub fn begin_with<T1, T2>(init_func: T1, update_func: T2) where T1: Fn(&Context), T2: Fn(&Context) {

    let default_video_card_info = utility::get_default_video_card_info().unwrap();
    print_info!(
        "default_video_card_path: {:#?}, fd: {:#?}",
        default_video_card_info.path,
        default_video_card_info.fd
    );

    let fd = default_video_card_info.fd;
    let drm = drm_rs::core::Drm::new(fd, |conn| {
        conn.get_connection_status() == drm_rs::ConnectionStatus::Connected
    });
    let mode = drm.get_mode();
    print_info!("actived_mode_name: {:#?}", mode.get_name());

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
        .collect::<Vec<gbm_rs::def::SurfaceFormat>>();

    print_info!(
        "supported_surface_formats: {}",
        supported_surface_format
            .into_iter()
            .map(|format| format!("{:?} ", format))
            .collect::<Vec<String>>()
            .join(" ")
    );

    let context: egl_rs::Context = egl_rs::Context::new(
        gbm.get_surface().get_handle(),
        gbm.get_surface().get_device().get_handle(),
        width,
        height,
        true,
    );
    print_warning!("context: {:#?}", context);
    context.initialize(&mut gbm, &drm);

    let kms_context = Context::new(width, height);
    init_func(&kms_context);
    loop {
        update_func(&kms_context);
        context.frame_vertical_synchronize(&mut gbm, &drm);
    }
}