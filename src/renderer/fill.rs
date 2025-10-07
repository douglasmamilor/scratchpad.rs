use super::Renderer;

impl<'a> Renderer<'a> {
    pub fn flood_fill<F>(&self, px: (i32, i32), matched: F)
    where
        F: Fn(u32, u32) -> bool,
    {
        let (x, y) = (px.0 as usize, px.1 as usize);

        let t = self.framebuffer.get_pixel(x as usize, y as usize);
        if t.is_none() {
            return;
        }

        let target = t.unwrap();

        let expand_run = |x: usize, y: usize| -> (i32, i32) {
            let mut xl = x;
            while let Some(c) = self.framebuffer.get_pixel(xl - 1, y) {
                if !matched(c, target) {
                    break;
                }
                xl -= 1;
            }

            let mut xr = x;
            while let Some(c) = self.framebuffer.get_pixel(xr + 1, y) {
                if !matched(c, target) {
                    break;
                }
                xr += 1;
            }

            (xl as i32, xr as i32)
        };
        let stack: Vec<(i32, i32, i32)> = Vec::new();
    }
}
