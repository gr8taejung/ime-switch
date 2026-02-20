use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::Ime::{
            IMC_SETCONVERSIONMODE, IMC_SETOPENSTATUS, IME_CMODE_NATIVE, ImmGetDefaultIMEWnd,
        },
        WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
    },
};

fn get_ime_status(hwnd: HWND) -> i32 {
    unsafe {
        // 현재 IME가 켜져 있는지(1) 꺼져 있는지(0) 확인
        SendMessageW(hwnd, WM_IME_CONTROL, WPARAM(1), LPARAM(0)).0 as i32
    }
}

fn set_ime(hwnd: HWND, ko: bool) {
    unsafe {
        // 1. IME 열기/닫기 상태 설정
        SendMessageW(
            hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as usize),
            LPARAM(if ko { 1 } else { 0 }),
        );
        
        // 2. 한글 모드일 경우 추가로 Native(한글) 모드 강제 지정
        if ko {
            SendMessageW(
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
    
    // 현재 포커스된 창의 핸들을 가져옴
    let fg_hwnd = unsafe { GetForegroundWindow() };
    if fg_hwnd.0 == 0 { return; }

    // 해당 창의 기본 IME 윈도우 핸들을 가져옴
    let hwnd = unsafe { ImmGetDefaultIMEWnd(fg_hwnd) };
    if hwnd.0 == 0 { return; }

    if args.len() < 2 {
        // 인자가 없으면 현재 상태 출력 (Vim의 obtainIMCmd용)
        println!("{}", get_ime_status(hwnd));
    } else {
        // 인자가 있으면(0 또는 1) 해당 상태로 변경 (Vim의 switchIMCmd용)
        let ko = args[1] == "1";
        set_ime(hwnd, ko);
    }
}
