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
    unsafe { SendMessageW(hwnd, WM_IME_CONTROL, Some(WPARAM(1)), Some(LPARAM(0))).0 as i32 }
}

fn set_ime(hwnd: HWND, ko: bool) {
    unsafe {
        let _ = PostMessageW(hwnd, WM_IME_CONTROL, WPARAM(IMC_SETOPENSTATUS as usize), LPARAM(if ko { 1 } else { 0 }));
        if ko {
            let _ = PostMessageW(hwnd, WM_IME_CONTROL, WPARAM(IMC_SETCONVERSIONMODE as usize), LPARAM(IME_CMODE_NATIVE.0 as isize));
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let hwnd = unsafe { ImmGetDefaultIMEWnd(GetForegroundWindow()) };
    if hwnd.is_invalid() { return; }

    if args.len() < 2 {
        println!("{}", get_ime_status(hwnd));
    } else {
        set_ime(hwnd, args[1] == "1");
    }
}
