/// 获取全网页即时翻译注入脚本
pub fn get_translate_script() -> &'static str {
    r#"
    (function() {
        if (window._via_translated) {
            console.log("页面已在翻译中");
            return;
        }
        window._via_translated = true;
        
        var script = document.createElement('script');
        script.type = 'text/javascript';
        script.src = '//translate.google.com/translate_a/element.js?cb=googleTranslateElementInit';
        document.body.appendChild(script);

        window.googleTranslateElementInit = function() {
            new google.translate.TranslateElement({
                pageLanguage: 'auto',
                includedLanguages: 'zh-CN,en',
                layout: google.translate.TranslateElement.InlineLayout.SIMPLE
            }, 'google_translate_element');
        };
    })();
    "#
}
