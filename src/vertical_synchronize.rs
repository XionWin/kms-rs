use egl_rs::Context;

pub trait VerticalSynchronizeTrait {
    fn init_double_buffer(&self, gbm: &mut gbm_rs::Gbm, drm: &drm_rs::Drm);
    fn wait_vertical_synchronize(&self, p1: &mut gbm_rs::Gbm, p2: &drm_rs::Drm);
}

impl VerticalSynchronizeTrait for Context {
    fn init_double_buffer(&self, gbm: &mut gbm_rs::Gbm, drm: &drm_rs::Drm) {
        let surface = gbm.get_surface_mut();

        let func = |display: *const libc::c_void, surface: *const libc::c_void| {
            egl_rs::swap_buffers(display, surface)
        };
        surface.register_swap_callback((func, self.display as _, self.surface as _));

        let (_, fb) = surface.lock();
        let drm_fd = drm.get_fd();
        let drm_crtc_id = drm.crtc.get_id();
        let drm_connector_ids = &vec![drm.connector.get_id()];
        let drm_mode = drm.get_mode().get_handle();
        match drm_rs::set_crtc(
            drm_fd,
            drm_crtc_id,
            fb as _,
            0,
            0,
            drm_connector_ids.as_ptr(),
            drm_connector_ids.len() as _,
            drm_mode,
        ) {
            result if result == 0 => {
                colored_rs::print_debug!("set_crtc: {:#x?}", drm_crtc_id);
                result
            },
            _ => panic!("surface initialize set_crtc error"),
        };
    }

    fn wait_vertical_synchronize(&self, gbm: &mut gbm_rs::Gbm, drm: &drm_rs::Drm) {
        let fd = drm.get_fd();
        let crtc_id = drm.crtc.get_id();
        let surface = gbm.get_surface_mut();
        let (_, fb) = surface.lock();
        if self.is_vertical_synchronize {
            drm_rs::vertical_synchronize(fd, crtc_id, fb);
        }
    }
}
