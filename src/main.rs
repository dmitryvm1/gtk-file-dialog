use std::ptr;

use web_view::Content;

use crate::{dialog_gtk::GtkFileDialog, file_dialog::FileDialog};

mod dialog_gtk;
mod file_dialog;

fn main() {
    std::thread::spawn(|| {
        let initialized =
        unsafe { gtk_sys::gtk_init_check(ptr::null_mut(), ptr::null_mut()) == 1 };
        println!("GTK initialized: {}", initialized);
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    let webview = web_view::builder()
        .title("Test")
        .content(Content::Html(r#"<html>
        <head>
        </head>
        <body>
        <script>
            window.webkit.messageHandlers.external.postMessage("Test")
        </script>
        </body>
        </html>"#))
        .resizable(true)
        .debug(true)
        .user_data(1)
        .visible(true)
        .invoke_handler(|webview, arg| {

            let res = 
                FileDialog::new()
                .set_title("Choose a file")
                .add_filter("Image files", &["jpg", "jpeg", "png", "gif", "bmp"]);
            let dialog = GtkFileDialog::build_pick_file(&res);
            let result = if dialog.run() == gtk_sys::GTK_RESPONSE_ACCEPT {
                dialog.get_result()
            } else {
                None
            };
            println!("{:?}", result);
            Ok(())
        });
    webview.run().unwrap();
}
