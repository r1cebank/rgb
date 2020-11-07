use rand::Rng;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

// Digital image with mode RGB. Size = 144 * 160 * 3.
// 3---------
// ----------
// ----------
// ---------- 160
//        144
pub type PPUFramebuffer = [[[u8; 3]; SCREEN_W]; SCREEN_H];

pub fn random_framebuffer() -> PPUFramebuffer {
    let mut framebuffer = [[[0x00; 3]; SCREEN_W]; SCREEN_H];
    let mut rng = rand::thread_rng();
    for i in 0..framebuffer.len() {
        let random_color = [
            rng.gen_range(0, 256),
            rng.gen_range(0, 256),
            rng.gen_range(0, 256),
        ];
        for j in 0..framebuffer[i].len() {
            framebuffer[i][j] = [
                random_color[0] as u8,
                random_color[1] as u8,
                random_color[2] as u8,
            ];
        }
    }

    framebuffer
}
