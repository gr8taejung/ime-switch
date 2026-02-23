use std::env;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::Ime::{
            IMC_SETCONVERSIONMODE, IMC_SETOPENSTATUS, IME_CMODE_NATIVE, ImmGetDefaultIMEWnd,
        },
        WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
    },
};

/// 현재 IME 상태 조회 (동기 방식)
fn get_ime_status(hwnd: HWND) -> i32 {
    unsafe {
        // v0.60.0 규격: WPARAM/LPARAM을 Some()으로 감싸서 전달
        SendMessageW(hwnd, WM_IME_CONTROL, Some(WPARAM(1)), Some(LPARAM(0))).0 as i32
    }
}

/// IME 상태 설정 (안정적인 SendMessageW 동기 방식)
fn set_ime(hwnd: HWND, ko: bool) {
    unsafe {
        if ko {
            // 한글로 바꿀 때는 '열기'와 '모드 설정' 두 번 호출 (안전성 최우선)
            SendMessageW(
                hwnd,
                WM_IME_CONTROL,
                Some(WPARAM(IMC_SETOPENSTATUS as usize)),
                Some(LPARAM(1)),
            );
            SendMessageW(
                hwnd,
                WM_IME_CONTROL,
                Some(WPARAM(IMC_SETCONVERSIONMODE as usize)),
                Some(LPARAM(IME_CMODE_NATIVE.0 as isize)),
            );
        } else {
            // 영문으로 바꿀 때는 '닫기'만 한 번 호출 (속도 최우선)
            // IME가 닫히면 모드는 자동으로 무시되므로 가장 빠릅니다.
            SendMessageW(
                hwnd,
                WM_IME_CONTROL,
                Some(WPARAM(IMC_SETOPENSTATUS as usize)),
                Some(LPARAM(0)),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    unsafe {
        // 현재 활성화된 창 핸들 캡처
        let fg_hwnd = GetForegroundWindow();
        if fg_hwnd.0.is_null() { return; }

        // 해당 창의 기본 IME 윈도우 핸들 확보
        let hwnd = ImmGetDefaultIMEWnd(fg_hwnd);
        if hwnd.0.is_null() { return; }

        if args.len() < 2 {
            // 조회 모드: 결과 출력
            println!("{}", get_ime_status(hwnd));
        } else {
            // 변경 모드: 동기식으로 확실하게 변경 후 종료
            let ko = args[1] == "1";
            set_ime(hwnd, ko);
        }
    }
}
