// build.rs
fn main() {
    // 윈도우 타겟일 때만 리소스를 컴파일합니다.
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        
        // Cargo.toml에 있는 버전과 이름을 자동으로 가져옵니다.
        // 필요하다면 아이콘(.ico)도 여기서 설정 가능합니다.
        // res.set_icon("my_icon.ico"); 

        res.set("FileDescription", "IME Auto Switcher for VSCode Vim plugin");
        res.set("ProductName", "IME Select");
        res.set("OriginalFilename", "ime-select.exe");
        res.set("LegalCopyright", "Copyright (c) 2026");

        res.compile().unwrap();
    }
}
