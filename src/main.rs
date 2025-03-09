use eframe::egui;
use image::ImageReader;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::<ImageViewer>::default())),
    )
}

struct ImageViewer {
    image_path: Option<PathBuf>,
    texture: Option<egui::TextureHandle>,
    image_size: Option<egui::Vec2>, // 存储图片的原始尺寸
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            image_path: None,
            texture: None,
            image_size: None,
        }
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理文件拖放
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            if let Some(dropped_file) = ctx.input(|i| i.raw.dropped_files.first().cloned()) {
                if let Some(path) = dropped_file.path {
                    self.image_path = Some(path.clone());
                    self.load_image(ctx, &path);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示图片
            if let Some(texture) = &self.texture {
                if let Some(image_size) = self.image_size {
                    // 获取当前窗口的大小
                    let available_size = ui.available_size();

                    // 计算缩放比例
                    let scale = (available_size.x / image_size.x).min(available_size.y / image_size.y);
                    let scaled_size = image_size * scale;

                    // 计算居中位置
                    let margin = (available_size - scaled_size) * 0.5;
                    ui.add_space(margin.y);
                    ui.horizontal(|ui| {
                        ui.add_space(margin.x);
                        ui.add(egui::Image::new(texture).fit_to_exact_size(scaled_size));
                    });
                }
            }
        });
    }
}

impl ImageViewer {
    fn load_image(&mut self, ctx: &egui::Context, path: &PathBuf) {
        if let Ok(img) = ImageReader::open(path).unwrap().decode() {
            let rgba = img.to_rgba8();
            let image_data = egui::ColorImage::from_rgba_unmultiplied(
                [rgba.width() as usize, rgba.height() as usize],
                &rgba,
            );

            // 存储图片的原始尺寸
            self.image_size = Some(egui::Vec2::new(rgba.width() as f32, rgba.height() as f32));

            // 创建纹理，使用 Default::default() 作为纹理选项
            self.texture = Some(ctx.load_texture("image", image_data, Default::default()));
        }
    }
}
