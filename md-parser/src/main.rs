mod file_parser;
use file_parser::parse;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            return;
        }
    };

    println!("Current working directory: {}", current_dir.display());

    let src_path = current_dir.join("../blog-src");
    let html_path = current_dir.join("../html");

    let header = fs::read_to_string(current_dir.join("../blog-template/header.html")).unwrap();
    let footer = fs::read_to_string(current_dir.join("../blog-template/footer.html")).unwrap();
    let entry = fs::read_to_string(current_dir.join("../blog-template/entry.html")).unwrap();

    // Delete everything currently in html/
    clear_folder(&html_path);

    copy_imgs(&current_dir.join("../blog-src/_images"), &html_path.join("_images/"));

    // Parse md files
    match list_files_in_directory(&src_path) {
        Ok(files) => {
            // Skip WIP Entries
            let files: Vec<String> = files.into_iter().filter(|f| !f.starts_with("_")).collect();

            let entries_page = construct_entries_page(&files);
            for file in &files {
                let title = &file[..file.len() - 3];
                let content = parse(src_path.join(file).to_str().unwrap());
                let html = construct_entry(
                    entry.as_str(),
                    header.as_str(),
                    footer.as_str(),
                    content.as_str(),
                    entries_page.as_str(),
                    title,
                );
                match write_file_to_folder(
                    html_path.to_str().unwrap(),
                    format!("{}.html", title).as_str(),
                    html.as_str(),
                ) {
                    Ok(_) => println!("Converted: {}", file),
                    Err(_) => eprintln!("Error saving: {}", file),
                }
            }

            add_template(&current_dir, &html_path, &header, &footer, entries_page.as_str());
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    }
}

fn list_files_in_directory<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                files.push(file_name.to_string());
            }
        }
    }

    Ok(files)
}

fn write_file_to_folder(folder: &str, file_name: &str, content: &str) -> io::Result<()> {
    let file_path = format!("{}/{}", folder, file_name);

    fs::write(file_path, content)?;
    Ok(())
}

fn construct_entries_page(files: &[String]) -> String {
    let entry_links: String = files
        .iter()
        .map(|f| Path::new(f).file_stem().unwrap().to_str().unwrap())
        .map(|f| format!("<li><a href=\"{}.html\">{}</a></li>", f, f))
        .fold("".to_string(), |acc, s| acc + "\n" + s.as_str());

    format!(
        "<section class=\"recent-entries\">
      <h2>Recent Blog Entries</h2>
      <ul>
        {}
      </ul>
    </section>",
        entry_links
    )
}

fn construct_entry(entry: &str, header: &str, footer: &str, content: &str, entries_page: &str, title: &str) -> String {
    entry
        .replace("[TITLE]", title)
        .replace("<!--ENTRIES-->", entries_page)
        .replace("<!--ENTRY-->", content)
        .replace("<!--HEADER-->", header)
        .replace("<!--FOOTER-->", footer)
}

fn copy_imgs(img_path: &Path, html_path: &Path) {
    match list_files_in_directory(img_path) {
        Ok(files) => {
            for file in files {
                println!("path: {}", html_path.canonicalize().unwrap().to_str().unwrap());
                match fs::copy(
                    img_path.join(&file).canonicalize().unwrap(),
                    html_path.canonicalize().unwrap().join(file),
                ) {
                    Ok(_) => (),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    }
}

fn add_template(current_dir: &Path, html_path: &Path, header: &str, footer: &str, entries: &str) {
    fs::copy(
        current_dir.join("../blog-template/style.css"),
        html_path.join("style.css"),
    )
    .unwrap();
    fs::copy(current_dir.join("../blog-template/bg.png"), html_path.join("bg.png")).unwrap();

    let mut contact = fs::read_to_string(current_dir.join("../blog-template/contact.html")).unwrap();
    contact = contact
        .replace("<!--HEADER-->", header)
        .replace("<!--ENTRIES-->", entries)
        .replace("<!--FOOTER-->", footer);

    fs::write(html_path.join("contact.html"), contact).unwrap();

    let mut index = fs::read_to_string(current_dir.join("../blog-template/index.html")).unwrap();
    index = index
        .replace("<!--HEADER-->", header)
        .replace("<!--ENTRIES-->", entries)
        .replace("<!--FOOTER-->", footer);

    fs::write(html_path.join("index.html"), index).unwrap();
}

fn clear_folder(html_path: &PathBuf) {
    match list_files_in_directory(html_path) {
        Ok(files) => {
            for file in files {
                fs::remove_file(html_path.join(file)).unwrap();
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    }
}
