use dioxus::prelude::*;
use crate::router::AppRoute;

use crate::assets::header_data;

#[component]
pub fn Navbar() -> Element {
    let mut language = use_context::<Signal<String>>();
    let translations = use_memo(move || match language().as_str() {
        "fr" => (
            "Accueil",
            "À propos",
            "Retour à l'accueil Felty Oï",
        ),
        "es" => (
            "Inicio",
            "Acerca de",
            "Volver a la página de inicio de Felty Oï",
        ),
        "de" => (
            "Startseite",
            "Über",
            "Zurück zur Felty Oï Startseite",
        ),
        _ => ("Home", "About", "Return to Felty Oï Home"),
    });

    rsx! {
        nav { class: "navbar",
            div { class: "navbar-content",
                Link {
                    to: AppRoute::Home {},
                    class: "header-logo",
                    "aria-label": "{translations().2}",
                    img {
                        src: "{header_data()}",
                        alt: "Felty Oï Logo",
                        class: "header-img"
                    }
                    strong { "Felty Oï" }
                    span { "" }
                }
                div { class: "navbar-links",
                    Link {
                        to: AppRoute::Home {},
                        class: "header-link",
                        i { class: "fas fa-home mr-1" },
                        "{translations().0}"
                    }
                    Link {
                        to: AppRoute::About {},
                        class: "header-link",
                        i { class: "fas fa-info-circle mr-1" },
                        "{translations().1}"
                    }
                    select {
                        class: "language-select",
                        onchange: move |e| language.set(e.value().clone()),
                        option { value: "en", selected: language() == "en", "English" }
                        option { value: "fr", selected: language() == "fr", "Français" }
                        option { value: "es", selected: language() == "es", "Español" }
                        option { value: "de", selected: language() == "de", "Deutsch" }
                    }
                }
            }
        }
    }
}