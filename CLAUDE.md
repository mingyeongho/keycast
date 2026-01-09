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
- **Library name**: `keycast_lib` (suffixed to avoid Windows naming conflicts)
- **Plugins**: `tauri-plugin-opener` for opening URLs/files
- **Build script**: `src-tauri/build.rs`

### Tauri Commands
Rust functions are exposed to the frontend using the `#[tauri::command]` macro. These are registered in `src-tauri/src/lib.rs` via `invoke_handler`. Example command: `greet(name: &str) -> String`

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

1. Define Rust command in `src-tauri/src/lib.rs` with `#[tauri::command]`
2. Register command in `.invoke_handler(tauri::generate_handler![command_name])`
3. Import `invoke` from `@tauri-apps/api/core` in React components
4. Call via `await invoke('command_name', { args })`

## TypeScript Configuration

The project uses strict TypeScript settings:
- `strict: true`
- `noUnusedLocals: true`
- `noUnusedParameters: true`
- `noFallthroughCasesInSwitch: true`

## Build Process

Frontend build outputs to `dist/`, which Tauri bundles as `frontendDist` in the final application.

## UI/UX Architecture

### Transparent Window Setup

The application implements a Raycast-style transparent UI with the following configuration:

**Tauri Configuration (`tauri.conf.json`):**
- `"label": "main"` - Required for programmatic window access
- `"transparent": true` - Enables window transparency
- `"decorations": false` - Removes OS window frame
- `"center": true` - Centers window on screen
- `"resizable": false` - Fixed size window
- `"macOSPrivateApi": true` - Enables macOS-specific features

**CSS Implementation:**
- Use `backdrop-filter: blur()` for background blur effects
- Prefix with `-webkit-backdrop-filter` for WebKit compatibility
- Semi-transparent backgrounds with `bg-{color}/{opacity}` in Tailwind
- `data-tauri-drag-region` attribute on elements to enable window dragging

**Important Notes:**
- Window must have `"label": "main"` in config to be accessed via `get_webview_window("main")`
- CSS `background: transparent` on `html`, `body`, `#root` is required for transparency
- Backdrop blur works best when combined with semi-transparent backgrounds (e.g., `bg-slate-600/90`)
- For advanced macOS blur effects, consider using `window-vibrancy` crate with `NSVisualEffectMaterial`

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
