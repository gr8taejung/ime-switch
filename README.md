# ime-select

> **VSCode Vim IME 자동 전환 도구**

`ime-select`는 Windows 환경의 Vim(VSCodeVim, NeoVim 등) 사용자를 위해 Rust로 작성된 CLI 도구입니다. 
Win32 API를 직접 호출하여 가장 확실하고 빠르게 한/영 입력 상태를 전환합니다.
- VSCode VIM에서 제공하는 im-select는 입력기 자체를 전환하는 기능을 제공하지만,
  Microsoft IME 하나만 사용하여 한/영 전환을 하는 사용자에게는 적합하지 않습니다.
- ime-select는 im을 전환하지 않고, 입력 언어만 변경하도록 개발 되었습니다.

## ✨ 주요 특징
- 외부 의존성 없이 Win32 API(`SendMessageW`)를 직접 호출하며, VSCode VIM의 옵션을 적용 가능합니다.

- **영문 전환(0)**: 단 1회의 시스템 호출로 즉시 전환 (Esc 지연 최소화).
- **한글 복귀(1)**: 입력기 활성화 및 한글 모드 강제 적용(Native Mode).

## 🚀 설치 방법

1. [Releases](../../releases) 페이지에서 최신 `ime-select.exe`를 다운로드합니다.
2. 원하는 경로(예: `C:\tools\`)에 파일을 배치합니다.

## 🛠 사용법

명령행 인자에 따라 다음과 같이 동작합니다.

| 명령어 | 동작 | 설명 |
| :--- | :--- | :--- |
| `ime-select.exe` | **조회(Obtain)** | 현재 IME 상태 출력 (0: 영문, 1: 한글) |
| `ime-select.exe 0` | **영문 전환** | IME를 닫아 즉시 영문 모드로 전환 |
| `ime-select.exe 1` | **한글 전환** | IME를 열고 한글(Native) 모드로 전환 |

## ⚙️ 설정 (VSCodeVim)

VSCode의 `settings.json`에 아래 설정을 추가하세요.

### 1. 한글 복귀 기능을 포함할 때 (표준)
`Esc`를 누르면 영문으로 바뀌고, 다시 `i`를 눌러 입력 모드로 갈 때 이전 상태(한글)를 복구합니다.
```json
{
    "vim.autoSwitchInputMethod.enable": true,
    "vim.autoSwitchInputMethod.defaultIM": "0",
    "vim.autoSwitchInputMethod.obtainIMCmd": "C:\\path\\to\\ime-select.exe",
    "vim.autoSwitchInputMethod.switchIMCmd": "C:\\path\\to\\ime-select.exe {im}"
}
```

### 2. 영문 변경 기능만을 사용할 때 (선택)
`Esc`를 누르면 영문으로 바뀌고, 이전 상태(한글) 복구는 사용하지 않습니다.
```json
{
    "vim.autoSwitchInputMethod.enable": true,
    "vim.autoSwitchInputMethod.defaultIM": "0",
    "vim.autoSwitchInputMethod.obtainIMCmd": "",
    "vim.autoSwitchInputMethod.switchIMCmd": "C:\\path\\to\\ime-select.exe {im}"
}
