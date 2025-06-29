use crate::components::navbar::Navbar;
use dioxus::prelude::*;
// use gloo_timers::future::sleep;
use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};
// use std::time::Duration;
use walkdir::WalkDir;

/// Test whether a path matches any of the exclusion rules
fn is_excluded(
    entry: &Path,
    ext_on: bool,
    exts: &[String],
    file_on: bool,
    files: &[String],
    dir_on: bool,
    dirs: &[String],
) -> bool {
    if ext_on {
        if let Some(ext) = entry.extension().and_then(|s| s.to_str()) {
            if exts.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                return true;
            }
        }
    }
    if file_on {
        if let Some(name) = entry.file_name().and_then(|s| s.to_str()) {
            if files.iter().any(|f| f.eq_ignore_ascii_case(name)) {
                return true;
            }
        }
    }
    if dir_on {
        for ancestor in entry.ancestors() {
            if let Some(dir) = ancestor.file_name().and_then(|s| s.to_str()) {
                if dirs.iter().any(|d| d.eq_ignore_ascii_case(dir)) {
                    return true;
                }
            }
        }
    }
    false
}

/// Recursively collect every file that should be copied
fn gather_files(
    src: &Path,
    ext_on: bool,
    exts: Vec<String>,
    file_on: bool,
    files: Vec<String>,
    dir_on: bool,
    dirs: Vec<String>,
) -> Vec<PathBuf> {
    WalkDir::new(src)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| !is_excluded(e.path(), ext_on, &exts, file_on, &files, dir_on, &dirs))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Copy `file` from `src_root` to `dst_root`, preserving relative path
fn copy_one(src_root: &Path, dst_root: &Path, file: &Path) -> std::io::Result<()> {
    let rel = file.strip_prefix(src_root).expect("path not under root");
    let dst = dst_root.join(rel);
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(file, dst)?;
    Ok(())
}



#[derive(Clone, PartialEq)]
struct Translations {
    title: &'static str,
    source_folder: &'static str,
    dest_folder: &'static str,
    exclusion_options: &'static str,
    exclude_extensions: &'static str,
    ext_placeholder: &'static str,
    exclude_files: &'static str,
    file_placeholder: &'static str,
    exclude_folders: &'static str,
    folder_placeholder: &'static str,
    browse: &'static str,
    copy: &'static str,
    transfer: &'static str,
    cancel: &'static str,
    confirm_transfer: &'static str,
    confirm_message: &'static str,
    confirm: &'static str,
    success_message: &'static str,
    overall_progress: &'static str,
    progress_info: &'static str,
    current_file: &'static str,
}

#[component]
pub fn Home() -> Element {
    let language = use_context::<Signal<String>>();
    let translations = use_memo(move || match language().as_str() {
        "fr" => Translations {
            title: "Transfert de fichiers",
            source_folder: "Dossier source",
            dest_folder: "Dossier destination",
            exclusion_options: "Options d'exclusion",
            exclude_extensions: "Exclure les extensions",
            ext_placeholder: "Ex: png, jpg, mp4",
            exclude_files: "Exclure les fichiers",
            file_placeholder: "Ex: secret.txt, README.md",
            exclude_folders: "Exclure les dossiers",
            folder_placeholder: "Ex: target, node_modules",
            browse: "Parcourir",
            copy: "Copier",
            transfer: "Transférer",
            cancel: "Annuler",
            confirm_transfer: "Confirmer le transfert",
            confirm_message: "Cette action va copier tous les fichiers non exclus, puis supprimer les originaux.",
            confirm: "Confirmer",
            success_message: "Transfert terminé avec succès !",
            overall_progress: "Progression globale — {}%",
            progress_info: "{} fichiers traités | {}s écoulées | ~{}s restantes",
            current_file: "Fichier courant — {}%",
        },
        "es" => Translations {
            title: "Transferencia de archivos",
            source_folder: "Carpeta de origen",
            dest_folder: "Carpeta de destino",
            exclusion_options: "Opciones de exclusión",
            exclude_extensions: "Excluir extensiones",
            ext_placeholder: "Ej: png, jpg, mp4",
            exclude_files: "Excluir archivos",
            file_placeholder: "Ej: secret.txt, README.md",
            exclude_folders: "Excluir carpetas",
            folder_placeholder: "Ej: target, node_modules",
            browse: "Explorar",
            copy: "Copiar",
            transfer: "Transferir",
            cancel: "Cancelar",
            confirm_transfer: "Confirmar transferencia",
            confirm_message: "Esta acción copiará todos los archivos no excluidos y luego eliminará los originales.",
            confirm: "Confirmar",
            success_message: "¡Transferencia completada con éxito!",
            overall_progress: "Progreso global — {}%",
            progress_info: "{} archivos procesados | {}s transcurridos | ~{}s restantes",
            current_file: "Archivo actual — {}%",
        },
        "de" => Translations {
            title: "Dateiübertragung",
            source_folder: "Quellordner",
            dest_folder: "Zielordner",
            exclusion_options: "Ausschlussoptionen",
            exclude_extensions: "Erweiterungen ausschließen",
            ext_placeholder: "Bsp: png, jpg, mp4",
            exclude_files: "Dateien ausschließen",
            file_placeholder: "Bsp: secret.txt, README.md",
            exclude_folders: "Ordner ausschließen",
            folder_placeholder: "Bsp: target, node_modules",
            browse: "Durchsuchen",
            copy: "Kopieren",
            transfer: "Übertragen",
            cancel: "Abbrechen",
            confirm_transfer: "Übertragung bestätigen",
            confirm_message: "Diese Aktion kopiert alle nicht ausgeschlossenen Dateien und löscht dann die Originale.",
            confirm: "Bestätigen",
            success_message: "Übertragung erfolgreich abgeschlossen!",
            overall_progress: "Gesamtfortschritt — {}%",
            progress_info: "{} Dateien verarbeitet | {}s vergangen | ~{}s verbleibend",
            current_file: "Aktuelle Datei — {}%",
        },
        _ => Translations {
            title: "File Transfer",
            source_folder: "Source Folder",
            dest_folder: "Destination Folder",
            exclusion_options: "Exclusion Options",
            exclude_extensions: "Exclude Extensions",
            ext_placeholder: "Ex: png, jpg, mp4",
            exclude_files: "Exclude Files",
            file_placeholder: "Ex: secret.txt, README.md",
            exclude_folders: "Exclude Folders",
            folder_placeholder: "Ex: target, node_modules",
            browse: "Browse",
            copy: "Copy",
            transfer: "Transfer",
            cancel: "Cancel",
            confirm_transfer: "Confirm Transfer",
            confirm_message: "This action will copy all non-excluded files and then delete the originals.",
            confirm: "Confirm",
            success_message: "Transfer completed successfully!",
            overall_progress: "Overall Progress — {}%",
            progress_info: "{} files processed | {}s elapsed | ~{}s remaining",
            current_file: "Current File — {}%",
        },
    });

    //---------------------------------- STATES ----------------------------------
    let mut src_path = use_signal(|| String::new());
    let mut dst_path = use_signal(|| String::new());
    let mut exclude_ext_on = use_signal(|| false);
    let mut exclude_file_on = use_signal(|| false);
    let mut exclude_dir_on = use_signal(|| false);
    let mut exclude_ext_list = use_signal(|| String::new());
    let mut exclude_file_list = use_signal(|| String::new());
    let mut exclude_dir_list = use_signal(|| String::new());
    let overall_progress = use_signal(|| 0u32);
    let current_progress = use_signal(|| 0u32);
    let mut done_files = use_signal(|| 0usize);
    let mut elapsed_time = use_signal(|| 0u64);
    let mut estimated_remaining = use_signal(|| 0u64);
    let mut transfer_done = use_signal(|| false);
    let mut show_confirm = use_signal(|| false);
    let mut running = use_signal(|| false);

    // Auto-hide notification after 3 seconds
   /* 
    use_effect(move || {
        if transfer_done() {
            let mut td = transfer_done.clone();
            spawn(async move {
                sleep(Duration::from_secs(3)).await;
                td.set(false);
            });
        }
    });
    */

    //-------------------------------- TRANSFER ACTION -------------------------------- 👇
    let mut start_transfer = {
        let src_path = src_path.clone();
        let dst_path = dst_path.clone();
        let exclude_ext_on = exclude_ext_on.clone();
        let exclude_file_on = exclude_file_on.clone();
        let exclude_dir_on = exclude_dir_on.clone();
        let exclude_ext_list = exclude_ext_list.clone();
        let exclude_file_list = exclude_file_list.clone();
        let exclude_dir_list = exclude_dir_list.clone();
        let mut overall_progress = overall_progress.clone();
        let mut current_progress = current_progress.clone();
        let mut running = running.clone();

        move |_| {
            if src_path().is_empty() || dst_path().is_empty() {
                println!("Chemins source/destination manquants");
                return;
            }

            running.set(true);
            overall_progress.set(0);
            current_progress.set(0);
            done_files.set(0);
            elapsed_time.set(0);
            estimated_remaining.set(0);
            transfer_done.set(false);

            let exts: Vec<String> = exclude_ext_list()
                .split(',')
                .map(|s| s.trim().trim_start_matches('.').to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let files: Vec<String> = exclude_file_list()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let dirs: Vec<String> = exclude_dir_list()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let src_root = PathBuf::from(src_path());
            let dst_root = PathBuf::from(dst_path());

            spawn({
                let mut global_prog = overall_progress.clone();
                let mut file_prog = current_progress.clone();
                let mut run = running.clone();

                async move {
                    let to_do = gather_files(
                        &src_root,
                        exclude_ext_on(),
                        exts,
                        exclude_file_on(),
                        files,
                        exclude_dir_on(),
                        dirs,
                    );
                    let total = to_do.len().max(1) as f32;

                    let start_time = std::time::Instant::now();

                    for (idx, f) in to_do.iter().enumerate() {
                        file_prog.set(0);
                        if let Err(e) = copy_one(&src_root, &dst_root, f) {
                            println!("Erreur copie: {e}");
                            continue;
                        }

                        if let Err(e) = std::fs::remove_file(f) {
                            println!("Erreur suppression: {e}");
                        }

                        file_prog.set(100);
                        let pct = (((idx + 1) as f32 / total) * 100.0).round() as u32;
                        global_prog.set(pct);
                        done_files.set(idx + 1);
                        let elapsed = start_time.elapsed().as_secs();
                        let files_done = idx + 1;
                        let files_total = to_do.len().max(1);
                        let estimated = if files_done > 0 {
                            (elapsed as f32 / files_done as f32 * (files_total - files_done) as f32)
                                .round() as u64
                        } else {
                            0
                        };

                        elapsed_time.set(elapsed);
                        estimated_remaining.set(estimated);
                    }

                    run.set(false);
                    transfer_done.set(true);
                }
            });
        }
    };

    //-------------------------------- COPY ACTION --------------------------------
    let start_copy = {
        let src_path = src_path.clone();
        let dst_path = dst_path.clone();
        let exclude_ext_on = exclude_ext_on.clone();
        let exclude_file_on = exclude_file_on.clone();
        let exclude_dir_on = exclude_dir_on.clone();
        let exclude_ext_list = exclude_ext_list.clone();
        let exclude_file_list = exclude_file_list.clone();
        let exclude_dir_list = exclude_dir_list.clone();
        let overall_progress = overall_progress.clone();
        let current_progress = current_progress.clone();
        let running = running.clone();

        move |_| {
            if src_path().is_empty() || dst_path().is_empty() {
                println!("Chemins source/destination manquants");
                return;
            }

            let mut run = running.clone();
            let mut global_prog = overall_progress.clone();
            let mut file_prog = current_progress.clone();

            run.set(true);
            global_prog.set(0);
            file_prog.set(0);
            done_files.set(0);
            elapsed_time.set(0);
            estimated_remaining.set(0);
            transfer_done.set(false);

            let exts: Vec<String> = exclude_ext_list()
                .split(',')
                .map(|s| s.trim().trim_start_matches('.').to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let files: Vec<String> = exclude_file_list()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let dirs: Vec<String> = exclude_dir_list()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let src_root = PathBuf::from(src_path());
            let dst_root = PathBuf::from(dst_path());

            spawn({
                let mut global_prog = global_prog;
                let mut file_prog = file_prog;
                let mut run = run;

                async move {
                    let to_do = gather_files(
                        &src_root,
                        exclude_ext_on(),
                        exts,
                        exclude_file_on(),
                        files,
                        exclude_dir_on(),
                        dirs,
                    );
                    let total = to_do.len().max(1) as f32;

                    let start_time = std::time::Instant::now();

                    for (idx, f) in to_do.iter().enumerate() {
                        file_prog.set(0);
                        if let Err(e) = copy_one(&src_root, &dst_root, f) {
                            println!("Erreur copie {:?}: {}", f, e);
                            continue;
                        }
                        file_prog.set(100);
                        let pct = (((idx + 1) as f32 / total) * 100.0).round() as u32;
                        global_prog.set(pct);

                        done_files.set(idx + 1);
                        let elapsed = start_time.elapsed().as_secs();
                        let files_done = idx + 1;
                        let files_total = to_do.len().max(1);
                        let estimated = if files_done > 0 {
                            (elapsed as f32 / files_done as f32 * (files_total - files_done) as f32)
                                .round() as u64
                        } else {
                            0
                        };

                        elapsed_time.set(elapsed);
                        estimated_remaining.set(estimated);
                    }
                    run.set(false);
                    transfer_done.set(true);
                }
            });
        }
    };

    //------------------------------------ UI ------------------------------------
    rsx! {
        Navbar {}
        // Confirmation Popup
        if show_confirm() {
            div {
                class: "popup-overlay",
                aria_modal: "true",
                role: "dialog",
                div {
                    class: "popup-box",
                    h3 {
                        class: "popup-title",
                        i { class: "fas fa-exclamation-triangle mr-2" }
                        "{translations().confirm_transfer}"
                    }
                    p {
                        class: "popup-text",
                        "{translations().confirm_message}"
                    }
                    nav {
                        class: "popup-actions",
                        button {
                            class: "btn btn-cancel",
                            onclick: move |_| show_confirm.set(false),
                            "{translations().cancel}"
                        }
                        button {
                            class: "btn btn-confirm",
                            onclick: move |_| {
                                show_confirm.set(false);
                                start_transfer(());
                            },
                            "{translations().confirm}"
                        }
                    }
                }
            }
        }
        // Success Notification
        if transfer_done() {
            div {
                class: "notification",
                i { class: "fas fa-check-circle mr-2" }
                span { "{translations().success_message}" }
            }
        }
        // Main Content
        main {
            class: "main-container",
            h1 {
                class: "main-title",
                i { class: "fas fa-file-export mr-2" }
                "{translations().title}"
            }
            section {
                class: "content-section",
                // Source Path
                div {
                    class: "path-group",
                    label {
                        class: "path-label",
                        i { class: "fas fa-folder mr-2" }
                        "{translations().source_folder}"
                    }
                    div {
                        class: "input-group",
                        input {
                            class: "path-input",
                            r#type: "text",
                            value: "{src_path()}",
                            oninput: move |e| src_path.set(e.value().clone())
                        }
                        button {
                            class: "btn btn-browse",
                            onclick: move |_| {
                                if let Some(folder) = FileDialog::new().pick_folder() {
                                    src_path.set(folder.display().to_string());
                                }
                            },
                            i { class: "fas fa-search mr-2" }
                            "{translations().browse}"
                        }
                    }
                }
                // Destination Path
                div {
                    class: "path-group",
                    label {
                        class: "path-label",
                        i { class: "fas fa-folder-open mr-2" }
                        "{translations().dest_folder}"
                    }
                    div {
                        class: "input-group",
                        input {
                            class: "path-input",
                            r#type: "text",
                            value: "{dst_path()}",
                            oninput: move |e| dst_path.set(e.value().clone())
                        }
                        button {
                            class: "btn btn-browse",
                            onclick: move |_| {
                                if let Some(folder) = FileDialog::new().pick_folder() {
                                    dst_path.set(folder.display().to_string());
                                }
                            },
                            i { class: "fas fa-search mr-2" }
                            "{translations().browse}"
                        }
                    }
                }
                // Exclusions Panel
                details {
                    class: "exclusion-panel",
                    open: "{exclude_ext_on() || exclude_file_on() || exclude_dir_on()}",
                    summary {
                        class: "exclusion-summary",
                        i { class: "fas fa-filter mr-2" }
                        "{translations().exclusion_options}"
                    }
                    div {
                        class: "exclusion-content",
                        // Extensions
                        label {
                            class: "exclusion-label",
                            input {
                                r#type: "checkbox",
                                checked: exclude_ext_on(),
                                onchange: move |_| exclude_ext_on.set(!exclude_ext_on())
                            }
                            span { "{translations().exclude_extensions}" }
                        }
                        if exclude_ext_on() {
                            textarea {
                                class: "exclusion-textarea",
                                placeholder: "{translations().ext_placeholder}",
                                value: "{exclude_ext_list()}",
                                oninput: move |e| exclude_ext_list.set(e.value().clone())
                            }
                        }
                        // Files
                        label {
                            class: "exclusion-label",
                            input {
                                r#type: "checkbox",
                                checked: exclude_file_on(),
                                onchange: move |_| exclude_file_on.set(!exclude_file_on())
                            }
                            span { "{translations().exclude_files}" }
                        }
                        if exclude_file_on() {
                            textarea {
                                class: "exclusion-textarea",
                                placeholder: "{translations().file_placeholder}",
                                value: "{exclude_file_list()}",
                                oninput: move |e| exclude_file_list.set(e.value().clone())
                            }
                        }
                        // Directories
                        label {
                            class: "exclusion-label",
                            input {
                                r#type: "checkbox",
                                checked: exclude_dir_on(),
                                onchange: move |_| exclude_dir_on.set(!exclude_dir_on())
                            }
                            span { "{translations().exclude_folders}" }
                        }
                        if exclude_dir_on() {
                            textarea {
                                class: "exclusion-textarea",
                                placeholder: "{translations().folder_placeholder}",
                                value: "{exclude_dir_list()}",
                                oninput: move |e| exclude_dir_list.set(e.value().clone())
                            }
                        }
                    }
                }
                // Progress Section
                if running() {
                    section {
                        class: "progress-section animate-slide-in",
                        div {
                            class: "progress-item",
                            span {
                                class: "progress-label",
                                { format_args!("{} {}", translations().overall_progress, overall_progress()) }
                            }
                            div {
                                class: "progress-bar",
                                div {
                                    class: "progress-fill",
                                    style: "width: {overall_progress()}%;"
                                }
                            }
                            p {
                                class: "progress-info",
                                { format_args!("{} {} {} {}", translations().progress_info, done_files(), elapsed_time(), estimated_remaining()) }
                            }
                        }
                        div {
                            class: "progress-item",
                            span {
                                class: "progress-label",
                                { format_args!("{}{}", translations().current_file, current_progress()) }
                            }
                            div {
                                class: "progress-bar",
                                div {
                                    class: "progress-fill",
                                    style: "width: {current_progress()}%;"
                                }
                            }
                        }
                    }
                }
                // Action Buttons
                nav {
                    class: "action-buttons",
                    button {
                        class: "btn btn-copy",
                        disabled: running(),
                        onclick: start_copy,
                        i { class: "fas fa-copy mr-2" }
                        "{translations().copy}"
                    }
                    button {
                        class: "btn btn-transfer",
                        disabled: running(),
                        onclick: move |_| show_confirm.set(true),
                        i { class: "fas fa-truck mr-2" }
                        "{translations().transfer}"
                    }
                    button {
                        class: "btn btn-cancel-action",
                        disabled: !running(),
                        onclick: move |_| running.set(false),
                        i { class: "fas fa-ban mr-2" }
                        "{translations().cancel}"
                    }
                }
            }
        }
    }
}