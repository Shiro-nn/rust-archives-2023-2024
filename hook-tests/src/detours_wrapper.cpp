#include <Windows.h>
#include <detours.h>
#include <iostream>

using CreateCompatibleBitmap_t = HBITMAP(WINAPI*)(HDC, int, int);
CreateCompatibleBitmap_t OriginalCreateCompatibleBitmap = nullptr;

// Тип для коллбэка из Rust
extern "C" typedef HBITMAP (WINAPI *RustCallback)(HDC, int, int);
static RustCallback rust_callback = nullptr;

// Перехваченная функция
HBITMAP WINAPI HookedCreateCompatibleBitmap(HDC hdc, int cx, int cy) {
    if (rust_callback) {
        return rust_callback(hdc, cx, cy);
    }
    return OriginalCreateCompatibleBitmap(hdc, cx, cy);
}

extern "C" __declspec(dllexport) void InstallHook(RustCallback callback) {
    rust_callback = callback;
    OriginalCreateCompatibleBitmap = ::CreateCompatibleBitmap;

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)OriginalCreateCompatibleBitmap, HookedCreateCompatibleBitmap);
    DetourTransactionCommit();
}

extern "C" __declspec(dllexport) void UninstallHook() {
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourDetach(&(PVOID&)OriginalCreateCompatibleBitmap, HookedCreateCompatibleBitmap);
    DetourTransactionCommit();
}