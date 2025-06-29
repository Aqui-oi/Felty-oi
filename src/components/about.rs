use dioxus::prelude::*;
use crate::components::navbar::Navbar;

#[derive(Clone, PartialEq)]
struct Translations {
    title: &'static str,
    how_it_works: &'static str,
    description: &'static str,
    copy_label: &'static str,
    copy_desc: &'static str,
    transfer_label: &'static str,
    transfer_desc: &'static str,
    action_note: &'static str,
    creator: &'static str,
    website: &'static str,
    email: &'static str,
    github: &'static str,
}

#[component]
pub fn About() -> Element {
    let language = use_context::<Signal<String>>();
    let translations = use_memo(move || match language().as_str() {
        "fr" => Translations {
            title: "À propos de Felty Oï",
            how_it_works: "Fonctionnement",
            description: "Felty Oï est un utilitaire de bureau écrit en Rust avec Dioxus 0.6.3. Il permet de copier ou transférer des fichiers rapidement avec exclusion avancée.",
            copy_label: "Copier :",
            copy_desc: "duplique les fichiers sans modifier l'original.",
            transfer_label: "Transférer :",
            transfer_desc: "copie les fichiers puis les supprime du dossier source.",
            action_note: "Choisissez votre action avec les boutons en bas de l'écran principal.",
            creator: "Créé et codé par Zyouax",
            website: "Site web :",
            email: "Email :",
            github: "Github :",
        },
        "es" => Translations {
            title: "Acerca de Felty Oï",
            how_it_works: "Funcionamiento",
            description: "Felty Oï es una utilidad de escritorio escrita en Rust con Dioxus 0.6.3. Permite copiar o transferir archivos rápidamente con exclusión avanzada.",
            copy_label: "Copiar:",
            copy_desc: "duplica los archivos sin modificar el original.",
            transfer_label: "Transferir:",
            transfer_desc: "copia los archivos y luego los elimina de la carpeta de origen.",
            action_note: "Elija su acción con los botones en la parte inferior de la pantalla principal.",
            creator: "Creado y codificado por Zyouax",
            website: "Sitio web:",
            email: "Correo:",
            github: "Github:",
        },
        "de" => Translations {
            title: "Über Felty Oï",
            how_it_works: "Funktionsweise",
            description: "Felty Oï ist ein Desktop-Dienstprogramm, geschrieben in Rust mit Dioxus 0.6.3. Es ermöglicht das schnelle Kopieren oder Übertragen von Dateien mit erweiterten Ausschlussoptionen.",
            copy_label: "Kopieren:",
            copy_desc: "dupliziert Dateien ohne das Original zu verändern.",
            transfer_label: "Übertragen:",
            transfer_desc: "kopiert Dateien und löscht sie anschließend aus dem Quellordner.",
            action_note: "Wählen Sie Ihre Aktion mit den Schaltflächen am unteren Rand des Hauptbildschirms.",
            creator: "Erstellt und programmiert von Zyouax",
            website: "Website:",
            email: "E-Mail:",
            github: "Github:",
        },
        _ => Translations {
            title: "About Felty Oï",
            how_it_works: "How It Works",
            description: "Felty Oï is a desktop utility written in Rust with Dioxus 0.6.3. It allows for quick file copying or transferring with advanced exclusion options.",
            copy_label: "Copy:",
            copy_desc: "duplicates files without modifying the original.",
            transfer_label: "Transfer:",
            transfer_desc: "copies files and then deletes them from the source folder.",
            action_note: "Choose your action with the buttons at the bottom of the main screen.",
            creator: "Created and coded by Zyouax",
            website: "Website:",
            email: "Email:",
            github: "Github:",
        },
    });

    rsx! {
        Navbar {}
        div {
            class: "about-container",
            h1 {
                class: "about-title",
                i { class: "fas fa-info-circle mr-2" }
                "{translations().title}"
            }
            div {
                class: "content-section",
                p {
                    class: "about-text",
                    "{translations().description}"
                }
                div {
                    class: "function-box",
                    h2 {
                        class: "function-title",
                        i { class: "fas fa-bolt mr-2" }
                        "{translations().how_it_works}"
                    }
                    ul {
                        class: "function-list",
                        li {
                            span { class: "highlight-green", "{translations().copy_label}" }
                            " {translations().copy_desc}"
                        }
                        li {
                            span { class: "highlight-yellow", "{translations().transfer_label}" }
                            " {translations().transfer_desc}"
                        }
                    }
                    p {
                        class: "function-note",
                        "{translations().action_note}"
                    }
                }
                div {
                    class: "creator-section",
                    p {
                        class: "creator-text",
                        "{translations().creator}"
                    }
                    div {
                        class: "creator-links",
                        a {
                            href: "https://aquioi.com",
                            class: "creator-link",
                            i { class: "fas fa-globe mr-1" }
                            "{translations().website} AquiOi.com"
                        }
                        a {
                            href: "mailto:Github.project@aquioi.com",
                            class: "creator-link",
                            i { class: "fas fa-envelope mr-1" }
                            "{translations().email} Github.project@aquioi.com"
                        }
                        a {
                            href: "https://github.com/Zyouax/Felty-Oi",
                            class: "creator-link",
                            i { class: "fab fa-github mr-1" }
                            "{translations().github} Aqui-oi/Felty-Oi"
                        }
                    }
                }
            }
        }
    }
}