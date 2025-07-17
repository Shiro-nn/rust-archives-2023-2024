use std::os::raw::{c_int};
use winapi::shared::windef::{HBITMAP, HDC, HDESK};
use winapi::um::winuser::GetThreadDesktop;

// Объявление внешних функций из C++ обёртки
#[link(name = "detours_wrapper")]
extern "C" {
    fn InstallHook(callback: extern "system" fn(HDC, c_int, c_int) -> HBITMAP);
    fn UninstallHook();
}

// Коллбэк для обработки перехваченного вызова
extern "system" fn hook_handler(hdc: HDC, cx: c_int, cy: c_int) -> HBITMAP {
    println!(
        "CreateCompatibleBitmap перехвачен: cx={}, cy={}",
        cx, cy
    );

    // Вызов оригинальной функции
    unsafe { original_create_compatible_bitmap(hdc, cx, cy) }
}

// Оригинальная функция (объявлена через winapi)
#[link(name = "Gdi32")]
extern "system" {
    fn CreateCompatibleBitmap(hdc: HDESK, cx: c_int, cy: c_int) -> HBITMAP;
}

// Глобальная ссылка на оригинальную функцию
lazy_static::lazy_static! {
    static ref original_create_compatible_bitmap: unsafe extern "system" fn(HDC, c_int, c_int) -> HBITMAP = {
        unsafe { std::mem::transmute(CreateCompatibleBitmap as *const ()) }
    };
}

fn main() {
    unsafe {
        InstallHook(hook_handler);

        // Пример вызова (для демонстрации)
        let hdc = GetThreadDesktop(0); // Получение HDC (пример)
        let _bmp = CreateCompatibleBitmap(hdc, 100, 100);

        // Убрать хук при завершении
        UninstallHook();
    }
}