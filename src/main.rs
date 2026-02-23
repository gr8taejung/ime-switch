use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::Ime::{
            IMC_SETCONVERSIONMODE, IMC_SETOPENSTATUS, IME_CMODE_NATIVE, ImmGetDefaultIMEWnd,
        },
        WindowsAndMessaging::{
            GetForegroundWindow, SendMessageW, SendNotifyMessageW, WM_IME_CONTROL
        },
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
/// 상태 변경은 응답을 기다리지 않는 비동기식(SendNotifyMessageW)을 사용하여 즉시 종료합니다.
fn set_ime(hwnd: HWND, ko: bool) {
    unsafe {
        // SendNotifyMessageW는 메시지를 큐에 넣고 즉시 리턴합니다.
        let _ = SendNotifyMessageW(
            hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as usize),
            LPARAM(if ko { 1 } else { 0 }),
        );
        
        if ko {
            let _ = SendNotifyMessageW(
                hwnd,
                WM_IME_CONTROL,
                WPARAM(IMC_SETCONVERSIONMODE as usize),
                LPARAM(IME_CMODE_NATIVE.0 as isize),
            );
        }
        // OS가 메시지를 발송할 최소한의 시간을 확보 (1ms)
        std::thread::sleep(std::time::Duration::from_millis(1));
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
