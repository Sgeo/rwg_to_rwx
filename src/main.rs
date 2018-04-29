

use std::io::Read;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::c_int;
use std::os::raw::c_long;
use std::ptr;

extern crate libloading as lib;

#[macro_use]
extern crate lazy_static;



lazy_static! {
    static ref RWL20: lib::Library = lib::Library::new("RWL20.dll").expect("Unable to load RWL20.DLL");
    static ref RwOpen: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> c_int> = unsafe { RWL20.get(b"RwOpen\0").expect("Unable to load RwOpen") };
    static ref RwInitialize: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwInitialize\0").expect("Unable to load RwInitialize") };
    static ref RwRelease: lib::Symbol<'static, unsafe extern "stdcall" fn()> = unsafe { RWL20.get(b"RwRelease\0").expect("Unable to load RwRelease") };
    static ref RwClose: lib::Symbol<'static, unsafe extern "stdcall" fn()> = unsafe { RWL20.get(b"RwClose\0").expect("Unable to load RwClose") };
    static ref RwGetError: lib::Symbol<'static, unsafe extern "stdcall" fn() -> c_int> = unsafe { RWL20.get(b"RwGetError\0").expect("Unable to load RwGetError") };
    static ref RwGetInternalError: lib::Symbol<'static, unsafe extern "stdcall" fn() -> c_long> = unsafe { RWL20.get(b"RwGetInternalError\0").expect("Unable to load RwGetInternalError") };
    static ref RwOpenStream: lib::Symbol<'static, unsafe extern "stdcall" fn(c_int, c_int, *mut c_char) -> *mut c_void> = unsafe { RWL20.get(b"RwOpenStream\0").expect("Unable to load RwOpenStream") };
    static ref RwOpenDebugStream: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char) -> c_int> = unsafe { RWL20.get(b"RwOpenDebugStream\0").expect("Unable to load RwOpenDebugStream") };
    static ref RwFindStreamChunk: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, u32) -> c_int> = unsafe { RWL20.get(b"RwFindStreamChunk\0").expect("Unable to load RwFindStreamChunk") };
    static ref RwReadStreamChunkType: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void, *mut u32) -> c_int> = unsafe { RWL20.get(b"RwReadStreamChunkType\0").expect("Unable to load RwReadStreamChunkType") };
    static ref RwSkipStreamChunk: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwSkipStreamChunk\0").expect("Unable to load RwSkipStreamChunk") };
    static ref RwSeekStream: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void, i32) -> c_int> = unsafe { RWL20.get(b"RwSeekStream\0").expect("Unable to load RwSeekStream") };
    static ref RwReadStreamChunk: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void, u32, *mut *mut c_void, u32) -> c_int> = unsafe { RWL20.get(b"RwReadStreamChunk\0").expect("Unable to load RwReadStreamChunk") };
    static ref RwGetDisplayDevices: lib::Symbol<'static, unsafe extern "stdcall" fn() -> *mut c_char> = unsafe { RWL20.get(b"RwGetDisplayDevices\0").expect("Unable to load RwGetDisplayDevices") };
    static ref RwOpenDisplayDevice: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> *mut c_void> = unsafe { RWL20.get(b"RwOpenDisplayDevice\0").expect("Unable to load RwOpenDisplayDevice") };
    static ref RwSetShapePath: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, u32) -> c_int> = unsafe { RWL20.get(b"RwSetShapePath\0").expect("Unable to load RwSetShapePath") };
    static ref RwStartDisplayDevice: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, *mut c_void) -> c_int> = unsafe { RWL20.get(b"RwStartDisplayDevice\0").expect("Unable to load RwStartDisplayDevice") };
    static ref RwCreateRaster: lib::Symbol<'static, unsafe extern "stdcall" fn(i32, i32) -> *mut c_void> = unsafe { RWL20.get(b"RwCreateRaster\0").expect("Unable to load RwCreateRaster") }; 
    static ref RwReadRaster: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, u32) -> *mut c_void> = unsafe { RWL20.get(b"RwReadRaster\0").expect("Unable to load RwReadRaster") }; 
    static ref RwCreateTexture: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> *mut c_void> = unsafe { RWL20.get(b"RwCreateTexture\0").expect("Unable to load RwCreateTexture") };
    static ref RwAddTextureToDict: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> *mut c_void> = unsafe { RWL20.get(b"RwAddTextureToDict\0").expect("Unable to load RwAddTextureToDict") };
    static ref RwGetRasterWidth: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwGetRasterWidth\0").expect("Unable to load RwGetRasterWidth") };
    static ref RwReadNamedTexture: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char) -> *mut c_void> = unsafe { RWL20.get(b"RwReadNamedTexture\0").expect("Unable to load RwReadNamedTexture") };
    static ref RwWriteShape: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> c_int> = unsafe { RWL20.get(b"RwWriteShape\0").expect("Unable to load RwWriteShape") };
}



fn rw_open(device: &str) -> Result<(), c_int> {
    let MSWindows = CString::new(device).expect("Unable to create MSWindows CString").into_raw();
    unsafe {
        if(RwOpen(MSWindows, ptr::null_mut()) == 0) {
            Err(RwGetError())
        } else {
            Ok(())
        }
    }
    
}

fn add_dummy_texture(name: &str) {
    unsafe {
        let texture = RwReadNamedTexture(cs("rustwood"));
        println!("texture: {:?}", texture);
        println!("Error: {}", RwGetError());
        RwAddTextureToDict(CString::new(name).unwrap().into_raw(), texture);
    }
}

fn cs(s: &str) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

fn main() {
    //println!("{:?}", rw_open());
    unsafe {

        RwInitialize(ptr::null_mut());
        println!("Devices: {:?}", CStr::from_ptr(RwGetDisplayDevices()));
        let displayDevice = RwOpenDisplayDevice(cs("rwdl8d20"), ptr::null_mut());
        println!("displayDevice = {:?}", displayDevice);
        println!("start display device = {}", RwStartDisplayDevice(displayDevice, ptr::null_mut()));
        println!("Error: {}", RwGetError());
        RwSetShapePath(cs("."), 1);
        let stream = RwOpenStream(2, 1, cs("counter7_modified.rwg"));
        add_dummy_texture("dai");
        add_dummy_texture("mizo2");
        add_dummy_texture("bwn3");
        println!("stream = {:?}", stream);
        let mut chunkType: u32 = 0;
        let readChunkTypeSuccess = RwReadStreamChunkType(stream, &mut chunkType);
        println!("Success?: {:?}, Type: {:X}", readChunkTypeSuccess, chunkType);
        if chunkType == 0x434C554D { // CLUM
            let mut clump = ptr::null_mut();
            let success = RwReadStreamChunk(stream, 0x434C554D, &mut clump, 0);
            println!("Success: {}, Clump: {:?}", success, clump);
            println!("Error: {}", RwGetError());
            println!("Internal Error: {}", RwGetInternalError());
            RwWriteShape(cs("counter7_modified.rwg.rwx"), clump);
        }

    }
    println!("Hello, world!");
    unsafe {
        RwRelease();
    }
}
