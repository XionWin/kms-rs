#[derive(Debug)]
pub struct Graphic<T> {
    width: libc::c_int,
    height: libc::c_int,
    tag: T
}

impl<T> Graphic<T> {
    pub fn new(width: libc::c_int, height: libc::c_int, tag: T) -> Self {
        Self {
            width,
            height,
            tag
         }
    }

    pub fn get_width(&self) -> libc::c_int {
        self.width
    }

    pub fn get_height(&self) -> libc::c_int {
        self.height
    }
    pub fn get_tag(&self) -> &T {
        &self.tag
    }

    pub fn get_tag_mut(&mut self) -> &mut T {
        &mut self.tag
    }
}