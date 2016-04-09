extern crate winapi;
extern crate kernel32;

use std::io::Write;

pub fn clear_line_move_one_up<W>(_: &mut W)
    where W: Write + Send + 'static
{
    let h_console = unsafe { kernel32::GetStdHandle(winapi::winbase::STD_OUTPUT_HANDLE) };
    let mut csbi = winapi::wincon::CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: winapi::wincon::COORD { X: 0, Y: 0 },
        dwCursorPosition: winapi::wincon::COORD { X: 0, Y: 0 },
        wAttributes: 0,
        srWindow: winapi::wincon::SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: winapi::wincon::COORD { X: 0, Y: 0 },
    };

    unsafe { kernel32::GetConsoleScreenBufferInfo(h_console, &mut csbi) };
    let expected_cursor_position = winapi::wincon::COORD {
        X: 0,
        Y: csbi.dwCursorPosition.Y - 1,
    };
    unsafe {
        kernel32::FillConsoleOutputCharacterA(h_console,
                                              ' ' as winapi::winnt::CHAR,
                                              csbi.dwSize.X as winapi::minwindef::DWORD,
                                              expected_cursor_position,
                                              &mut 0)
    };

    unsafe { kernel32::GetConsoleScreenBufferInfo(h_console, &mut csbi) };
    unsafe {
        kernel32::FillConsoleOutputAttribute(h_console,
                                             csbi.wAttributes,
                                             csbi.dwSize.X as winapi::minwindef::DWORD,
                                             expected_cursor_position,
                                             &mut 0)
    };

    unsafe { kernel32::SetConsoleCursorPosition(h_console, expected_cursor_position) };
}
