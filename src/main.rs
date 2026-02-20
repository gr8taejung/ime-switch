use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::Ime::{
            IMC_SETCONVERSIONMODE, IMC_SETOPENSTATUS, IME_CMODE_NATIVE, ImmGetDefaultIMEWnd,
        },
        WindowsAndMessaging::{GetForegroundWindow, PostMessageW, SendMessageW, WM_IME_CONTROL},
    },
};

fn get_ime_status(hwnd: HWND) -> i32 {
    unsafe { 
        // Some()을 제거하고 직접 전달합니다.
        SendMessageW(hwnd, WM_IME_CONTROL, WPARAM(1), LPARAM(0)).0 as i32 
    }
}

fn set_ime(hwnd: HWND, ko: bool) {
    unsafe {
        // PostMessageW도 직접 전달 방식으로 수정합니다.
        let _ = PostMessageW(
            hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as usize),
            LPARAM(if ko { 1 } else { 0 }),
        );
        if ko {
            let _ = PostMessageW(
                hwnd,
                WM_IME_CONTROL,
                WPARAM(IMC_SETCONVERSIONMODE as usize),
                LPARAM(IME_CMODE_NATIVE.0 as isize),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // GetForegroundWindow 결과가 유효한지 체크
    let fg_hwnd = unsafe { GetForegroundWindow() };
    if fg_hwnd.0 == 0 { return; }

    let hwnd = unsafe { ImmGetDefaultIMEWnd(fg_hwnd) };
    if hwnd.0 == 0 { return; }

    if args.len() < 2 {
        println!("{}", get_ime_status(hwnd));
    } else {
        let ko = args[1] == "1";
        set_ime(hwnd, ko);
    }
}
