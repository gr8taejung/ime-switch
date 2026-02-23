use std::env;
use std::process::Command;
use std::os::windows::process::CommandExt;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::Ime::{
            IMC_SETCONVERSIONMODE, IMC_SETOPENSTATUS, IME_CMODE_NATIVE, ImmGetDefaultIMEWnd,
        },
        WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
    },
};

const DETACHED_PROCESS: u32 = 0x00000008;
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn set_ime_sync(hwnd: HWND, ko: bool) {
    unsafe {
        let imm_hwnd = ImmGetDefaultIMEWnd(hwnd);
        if imm_hwnd.0.is_null() { return; }

        SendMessageW(
            imm_hwnd,
            WM_IME_CONTROL,
            Some(WPARAM(IMC_SETOPENSTATUS as usize)),
            Some(LPARAM(if ko { 1 } else { 0 })),
        );
        if ko {
            SendMessageW(
                imm_hwnd,
                WM_IME_CONTROL,
                Some(WPARAM(IMC_SETCONVERSIONMODE as usize)),
                Some(LPARAM(IME_CMODE_NATIVE.0 as isize)),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // [변경 모드 - 자식(백그라운드)]
    // 부모로부터 받은 HWND를 사용하여 실제 작업을 수행합니다.
    if args.len() == 4 && args[3] == "--bg" {
        let ko = args[1] == "1";
        // 인자로 받은 HWND 주소를 다시 포인터로 복원합니다.
        let hwnd_val = args[2].parse::<usize>().unwrap_or(0);
        if hwnd_val != 0 {
            let hwnd = HWND(hwnd_val as *mut std::ffi::c_void);
            set_ime_sync(hwnd, ko);
        }
        return;
    }

    // [공통] 현재 활성 창 캡처 (부모/조회 모드 공통)
    let fg_hwnd = unsafe { GetForegroundWindow() };
    if fg_hwnd.0.is_null() { return; }

    if args.len() < 2 {
        // [조회 모드] 동기식으로 즉시 출력
        unsafe {
            let imm_hwnd = ImmGetDefaultIMEWnd(fg_hwnd);
            let status = SendMessageW(imm_hwnd, WM_IME_CONTROL, Some(WPARAM(1)), Some(LPARAM(0))).0;
            println!("{}", status);
        }
    } else {
        // [변경 모드 - 부모]
        // 1. 현재 창의 HWND를 숫자로 변환합니다.
        let hwnd_str = (fg_hwnd.0 as usize).to_string();
        let mode = &args[1];
        let current_exe = env::current_exe().unwrap();
        
        // 2. 자식에게 HWND를 넘겨주며 실행 (DETACHED)
        let _ = Command::new(current_exe)
            .arg(mode)
            .arg(hwnd_str)
            .arg("--bg")
            .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
            .spawn();

        // 3. 부모는 즉시 종료 (Vim에게 제어권 반납)
    }
}
