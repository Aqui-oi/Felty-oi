use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::desktop::tao::window::Icon;
use dioxus::LaunchBuilder;
use image::load_from_memory;
use router::AppRoute;

mod components;
mod router;
mod assets;

fn main() {
    let icon = load_icon();

    let cfg = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("Felty Oï")
                .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1280.0, 720.0))
                .with_min_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(800.0, 600.0))
                .with_resizable(true),
        )
        .with_icon(icon);

    LaunchBuilder::new()
        .with_cfg(cfg)
        .launch(App);
}

fn load_icon() -> Icon {
    // Le fichier est embarqué dans le binaire
    let bytes = include_bytes!("../assets/header_256.png");
    let img = load_from_memory(bytes)
        .expect("Erreur lecture image")
        .into_rgba8();
    let (w, h) = img.dimensions();
    Icon::from_rgba(img.into_raw(), w, h).expect("Erreur création icône")
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new("en".to_string()));

    rsx! {
        // Favicon
        link {
            rel: "icon",
            href: "{assets::favicon_data()}",
            r#type: "image/x-icon"
        }

        // Styles
        style { "{assets::main_css()}" }
        style { "{assets::tailwind_css()}" }

        // FontAwesome
        link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css"
        }

        Router::<AppRoute> {}
    }
}
