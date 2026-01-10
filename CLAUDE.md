# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Keycast is a desktop application built with Tauri v2, React 19, TypeScript, and Tailwind CSS v4. It uses pnpm for package management.

The application features a **Raycast-style transparent UI** with:
- Frameless window design (no OS decorations)
- Semi-transparent background with backdrop blur effects
- Centered window positioning
- macOS-specific visual effects using private APIs

## Architecture

### Frontend (React/TypeScript)
- **Entry point**: `src/main.tsx`
- **Root component**: `src/App.tsx`
- **Build tool**: Vite 7 with React plugin
- **Styling**: Tailwind CSS v4 via Vite plugin
- **Dev server**: Runs on port 1420 (fixed port required by Tauri)
- **HMR**: Configured on port 1421

### Backend (Rust/Tauri)
- **Entry point**: `src-tauri/src/main.rs` (calls `keycast_lib::run()`)
- **Application logic**: `src-tauri/src/lib.rs`
- **Command module**: `src-tauri/src/command.rs` (show/hide commands)
- **Window module**: `src-tauri/src/window.rs` (NSPanel and multi-monitor support)
- **Library name**: `keycast_lib` (suffixed to avoid Windows naming conflicts)
- **Build script**: `src-tauri/build.rs`

**Plugins:**
- `tauri-nspanel` - macOS NSPanel implementation for Spotlight-style panel
- `tauri-plugin-global-shortcut` - Global keyboard shortcut support
- `monitor` (tauri-toolkit) - Multi-monitor cursor detection

**Module Organization:**
- `src-tauri/src/lib.rs`: App initialization, plugin setup, global shortcut handler
- `src-tauri/src/command.rs`: Tauri commands (show/hide panel)
- `src-tauri/src/window.rs`: NSPanel conversion trait and multi-monitor positioning

### Tauri Commands
Rust functions are exposed to the frontend using the `#[tauri::command]` macro. These are registered in `src-tauri/src/lib.rs` via `invoke_handler`.

**Available Commands:**
- `show(app_handle: AppHandle)` - Shows the panel and makes it the key window
- `hide(app_handle: AppHandle)` - Hides the panel if visible

## Development Commands

### Frontend Development
```bash
pnpm dev                    # Start Vite dev server (port 1420)
pnpm build                  # Build TypeScript and frontend assets
pnpm preview                # Preview production build
```

### Tauri Application
```bash
pnpm tauri dev              # Run Tauri app in development mode
pnpm tauri build            # Build production Tauri app
```

Note: `pnpm tauri dev` runs `pnpm dev` automatically (configured in `tauri.conf.json`).

## Key Configuration Files

- `tauri.conf.json`: Tauri app configuration (window size, bundle settings, dev/build commands)
- `vite.config.ts`: Vite configuration with Tauri-specific settings
- `src-tauri/Cargo.toml`: Rust dependencies and crate configuration
- `tsconfig.json`: TypeScript compiler settings (strict mode enabled)
- `package.json`: Node.js dependencies and scripts

## Frontend-Backend Communication

**Command Registration Flow:**
1. Define Rust command in a module (e.g., `src-tauri/src/command.rs`) with `#[tauri::command]`
2. Add command to `.invoke_handler(tauri::generate_handler![command::show, command::hide])` in `lib.rs`
3. Import `invoke` from `@tauri-apps/api/core` in React components
4. Call via `await invoke('command_name', { args })`

**Example:**
```rust
// src-tauri/src/command.rs
#[tauri::command]
pub fn hide(app_handle: AppHandle) {
    let panel = app_handle.get_webview_panel(KEYCAST_LABEL).unwrap();
    if panel.is_visible() {
        panel.hide();
    }
}
```

```typescript
// src/hooks/useEscape.ts
import { invoke } from "@tauri-apps/api/core";

invoke("hide");
```

## TypeScript Configuration

The project uses strict TypeScript settings:
- `strict: true`
- `noUnusedLocals: true`
- `noUnusedParameters: true`
- `noFallthroughCasesInSwitch: true`

## Build Process

Frontend build outputs to `dist/`, which Tauri bundles as `frontendDist` in the final application.

## UI/UX Architecture

### NSPanel-Based Spotlight Panel

The application uses macOS NSPanel (via `tauri-nspanel`) to create a Spotlight-style floating panel with advanced window management.

**Key Architecture Components:**

1. **Panel Conversion** (`src-tauri/src/window.rs:43-86`)
   - `WebviewWindowExt::to_spotlight_panel()` converts standard Tauri window to NSPanel
   - Sets `PanelLevel::Floating` to keep panel above other windows
   - Configures `CollectionBehavior` for full-screen app support and active space tracking
   - Uses `StyleMask::nonactivating_panel()` to prevent app activation in Dock

2. **Multi-Monitor Support** (`src-tauri/src/window.rs:89-120`)
   - `center_at_cursor_monitor()` detects which monitor contains the cursor
   - Automatically centers panel on the active monitor
   - Uses `monitor` crate from tauri-toolkit for cursor detection

3. **Global Shortcut Handler** (`src-tauri/src/lib.rs:18-46`)
   - Listens for `Cmd+Shift+;` (SUPER + SHIFT + Semicolon)
   - Toggles panel visibility
   - Lazy panel conversion: converts window to NSPanel on first activation
   - Automatically centers panel at cursor position before showing

4. **Auto-Hide Behavior** (`src-tauri/src/window.rs:73-82`)
   - Panel automatically hides when it loses key window status (user clicks outside)
   - Implemented via `SpotlightPanelEventHandler::window_did_resign_key()`

5. **ESC Key Handler** (`src/hooks/useEscape.ts`)
   - React hook that calls `hide` command when ESC is pressed
   - Provides keyboard-based dismissal

**Tauri Configuration (`tauri.conf.json`):**
- `"label": "main"` - Required constant (`KEYCAST_LABEL`) used throughout codebase
- `"transparent": true` - Enables window transparency
- `"decorations": false` - Removes OS window frame
- `"center": true` - Initial window positioning
- `"resizable": false` - Fixed size window
- `"macOSPrivateApi": true` - Required for NSPanel features

**Capabilities (`src-tauri/capabilities/desktop.json`):**
- `"global-shortcut:default"` permission required for keyboard shortcuts
- Applied to window labeled "main"

**CSS Implementation:**
- Use `backdrop-filter: blur()` for background blur effects
- Prefix with `-webkit-backdrop-filter` for WebKit compatibility
- Semi-transparent backgrounds with `bg-{color}/{opacity}` in Tailwind
- `data-tauri-drag-region` attribute on elements to enable window dragging

**Accessory Mode:**
- App runs with `ActivationPolicy::Prohibited` on macOS
- Prevents app from appearing in Dock and menu bar
- Acts as a background utility accessible only via global shortcut

## Git Commit Guidelines

This project follows Conventional Commits and all commit messages must be written in Korean.

### Commit Format
```
<type>: <description in Korean>

[optional body in Korean]

[optional footer]
```

### Types
- `feat`: 새로운 기능 추가
- `fix`: 버그 수정
- `docs`: 문서 변경
- `style`: 코드 포맷팅, 세미콜론 누락 등 (코드 변경 없음)
- `refactor`: 코드 리팩토링
- `test`: 테스트 추가 또는 수정
- `chore`: 빌드 프로세스, 보조 도구 변경 등

### Examples
```
feat: 키보드 입력 캡처 기능 추가

fix: 윈도우 포커스 버그 수정

docs: README에 설치 가이드 추가

refactor: 상태 관리 로직 개선
```

## GitHub Issue Templates

This project uses structured issue templates to maintain consistency.

### Available Templates
- **기능 추가 요청** (`.github/ISSUE_TEMPLATE/feature_request.yml`): 새로운 기능 제안
  - Automatically adds `enhancement` label
  - Includes sections: 문제 설명, 제안하는 해결방법, 대안, 추가 정보

- **버그 리포트** (`.github/ISSUE_TEMPLATE/bug_report.yml`): 버그 신고
  - Automatically adds `bug` label
  - Includes sections: 버그 설명, 재현 방법, 예상/실제 동작, 환경 정보

All templates are written in Korean and include validation requirements to ensure quality submissions.
