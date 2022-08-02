use windows::{core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*, Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*, Win32::System::Com::*, Win32::System::LibraryLoader::*, Win32::System::Performance::*, Win32::System::SystemInformation::GetLocalTime, Win32::UI::Animation::*, Win32::UI::WindowsAndMessaging::*};

/*use windows::Win32::System::Console::GetStdHandle;
use windows::Win32::System::Console::STD_OUTPUT_HANDLE;
use windows::Win32::System::Console::CONSOLE_SCREEN_BUFFER_INFO;
use windows::Win32::System::Console::SMALL_RECT;
use windows::Win32::System::Console::COORD;
use windows::Win32::System::Console::CHAR_INFO;
use windows::Win32::System::Console::GetConsoleScreenBufferInfo;
use windows::Win32::System::Console::ScrollConsoleScreenBufferW;
use windows::Win32::System::Console::SetConsoleCursorPosition;*/

use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use std::io::{self, Write};

// the game state

const APP_NAME: &str = "Game of life direct 2d";

const MAX_COLUMN_COUNT: u32 = 825;
const MAX_ROWS_COUNT: u32 = 420;

const MINIMAL_UPDATE_DELAY: u8 = 1;

#[derive(Debug, Copy, Clone)]
struct Cell {
    is_fill: u8,
    position_x: u16,
    position_y: u16,
}
#[derive(Debug)]
struct Cells {
    cells_array: Vec<Vec<Cell>>,
}

impl Cells {
    fn new() -> Self {
        Self {
            cells_array: Vec::new(),
        }
    }

    fn fill_cells_array(&mut self) {
        if self.cells_array.len() == 0 {
            let total_count = MAX_COLUMN_COUNT * MAX_ROWS_COUNT;
            let mut x: u16 = 1;
            let mut y: u16 = 1;

            let mut iter = 0;

            let mut is_fill: u8 = 0;

            let mut rand_rng = rand::thread_rng();

            self.cells_array.push(Vec::new());

            while iter != total_count {
                if rand_rng.gen_range(0..2) == 1 {
                    is_fill = 1;
                } else {
                    is_fill = 0;
                }

                self.cells_array[(y - 1) as usize].push(Cell {
                    is_fill: is_fill,
                    position_x: x,
                    position_y: y,
                });

                if (x as u32) < MAX_COLUMN_COUNT {
                    x += 1;
                } else {
                    self.cells_array.push(Vec::new());
                    x = 1;
                    y += 1;
                }
                iter += 1;
            }
        }
    }
}

struct GameState {
    cells: Cells,
    fps: String,
    is_game_on: bool,
    is_game_over: bool,
}

impl GameState {
    fn new() -> Self {
        let mut new_game_state = Self {
            cells: Cells::new(),
            fps: String::from("fps: 0"),
            is_game_on: true,
            is_game_over: false,
        };

        new_game_state.cells.fill_cells_array();

        new_game_state
    }

    fn cell_status_update(&mut self) {
        let cells_arr_copy = self.cells.cells_array.clone();
        let mut new_cells_array = cells_arr_copy.clone();
    
        for cell_column in cells_arr_copy.iter() {
            for cell in cell_column {
                let mut near_cells: Vec<Cell> = Vec::new();
    
                let cell_position_x: i32 = (cell.position_x - 1) as i32;
                let cell_position_y: i32 = (cell.position_y - 1) as i32;
    
                if cell_position_x - 1 > -1 && cell_position_y - 1 > -1 {
                    near_cells.push(
                        cells_arr_copy[(cell_position_y - 1) as usize][(cell_position_x - 1) as usize],
                    );
                }
    
                if cell_position_y - 1 > -1 && cell_position_x < MAX_COLUMN_COUNT as i32 {
                    near_cells
                        .push(cells_arr_copy[(cell_position_y - 1) as usize][cell_position_x as usize]);
                }
    
                if cell_position_y - 1 > -1 && cell_position_x + 1 < MAX_COLUMN_COUNT as i32 {
                    near_cells.push(
                        cells_arr_copy[(cell_position_y - 1) as usize][(cell_position_x + 1) as usize],
                    );
                }
    
                if cell_position_x - 1 > -1 && cell_position_y < MAX_ROWS_COUNT as i32 {
                    near_cells
                        .push(cells_arr_copy[cell_position_y as usize][(cell_position_x - 1) as usize]);
                }
    
                if cell_position_x + 1 < MAX_COLUMN_COUNT as i32
                    && cell_position_y < MAX_ROWS_COUNT as i32
                {
                    near_cells
                        .push(cells_arr_copy[cell_position_y as usize][(cell_position_x + 1) as usize]);
                }
    
                if cell_position_x - 1 > -1 && cell_position_y + 1 < MAX_ROWS_COUNT as i32 {
                    near_cells.push(
                        cells_arr_copy[(cell_position_y + 1) as usize][(cell_position_x - 1) as usize],
                    );
                }
    
                if cell_position_y + 1 < MAX_ROWS_COUNT as i32
                    && cell_position_x < MAX_COLUMN_COUNT as i32
                {
                    near_cells
                        .push(cells_arr_copy[(cell_position_y + 1) as usize][cell_position_x as usize]);
                }
    
                if cell_position_x + 1 < MAX_COLUMN_COUNT as i32
                    && cell_position_y + 1 < MAX_ROWS_COUNT as i32
                {
                    near_cells.push(
                        cells_arr_copy[(cell_position_y + 1) as usize][(cell_position_x + 1) as usize],
                    );
                }
    
                let mut count_near_cells = 0;
    
                for curent_near_cell in near_cells {
                    if curent_near_cell.is_fill == 1 {
                        count_near_cells += 1;
                    }
                }
    
                if (cell.is_fill == 0 || cell.is_fill == 2) && count_near_cells == 3 {
                    new_cells_array[cell_position_y as usize][cell_position_x as usize].is_fill = 1;
                } else if cell.is_fill == 1 && (count_near_cells < 2 || count_near_cells > 3) {
                    new_cells_array[cell_position_y as usize][cell_position_x as usize].is_fill = 0;
                }
            }
        }
    
        self.cells.cells_array = new_cells_array;
    }

    fn change_game_state(&mut self) {
        self.is_game_on = !self.is_game_on;
    }

    fn change_game_over_state(&mut self) {
        self.is_game_over = !self.is_game_over;
    }
}

// console functions

/*fn clear_console() {
    unsafe {
        let console_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::default();
        let mut scroll_rect = SMALL_RECT::default();
        let mut scroll_target = COORD::default();
        let fill = CHAR_INFO::default();

        // Get the number of character cells in the current buffer.
        if GetConsoleScreenBufferInfo(console_handle.clone().unwrap(), &mut csbi) == false {
            return;
        }

        // Scroll the rectangle of the entire buffer.
        scroll_rect.Left = 0;
        scroll_rect.Top = 0;
        scroll_rect.Right = csbi.dwSize.X;
        scroll_rect.Bottom = csbi.dwSize.Y;

        // Scroll it upwards off the top of the buffer with a magnitude of the entire height.
        scroll_target.X = 0;
        scroll_target.Y = 0;

        // Do the scroll
        ScrollConsoleScreenBufferW(
            console_handle.clone().unwrap(),
            &scroll_rect,
            &SMALL_RECT::default(),
            scroll_target,
            &fill,
        );

        // Move the cursor to the top left corner too.
        csbi.dwCursorPosition.X = 0;
        csbi.dwCursorPosition.Y = 0;

        SetConsoleCursorPosition(console_handle.clone().unwrap(), csbi.dwCursorPosition);
    }
}*/

// the logic of creating a window

fn create_brush(target: &ID2D1DeviceContext) -> Result<ID2D1SolidColorBrush> {
    let color = D2D1_COLOR_F { r: 0.9, g: 0.8, b: 0.1, a: 1.0 };

    let properties = D2D1_BRUSH_PROPERTIES { opacity: 0.8, transform: Matrix3x2::identity() };

    unsafe { target.CreateSolidColorBrush(&color, &properties) }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, &options) }
}

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES { startCap: D2D1_CAP_STYLE_ROUND, endCap: D2D1_CAP_STYLE_TRIANGLE, ..Default::default() };

    unsafe { factory.CreateStrokeStyle(&props, &[]) }
}

fn create_device_with_type(drive_type: D3D_DRIVER_TYPE) -> Result<ID3D11Device> {
    let mut flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;

    if cfg!(debug_assertions) {
        flags |= D3D11_CREATE_DEVICE_DEBUG;
    }

    let mut device = None;

    unsafe { D3D11CreateDevice(None, drive_type, None, flags, &[], D3D11_SDK_VERSION, &mut device, std::ptr::null_mut(), &mut None).map(|()| device.unwrap()) }
}

fn create_device() -> Result<ID3D11Device> {
    let mut result = create_device_with_type(D3D_DRIVER_TYPE_HARDWARE);

    if let Err(err) = &result {
        if err.code() == DXGI_ERROR_UNSUPPORTED {
            result = create_device_with_type(D3D_DRIVER_TYPE_WARP);
        }
    }

    result
}

fn create_render_target(factory: &ID2D1Factory1, device: &ID3D11Device) -> Result<ID2D1DeviceContext> {
    unsafe {
        let d2device = factory.CreateDevice(&device.cast::<IDXGIDevice>()?)?;

        let target = d2device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?;

        target.SetUnitMode(D2D1_UNIT_MODE_DIPS);

        Ok(target)
    }
}

fn get_dxgi_factory(device: &ID3D11Device) -> Result<IDXGIFactory2> {
    let dxdevice = device.cast::<IDXGIDevice>()?;
    unsafe { dxdevice.GetAdapter()?.GetParent() }
}

fn create_swapchain_bitmap(swapchain: &IDXGISwapChain1, target: &ID2D1DeviceContext) -> Result<()> {
    let surface: IDXGISurface = unsafe { swapchain.GetBuffer(0)? };

    let props = D2D1_BITMAP_PROPERTIES1 {
        pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_IGNORE },
        dpiX: 96.0,
        dpiY: 96.0,
        bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
        colorContext: None,
    };

    unsafe {
        let bitmap = target.CreateBitmapFromDxgiSurface(&surface, &props)?;
        target.SetTarget(&bitmap);
    };

    Ok(())
}

fn create_swapchain(device: &ID3D11Device, window: HWND) -> Result<IDXGISwapChain1> {
    let factory = get_dxgi_factory(device)?;

    let props = DXGI_SWAP_CHAIN_DESC1 {
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        ..Default::default()
    };

    unsafe { factory.CreateSwapChainForHwnd(device, window, &props, std::ptr::null(), None) }
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}

struct Window {
    handle: HWND,
    factory: ID2D1Factory1,
    dxfactory: IDXGIFactory2,
    style: ID2D1StrokeStyle,
    target: Option<ID2D1DeviceContext>,
    swapchain: Option<IDXGISwapChain1>,
    brush: Option<ID2D1SolidColorBrush>,
    game_space: Option<ID2D1Bitmap1>,
    dpi: f32,
    visible: bool,
    occlusion: u32,
    game_state: GameState
}

impl Window {
    fn new() -> Result<Self> {
        let factory = create_factory()?;
        let dxfactory: IDXGIFactory2 = unsafe { CreateDXGIFactory1()? };
        let style = create_style(&factory)?;
        let mut dpi = 0.0;
        let mut dpiy = 0.0;
        unsafe { factory.GetDesktopDpi(&mut dpi, &mut dpiy) };

        let game_state = GameState::new();

        Ok(Window {
            handle: HWND(0),
            factory,
            dxfactory,
            style,
            target: None,
            swapchain: None,
            brush: None,
            game_space: None,
            dpi,
            visible: false,
            occlusion: 0,
            game_state
        })
    }

    fn render(&mut self) -> Result<()> {
        if self.target.is_none() {
            let device = create_device()?;
            let target = create_render_target(&self.factory, &device)?;
            unsafe { target.SetDpi(self.dpi, self.dpi) };

            let swapchain = create_swapchain(&device, self.handle)?;
            create_swapchain_bitmap(&swapchain, &target)?;

            self.brush = create_brush(&target).ok();
            self.target = Some(target);
            self.swapchain = Some(swapchain);
            self.create_device_size_resources()?;
        }

        let target = self.target.clone();
        unsafe { target.as_ref().unwrap().BeginDraw() };
        self.draw(target.as_ref().unwrap())?;

        unsafe {
            target.as_ref().unwrap().EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?;
        }

        if let Err(error) = self.present(1, 0) {
            if error.code() == DXGI_STATUS_OCCLUDED {
                self.occlusion = unsafe { self.dxfactory.RegisterOcclusionStatusWindow(self.handle, WM_USER)? };
                self.visible = false;
            } else {
                self.release_device();
            }
        }

        Ok(())
    }

    fn release_device(&mut self) {
        self.target = None;
        self.swapchain = None;
        self.release_device_resources();
    }

    fn release_device_resources(&mut self) {
        self.brush = None;
        self.game_space = None;
    }

    fn present(&self, sync: u32, flags: u32) -> Result<()> {
        unsafe { self.swapchain.as_ref().unwrap().Present(sync, flags).ok() }
    }

    fn draw(&mut self, target: &ID2D1DeviceContext) -> Result<()> {
        let game_space = self.game_space.clone();

        unsafe {
            target.Clear(&D2D1_COLOR_F { r: 0.1, g: 0.1, b: 0.1, a: 1.0 });

            let mut previous = None;
            target.GetTarget(&mut previous);
            target.SetTarget(game_space.as_ref().unwrap());
            target.Clear(std::ptr::null());
            self.draw_cells()?;
            target.SetTarget(previous.as_ref());
            target.SetTransform(&Matrix3x2::translation(5.0, 5.0));

            target.SetTransform(&Matrix3x2::identity());

            target.DrawImage(game_space.as_ref().unwrap(), std::ptr::null(), std::ptr::null(), D2D1_INTERPOLATION_MODE_LINEAR, D2D1_COMPOSITE_MODE_SOURCE_OVER);
        }

        Ok(())
    }

    fn draw_cells(&mut self) -> Result<()> {
        let target = self.target.as_ref().unwrap();
        let brush = self.brush.as_ref().unwrap();

        let size: i32 = 2;
        let mut left_position: i32 = size;
        let mut top_position: i32 = size;
        let mut right_position: i32 = size * 2;
        let mut bottom_position: i32 = size * 2;

        let mut prev_cell_position_y: u16 = 1;

        let mut cells = self.game_state.cells.cells_array.clone();

        for cell_column in self.game_state.cells.cells_array.iter() {
            for cell in cell_column {
                if prev_cell_position_y < cell.position_y {
                    left_position = size;
                    top_position = top_position + size;
                    right_position = size * 2;
                    bottom_position = bottom_position + size;
                }
                let rect = D2D_RECT_F {
                    left: left_position as f32,
                    top: top_position as f32,
                    right: right_position as f32,
                    bottom: bottom_position as f32,
                };
                if cell.is_fill == 1 {
                    unsafe {
                        target.FillRectangle(&rect, brush);
                    }
                } else if cell.is_fill == 0 {
                    cells[(cell.position_y - 1) as usize][(cell.position_x - 1) as usize].is_fill = 2;
                }

                left_position = left_position + size;
                right_position = right_position + size;

                prev_cell_position_y = cell.position_y;
            }
        }

        self.game_state.cells.cells_array = cells;

        Ok(())
    }

    fn create_device_size_resources(&mut self) -> Result<()> {
        let target = self.target.as_ref().unwrap();
        let game_space = self.create_game_space(target)?;
        self.game_space = Some(game_space);

        Ok(())
    }

    fn create_game_space(&self, target: &ID2D1DeviceContext) -> Result<ID2D1Bitmap1> {
        let size_f = unsafe { target.GetSize() };

        let size_u = D2D_SIZE_U { width: (size_f.width * self.dpi / 96.0) as u32, height: (size_f.height * self.dpi / 96.0) as u32 };

        let properties = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
            dpiX: self.dpi,
            dpiY: self.dpi,
            bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET,
            colorContext: None,
        };

        unsafe { target.CreateBitmap2(size_u, std::ptr::null(), 0, &properties) }
    }

    fn resize_swapchain_bitmap(&mut self) -> Result<()> {
        if let Some(target) = &self.target {
            let swapchain = self.swapchain.as_ref().unwrap();
            unsafe { target.SetTarget(None) };

            if unsafe { swapchain.ResizeBuffers(0, 0, 0, DXGI_FORMAT_UNKNOWN, 0).is_ok() } {
                create_swapchain_bitmap(swapchain, target)?;
                self.create_device_size_resources()?;
            } else {
                self.release_device();
            }

            self.render()?;
        }

        Ok(())
    }

    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    let mut ps = PAINTSTRUCT::default();
                    BeginPaint(self.handle, &mut ps);
                    self.render().unwrap();
                    EndPaint(self.handle, &ps);
                    LRESULT(0)
                }
                WM_SIZE => {
                    if wparam.0 != SIZE_MINIMIZED as usize {
                        self.resize_swapchain_bitmap().unwrap();
                    }
                    LRESULT(0)
                }
                WM_DISPLAYCHANGE => {
                    self.render().unwrap();
                    LRESULT(0)
                }
                WM_USER => {
                    if self.present(0, DXGI_PRESENT_TEST).is_ok() {
                        self.dxfactory.UnregisterOcclusionStatus(self.occlusion);
                        self.occlusion = 0;
                        self.visible = true;
                    }
                    LRESULT(0)
                }
                WM_ACTIVATE => {
                    self.visible = true; // TODO: unpack !HIWORD(wparam);
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(self.handle, message, wparam, lparam),
            }
        }
    }

    fn run(&mut self) -> Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None)?;
            debug_assert!(instance.0 != 0);
            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_HAND)?,
                hInstance: instance,
                lpszClassName: window_class,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(WINDOW_EX_STYLE::default(), window_class, s!("Sample Window"), WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, None, None, instance, self as *mut _ as _);

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
            let mut message = MSG::default();

            loop {
                if self.visible {
                    let update_time = Instant::now();

                    self.game_state.cell_status_update();
                    self.render()?;

                    //clear_console();

                    
                    let stdout = io::stdout();
                    let mut handle = io::BufWriter::new(stdout);
                   // writeln!(handle, "as_millis {}", 1000 / (update_time.elapsed().as_millis() as u16));
                    writeln!(handle, "as_millis {}", update_time.elapsed().as_millis() as u16);
                    

                    thread::sleep(Duration::from_millis(MINIMAL_UPDATE_DELAY as u64));

                    //self.game_state.fps = format!("fps: {}", 1000 / update_time.elapsed().as_millis());

                    while PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                        if message.message == WM_QUIT {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                } else {
                    GetMessageA(&mut message, None, 0, 0);

                    if message.message == WM_QUIT {
                        return Ok(());
                    }

                    DispatchMessageA(&message);
                }
            }
        }
    }

    extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLong(window, GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLong(window, GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcA(window, message, wparam, lparam)
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)?;
    }
    let mut window = Window::new()?;
    window.run()
}