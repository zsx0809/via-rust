use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookmarkItem {
    pub title: String,
    pub url: String,
    pub add_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFileContent {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistPayload {
    pub description: String,
    pub public: bool,
    pub files: std::collections::HashMap<String, GistFileContent>,
}

#[derive(Debug, Deserialize)]
pub struct GistResponse {
    pub id: String,
    pub html_url: String,
}

const GIST_FILENAME: &str = "via_rust_bookmarks.json";

/// 将书签同步保存到 GitHub Secret Gist
pub fn sync_bookmarks_to_gist(token: &str, bookmarks: &[BookmarkItem]) -> Result<String, String> {
    if token.trim().is_empty() {
        return Err("GitHub Token 为空，请先配置个人访问令牌".into());
    }

    let json_data = serde_json::to_string_pretty(bookmarks)
        .map_err(|e| format!("序列化书签失败: {}", e))?;

    let mut files = std::collections::HashMap::new();
    files.insert(
        GIST_FILENAME.to_string(),
        GistFileContent { content: json_data },
    );

    let payload = GistPayload {
        description: "ViaRust 浏览器书签备份".to_string(),
        public: false, // 私有 Gist
        files,
    };

    let body = serde_json::to_string(&payload)
        .map_err(|e| format!("构建请求失败: {}", e))?;

    let response = ureq::post("https://api.github.com/gists")
        .set("Authorization", &format!("token {}", token.trim()))
        .set("User-Agent", "ViaRust-Android-Browser")
        .set("Accept", "application/vnd.github.v3+json")
        .send_string(&body);

    match response {
        Ok(res) => {
            let parsed: GistResponse = res.into_json()
                .map_err(|e| format!("解析 GitHub 响应失败: {}", e))?;
            Ok(parsed.html_url)
        }
        Err(e) => Err(format!("同步失败: {}", e)),
    }
}
