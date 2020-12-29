#[derive(Clone)]
pub struct WRITE {
    pub wallpaper: String,
    pub theme: String,
    pub colors: Vec<pastel::Color>,
}

impl WRITE {
    fn set_wallpaper(&mut self, new_val: String) {
        self.wallpaper = new_val;
    }
    fn set_theme(&mut self, new_val: String) {
        self.theme = new_val;
    }
    fn set_colors(&mut self, new_val: Vec<pastel::Color>) {
        self.colors = new_val;
    }
}

#[derive(Clone)]
pub struct SCHEME {
    wallpaper: Option<String>,
    walldir: Option<String>,
    configs: Option<String>,
    templates: Option<String>,
    loop_t: Option<u8>,
    theme: Option<String>,
    palette: Option<String>,
    sort: Option<String>,
    saturation: Option<f32>,
    illumination: Option<f32>,
    hue: Option<f32>,
    difference: Option<f32>,
    blend: Option<f32>,
    mixes: Option<Vec<Option<pastel::Color>>>,
}
