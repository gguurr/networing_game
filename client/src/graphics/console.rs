use super::tileset::TileSet;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler};
use glium::{glutin, Surface};

#[derive(Clone, Copy)]
pub struct ColoredChar {
    id: u32,
    fg_color: [u8; 3],
    bg_color: [u8; 3],
    use_true_color: bool,
}

impl ColoredChar {
    pub fn new_with_color(id: u32, fg_color: [u8; 3], bg_color: [u8; 3]) -> Self {
        ColoredChar {
            id,
            fg_color,
            bg_color,
            use_true_color: false,
        }
    }
    pub fn new_with_true_color(id: u32) -> Self {
        ColoredChar {
            id,
            fg_color: [0,0,0],
            bg_color: [0,0,0],
            use_true_color: true,
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_position: [f32; 2],
    fg: [f32; 3],
    bg: [f32; 3],
    overlay: f32,
    use_true_color: f32,
}
implement_vertex!(Vertex, position, tex_position, fg, bg, overlay, use_true_color);

pub struct Root {
    display: glium::Display,
    tile_set: TileSet,
    texture: glium::Texture2d,
    screens: Vec<Vec<ColoredChar>>,
    pub size: (u32, u32),
    vertexes: Vec<Vertex>,
    shader_program: glium::Program,
    pub events_loop: glutin::EventsLoop,
}

impl Root {
    pub fn new(tile_set: TileSet, size: (u32, u32), name: &str) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_title(name);
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        display.gl_window().window().set_maximized(true);
        let total_size = (size.0 * size.1) as usize;
        let empty_cc = ColoredChar {
            id: 0,
            fg_color: [255, 255, 255],
            bg_color: [0, 0, 0],
            use_true_color: false,
        };
        let screen = vec![empty_cc; total_size];
        let screens = vec![screen; 1];
        let img = &tile_set.img;
        let img_d = img.dimensions();
        let img = glium::texture::RawImage2d::from_raw_rgb_reversed(&img.clone().into_raw(), img_d);
        let texture = glium::texture::Texture2d::new(&display, img).unwrap();
        let v_shader_src = r#"
            #version 140

            in vec2 position;
            in vec2 tex_position;
            in vec3 fg;
            in vec3 bg;
            in float overlay;
            in float use_true_color;

            out vec2 v_tex_position;
            out vec3 c_fg;
            out vec3 c_bg;
            out float v_overlay;
            out float c_use_true_color;

            void main() {
                c_fg = fg;
                c_bg = bg;
                v_tex_position = tex_position;
                v_overlay = overlay;
                c_use_true_color = use_true_color;
                gl_Position = vec4(position,overlay,1.0);
            }
        "#;
        let f_shader_src = r#"
            #version 140

            in vec2 v_tex_position;
            in vec3 c_fg;
            in vec3 c_bg;
            in float v_overlay;
            in float c_use_true_color;

            out vec4 color;

            uniform sampler2D glyph;

            void main() {
                vec4 c_val = texture(glyph, v_tex_position);
                if(c_use_true_color == 0){
                    float val = (c_val.x + c_val.y + c_val.z) / 3.0;
                    vec3 stap = vec3((c_fg.x - c_bg.x), (c_fg.y - c_bg.y), (c_fg.z - c_bg.z));
                    float r = c_bg.x + (stap.x * val);
                    float g = c_bg.y + (stap.y * val);
                    float b = c_bg.z + (stap.z * val);
                    float a = 1.0;
                    if(v_overlay > 0){
                        a = val;
                    }
                    color = vec4(r, g, b, a);
                    color.w = a;
                } else {
                    color = c_val;
                }
            }
        "#;
        let shader_program =
            glium::Program::from_source(&display, v_shader_src, f_shader_src, None).unwrap();
        Root {
            display,
            tile_set,
            screens,
            size,
            texture,
            shader_program,
            events_loop,
            vertexes: Vec::new(),
        }
    }

    pub fn rescale(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    pub fn clear(&mut self) {
        self.screens.clear();
        let total_size = (self.size.0 * self.size.1) as usize;
        let empty_cc = ColoredChar {
            id: 0,
            fg_color: [255, 255, 255],
            bg_color: [0, 0, 0],
            use_true_color: false,
        };
        self.screens = vec![vec![empty_cc; total_size]; 1];
    }
    pub fn put_char(&mut self, char: u32, pos: (u32, u32), true_color: bool) {
        let pos_y = (-(pos.1 as i64) + self.size.1 as i64) as u32 - 1;
        let loc = pos.0 + (self.size.0 * pos_y);
        self.screens[0][loc as usize].id = char;
        self.screens[0][loc as usize].use_true_color = true_color;
    }
    pub fn set_background(&mut self, color: [u8; 3], pos: (u32, u32)) {
        let pos_y = (-(pos.1 as i64) + self.size.1 as i64) as u32 - 1;
        let loc = pos.0 + (self.size.0 * pos_y);
        self.screens[0][loc as usize].bg_color = color;
    }
    pub fn set_foreground(&mut self, color: [u8; 3], pos: (u32, u32)) {
        let pos_y = (-(pos.1 as i64) + self.size.1 as i64) as u32 - 1;
        let loc = pos.0 + (self.size.0 * pos_y);
        self.screens[0][loc as usize].fg_color = color;
    }
    pub fn put_colored_char(
        &mut self,
        char: u32,
        foreground: [u8; 3],
        background: [u8; 3],
        pos: (u32, u32)
    ) {
        let pos_y = (-(pos.1 as i64) + self.size.1 as i64) as u32 - 1;
        let loc = pos.0 + (self.size.0 * pos_y);
        self.screens[0][loc as usize].id = char;
        self.screens[0][loc as usize].fg_color = foreground;
        self.screens[0][loc as usize].bg_color = background;
    }
    pub fn put_colored_multichar(
        &mut self,
        chars: Vec<u32>,
        foreground: [u8; 3],
        background: [u8; 3],
        pos: (u32, u32),
    ) {
        let pos_y = (-(pos.1 as i64) + self.size.1 as i64) as u32 - 1;
        let loc = pos.0 + (self.size.0 * pos_y);
        for (i, char) in chars.iter().enumerate() {
            if self.screens.len() <= i {
                self.add_overlay(self.create_overlay());
            }
            self.screens[i][loc as usize].id = *char;
            self.screens[i][loc as usize].fg_color = foreground;
            self.screens[i][loc as usize].bg_color = background;
        }
    }
    pub fn add_overlay(&mut self, layer: Vec<ColoredChar>) {
        self.screens.push(layer);
    }

    pub fn put_colored_char_overlay(
        &self,
        layer: &mut Vec<ColoredChar>,
        char: u32,
        foreground: [u8; 3],
        background: [u8; 3],
        pos: (u32, u32),
        use_true_color: bool,
    ) {
        let pos_y = (-(pos.1 as i64) + self.size.1 as i64) as u32 - 1;
        let loc = pos.0 + (self.size.0 * pos_y);
        layer[loc as usize].id = char;
        layer[loc as usize].fg_color = foreground;
        layer[loc as usize].bg_color = background;
        layer[loc as usize].use_true_color = use_true_color;

    }

    pub fn create_overlay(&self) -> Vec<ColoredChar> {
        let total_size = (self.size.0 * self.size.1) as usize;
        let empty_cc = ColoredChar {
            id: 0,
            fg_color: [255, 255, 255],
            bg_color: [0, 0, 0],
            use_true_color: false,
        };
        let screen = vec![empty_cc; total_size];
        screen
    }
    pub fn put_colored_str(
        &mut self,
        text: &str,
        foreground: [u8; 3],
        background: [u8; 3],
        pos: (u32, u32),
    ) {
        for (i, c) in text.chars().enumerate() {
            if (pos.0 + i as u32) < self.size.0 {
                self.put_colored_char(c as u32, foreground, background, (pos.0 + i as u32, pos.1))
            }
        }
    }

    pub fn draw(&mut self) {
        let width = (*self.display.gl_window()).window().get_inner_size().unwrap().width;
        let glyph_ratio = self.tile_set.ratio() as f32;
        let glyph_size = width as f32 / self.size.0 as f32;
        let glyph_scale = glyph_size / width as f32 * 2.0;
        self.vertexes.clear();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let local_x = (x as f32 / self.size.0 as f32) * 2.0 - 1.0;
                let local_y = (y as f32 / self.size.1 as f32) * 2.0 - 1.0;
                for (i, screen) in self.screens.iter().enumerate() {
                    let cc = screen[(x + (self.size.0 * y)) as usize];
                    let bg: [f32; 3] = [
                        cc.bg_color[0] as f32 / 255.0,
                        cc.bg_color[1] as f32 / 255.0,
                        cc.bg_color[2] as f32 / 255.0,
                    ];
                    let fg: [f32; 3] = [
                        cc.fg_color[0] as f32 / 255.0,
                        cc.fg_color[1] as f32 / 255.0,
                        cc.fg_color[2] as f32 / 255.0,
                    ];
                    let tex_cords = self.tile_set.get_glyph(cc.id);
                    let overlay = i as f32 / 5.0;
                    let v1 = Vertex {
                        position: [local_x, local_y],
                        tex_position: tex_cords[0],
                        bg,
                        fg,
                        overlay,
                        use_true_color: if cc.use_true_color {1.0} else {0.0},
                    };
                    let v2 = Vertex {
                        position: [local_x, local_y + (glyph_scale * glyph_ratio)],
                        tex_position: tex_cords[1],
                        bg,
                        fg,
                        overlay,
                        use_true_color: if cc.use_true_color {1.0} else {0.0},
                    };
                    let v3 = Vertex {
                        position: [local_x + glyph_scale, local_y + (glyph_scale * glyph_ratio)],
                        tex_position: tex_cords[2],
                        bg,
                        fg,
                        overlay,
                        use_true_color: if cc.use_true_color {1.0} else {0.0},
                    };
                    let v4 = Vertex {
                        position: [local_x + glyph_scale, local_y],
                        tex_position: tex_cords[3],
                        bg,
                        fg,
                        overlay,
                        use_true_color: if cc.use_true_color {1.0} else {0.0},
                    };
                    let mut shape = vec![v1, v2, v3, v1, v3, v4];
                    self.vertexes.append(&mut shape);
                }
            }
        }

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        let ver_buff = glium::VertexBuffer::new(&self.display, &self.vertexes).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let mut s_glyph = Sampler::new(&self.texture);
        s_glyph = s_glyph.minify_filter(MinifySamplerFilter::Nearest);
        s_glyph = s_glyph.magnify_filter(MagnifySamplerFilter::Nearest);
        let uni = uniform! { glyph: s_glyph };
        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };
        target
            .draw(&ver_buff, &indices, &self.shader_program, &uni, &params)
            .unwrap();
        target.finish().unwrap();
    }
}
