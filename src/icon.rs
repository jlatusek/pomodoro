use image::ImageReader;
use tray_icon::TrayIconBuilder;

pub struct Tray {}

impl Tray {
    pub fn new() -> Self {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/icons/pomodoro-technique.png");
        let icon = load_icon(std::path::Path::new(path));
        std::thread::spawn(|| {
            use tray_icon::menu::Menu;

            gtk::init().unwrap();
            let _tray_icon = TrayIconBuilder::new()
                .with_menu(Box::new(Menu::new()))
                .with_icon(icon)
                .build()
                .unwrap();

            gtk::main();
        });
        return Self {};
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = ImageReader::open(path).expect("Failed to open icon path");
        let rgba = image.decode().expect("Fail").into_rgba8();
        let (width, height) = rgba.dimensions();
        (rgba.into_vec(), width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
