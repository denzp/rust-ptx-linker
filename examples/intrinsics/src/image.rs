pub struct InputPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct OutputPixel {
    pub l: u8,
}

pub struct Image<T> {
    pub pixels: *const T,
    pub width: i32,
}

pub struct MutImage<T> {
    pub pixels: *mut T,
    pub width: i32,
}

impl<T> Image<T> {
    fn offset(&self, i: i32, j: i32) -> isize {
        (i * self.width + j) as isize
    }

    pub unsafe fn pixel(&self, i: i32, j: i32) -> &T {
        &*self.pixels.offset(self.offset(i, j))
    }
}

impl<T> MutImage<T> {
    fn offset(&self, i: i32, j: i32) -> isize {
        (i * self.width + j) as isize
    }

    pub unsafe fn mut_pixel(&mut self, i: i32, j: i32) -> &mut T {
        &mut *self.pixels.offset(self.offset(i, j))
    }
}
