use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::Ime::{
            IMC_SETCONVERSIONMODE, IMC_SETOPENSTATUS, IME_CMODE_NATIVE, ImmGetDefaultIMEWnd,
        },
        WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
    },
};

/// 현재 IME 상태를 가져옵니다 (0: 영문, 1: 한글)
fn get_ime_status(hwnd: HWND) -> i32 {
    unsafe {
        // v0.60.0에서는 인자를 Some()으로 감싸야 합니다.
        SendMessageW(hwnd, WM_IME_CONTROL, Some(WPARAM(1)), Some(LPARAM(0))).0 as i32
    }
}

/// IME 상태를 설정합니다.
fn set_ime(hwnd: HWND, ko: bool) {
    unsafe {
        // 1. IME 열기/닫기 설정 (Post 대신 Send를 사용하여 확실하게 적용)
        SendMessageW(
            hwnd,
            WM_IME_CONTROL,
            Some(WPARAM(IMC_SETOPENSTATUS as usize)),
            Some(LPARAM(if ko { 1 } else { 0 })),
        );
        
        // 2. 한글 모드일 경우 Native 모드(한글) 강제 지정
        if ko {
            SendMessageW(
                hwnd,
                WM_IME_CONTROL,
                Some(WPARAM(IMC_SETCONVERSIONMODE as usize)),
                Some(LPARAM(IME_CMODE_NATIVE.0 as isize)),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    unsafe {
        // 현재 활성화된 창 핸들 가져오기
        let fg_hwnd = GetForegroundWindow();
        // v0.60.0에서 HWND는 포인터이므로 .is_null()로 유효성 검사
        if fg_hwnd.0.is_null() { return; }

        // 해당 창의 IME 메시지를 처리할 윈도우 핸들 가져오기
        let hwnd = ImmGetDefaultIMEWnd(fg_hwnd);
        if hwnd.0.is_null() { return; }

        if args.len() < 2 {
            // 인자가 없으면 현재 상태를 출력 (Vim의 obtainIMCmd용)
            println!("{}", get_ime_status(hwnd));
        } else {
            // 인자가 있으면 해당 상태로 변경 (Vim의 switchIMCmd용)
            let ko = args[1] == "1";
            set_ime(hwnd, ko);
        }
    }
}
