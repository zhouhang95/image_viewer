use eframe::egui;
use image::ImageReader;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([800.0, 600.0]),
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
        // 检查 ESC 键
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // 处理文件拖放
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            if let Some(dropped_file) = ctx.input(|i| i.raw.dropped_files.first().cloned()) {
                if let Some(path) = dropped_file.path {
                    self.image_path = Some(path.clone());
                    self.load_image(ctx, &path);
                    
                    // 获取屏幕大小
                    
                    if let Some(image_size) = self.image_size {
                        // 计算合适的窗口大小
                        let screen_rect = ctx.screen_rect();
                        let screen_size = screen_rect.size();
                        println!("Image size: {}x{}", image_size.x, image_size.y);
                        println!("Screen ratio: {}", screen_size.x / screen_size.y);
                        println!("Image ratio: {}", image_size.x / image_size.y);
                        
                        let screen_ratio = screen_size.x / screen_size.y;
                        let image_ratio = image_size.x / image_size.y;
                        
                        let window_size = if image_ratio > screen_ratio {
                            // 图片更宽，以屏幕宽度为准
                            let width = screen_size.x * 0.9; // 留出一些边距
                            egui::Vec2::new(width, width / image_ratio)
                        } else {
                            // 图片更高，以屏幕高度为准
                            let height = screen_size.y * 0.9; // 留出一些边距
                            egui::Vec2::new(height * image_ratio, height)
                        };
                        
                        // 调整窗口大小并居中
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(window_size));
                        let window_pos = egui::Pos2::new(
                            (screen_size.x - window_size.x) * 0.5,
                            (screen_size.y - window_size.y) * 0.5
                        );
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(window_pos));
                    }
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
