slint::include_modules!();

mod sync;
mod translate;

use std::sync::{Arc, Mutex};
use sync::BookmarkItem;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    run_app();
}

fn main() {
    run_app();
}

fn run_app() {
    let main_window = MainWindow::new().unwrap();
    let bookmarks = Arc::new(Mutex::new(vec![
        BookmarkItem {
            title: "GitHub".to_string(),
            url: "https://github.com".to_string(),
            add_time: 1700000000,
        },
        BookmarkItem {
            title: "Bilibili".to_string(),
            url: "https://bilibili.com".to_string(),
            add_time: 1700000001,
        },
    ]));

    // 1. 网址导航处理
    let window_handle = main_window.as_weak();
    main_window.on_navigate(move |input_url| {
        let trimmed = input_url.trim();
        let final_url = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            trimmed.to_string()
        } else if trimmed.contains('.') && !trimmed.contains(' ') {
            format!("https://{}", trimmed)
        } else {
            // 默认极简搜索引擎: 百度/必应
            format!("https://www.bing.com/search?q={}", urlencoding::encode(trimmed))
        };

        println!("正在访问: {}", final_url);
        if let Some(w) = window_handle.upgrade() {
            w.set_current_url(final_url.into());
        }
    });

    // 2. 切换暗色模式
    let window_handle = main_window.as_weak();
    main_window.on_toggle_dark_mode(move || {
        if let Some(w) = window_handle.upgrade() {
            let current = w.get_is_dark_mode();
            w.set_is_dark_mode(!current);
        }
    });

    // 3. 网页翻译触发
    main_window.on_trigger_translate(move || {
        let script = translate::get_translate_script();
        println!("触发即时网页翻译脚本:\n{}", script);
    });

    // 4. GitHub 书签 Gist 云同步
    let window_handle = main_window.as_weak();
    let bookmarks_clone = bookmarks.clone();
    main_window.on_sync_github_bookmarks(move || {
        if let Some(w) = window_handle.upgrade() {
            let token = w.get_github_token().to_string();
            let b_list = bookmarks_clone.lock().unwrap().clone();
            
            w.set_sync_status("正在同步到 GitHub...".into());
            
            match sync::sync_bookmarks_to_gist(&token, &b_list) {
                Ok(url) => {
                    println!("GitHub 书签同步成功: {}", url);
                    w.set_sync_status("同步成功！已写入私有 Gist".into());
                }
                Err(err) => {
                    eprintln!("GitHub 同步报错: {}", err);
                    w.set_sync_status(format!("失败: {}", err).into());
                }
            }
        }
    });

    // 5. 退出程序
    main_window.on_exit_app(move || {
        std::process::exit(0);
    });

    main_window.run().unwrap();
}
