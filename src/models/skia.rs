pub struct Canvas<'a>(pub &'a skia_safe::Canvas);
unsafe impl Send for Canvas<'_> {}
unsafe impl Sync for Canvas<'_> {}

pub struct Image(pub skia_safe::Image);
unsafe impl Send for Image {}
unsafe impl Sync for Image {}

pub struct Surface(pub skia_safe::Surface);
unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}
