
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
    static ref RWL20: lib::Library = lib::Library::new("RWL21.dll").expect("Unable to load RWL21.DLL");
    static ref RwOpen: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> c_int> = unsafe { RWL20.get(b"RwOpen\0").expect("Unable to load RwOpen") };
    static ref RwInitialize: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwInitialize\0").expect("Unable to load RwInitialize") };
    static ref RwRelease: lib::Symbol<'static, unsafe extern "stdcall" fn()> = unsafe { RWL20.get(b"RwRelease\0").expect("Unable to load RwRelease") };
    static ref RwClose: lib::Symbol<'static, unsafe extern "stdcall" fn()> = unsafe { RWL20.get(b"RwClose\0").expect("Unable to load RwClose") };
    static ref RwGetError: lib::Symbol<'static, unsafe extern "stdcall" fn() -> c_int> = unsafe { RWL20.get(b"RwGetError\0").expect("Unable to load RwGetError") };
    static ref RwGetInternalError: lib::Symbol<'static, unsafe extern "stdcall" fn() -> c_long> = unsafe { RWL20.get(b"RwGetInternalError\0").expect("Unable to load RwGetInternalError") };
    static ref RwOpenStream: lib::Symbol<'static, unsafe extern "stdcall" fn(c_int, c_int, *mut c_char) -> *mut c_void> = unsafe { RWL20.get(b"RwOpenStream\0").expect("Unable to load RwOpenStream") };
    static ref RwCloseStream: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, *mut c_void) -> i32> = unsafe { RWL20.get(b"RwCloseStream\0").expect("Unable to load RwCloseStream") };
    static ref RwOpenDebugStream: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char) -> c_int> = unsafe { RWL20.get(b"RwOpenDebugStream\0").expect("Unable to load RwOpenDebugStream") };
    static ref RwFindStreamChunk: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, u32) -> c_int> = unsafe { RWL20.get(b"RwFindStreamChunk\0").expect("Unable to load RwFindStreamChunk") };
    static ref RwReadStreamChunkType: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void, *mut u32) -> c_int> = unsafe { RWL20.get(b"RwReadStreamChunkType\0").expect("Unable to load RwReadStreamChunkType") };
    static ref RwSkipStreamChunk: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwSkipStreamChunk\0").expect("Unable to load RwSkipStreamChunk") };
    static ref RwSeekStream: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void, i32) -> c_int> = unsafe { RWL20.get(b"RwSeekStream\0").expect("Unable to load RwSeekStream") };
    static ref RwReadStreamChunk: lib::Symbol<'static, unsafe extern "stdcall" fn (*mut c_void, u32, *mut *mut c_void, u32) -> c_int> = unsafe { RWL20.get(b"RwReadStreamChunk\0").expect("Unable to load RwReadStreamChunk") };
    static ref RwGetDisplayDevices: lib::Symbol<'static, unsafe extern "stdcall" fn() -> *mut c_char> = unsafe { RWL20.get(b"RwGetDisplayDevices\0").expect("Unable to load RwGetDisplayDevices") };
    static ref RwOpenDisplayDevice: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> *mut c_void> = unsafe { RWL20.get(b"RwOpenDisplayDevice\0").expect("Unable to load RwOpenDisplayDevice") };
    static ref RwCloseDisplayDevice: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> i32> = unsafe { RWL20.get(b"RwCloseDisplayDevice\0").expect("Unable to load RwCloseDisplayDevice") };
    static ref RwSetShapePath: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, u32) -> c_int> = unsafe { RWL20.get(b"RwSetShapePath\0").expect("Unable to load RwSetShapePath") };
    static ref RwStartDisplayDevice: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, *mut c_void) -> c_int> = unsafe { RWL20.get(b"RwStartDisplayDevice\0").expect("Unable to load RwStartDisplayDevice") };
    static ref RwCreateRaster: lib::Symbol<'static, unsafe extern "stdcall" fn(i32, i32) -> *mut c_void> = unsafe { RWL20.get(b"RwCreateRaster\0").expect("Unable to load RwCreateRaster") }; 
    static ref RwReadRaster: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, u32) -> *mut c_void> = unsafe { RWL20.get(b"RwReadRaster\0").expect("Unable to load RwReadRaster") }; 
    static ref RwCreateTexture: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> *mut c_void> = unsafe { RWL20.get(b"RwCreateTexture\0").expect("Unable to load RwCreateTexture") };
    static ref RwAddTextureToDict: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> *mut c_void> = unsafe { RWL20.get(b"RwAddTextureToDict\0").expect("Unable to load RwAddTextureToDict") };
    static ref RwGetRasterWidth: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwGetRasterWidth\0").expect("Unable to load RwGetRasterWidth") };
    static ref RwReadNamedTexture: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char) -> *mut c_void> = unsafe { RWL20.get(b"RwReadNamedTexture\0").expect("Unable to load RwReadNamedTexture") };
    static ref RwWriteShape: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_char, *mut c_void) -> c_int> = unsafe { RWL20.get(b"RwWriteShape\0").expect("Unable to load RwWriteShape") };
    static ref RwReadStreamChunkHeader: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwReadStreamChunkHeader\0").expect("Unable to load RwReadStreamChunkHeader") };
    static ref RwReadStream: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, *mut u8, u32) -> c_int> = unsafe { RWL20.get(b"RwReadStream\0").expect("Unable to load RwReadStream") };
    static ref RwGetClumpNumVertices: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void) -> c_int> = unsafe { RWL20.get(b"RwGetClumpNumVertices\0").expect("Unable to load RwGetClumpNumVertices") };
    static ref RwGetClumpVertexUV: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, i32, *mut UV) -> *mut UV> = unsafe { RWL20.get(b"RwGetClumpVertexUV\0").expect("Unable to load RwGetClumpVertexUV") };
}

lazy_static! {
    static ref RwCalculateClumpVertexNormal: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, i32) -> *mut c_void> = unsafe { RWL20.get(b"RwCalculateClumpVertexNormal\0").expect("Unable to load RwCalculateClumpVertexNormal") };
    static ref RwSetClumpVertexUV: lib::Symbol<'static, unsafe extern "stdcall" fn(*mut c_void, i32, f32, f32) -> *mut c_void> = unsafe { RWL20.get(b"RwSetClumpVertexUV\0").expect("Unable to load RwSetClumpVertexUV") };
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct UV {
    u: f32,
    v: f32,
}

impl UV {
    fn new() -> UV {
        UV {
            u: 0.0,
            v: 0.0
        }
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Need a filename!");
    unsafe {

        println!("Open: {}", RwOpen(cs("NullDevice"), ptr::null_mut()));
        println!("Devices: {:?}", CStr::from_ptr(RwGetDisplayDevices()));
        let displayDevice = RwOpenDisplayDevice(cs("rwdl8d21"), ptr::null_mut());
        println!("displayDevice = {:?}", displayDevice);
        println!("start display device = {}", RwStartDisplayDevice(displayDevice, ptr::null_mut()));
        println!("Error: {}", RwGetError());
        RwSetShapePath(cs("."), 1);
        let stream = RwOpenStream(2, 1, cs(&filename));
        println!("stream = {:?}", stream);
        let mut chunkType: u32 = 0;
        RwReadStreamChunkType(stream, &mut chunkType);
        let mut texture_header_data: Vec<u8> = Vec::new();
        if chunkType == 0x5A5A5A5B { // ZZZ[, a header used by Worlds to list textures.
            let header_size = RwReadStreamChunkHeader(stream);
            RwSeekStream(stream, 8);
            texture_header_data = vec![0; (header_size - 8) as usize];
            RwReadStream(stream, texture_header_data.as_mut_ptr(), (header_size - 8) as u32);
            let textures = texture_header_data.split(|&char| char == 0).filter(|&tex| tex.len() > 0).map(String::from_utf8_lossy);
            for texture in textures {
                add_dummy_texture(&texture);
            }
        }
        let mut chunkType: u32 = 0;
        RwReadStreamChunkType(stream, &mut chunkType);
        if chunkType == 0x434C554D { // CLUM
            let mut clump = ptr::null_mut();
            let success = RwReadStreamChunk(stream, 0x434C554D, &mut clump, 0);
            println!("Success loading clump: {}, Clump: {:?}", success, clump);
            let num_vertices = RwGetClumpNumVertices(clump);
            println!("Number of vertices: {}", num_vertices);
            for vertex in 1 .. (num_vertices + 1) {
                let mut uv = UV::new();
                RwGetClumpVertexUV(clump, vertex, &mut uv);
                println!("Vertex {} UV = {:?}", vertex, uv);
                //RwCalculateClumpVertexNormal(clump, vertex);
                RwSetClumpVertexUV(clump, vertex, uv.u, uv.v);
            }
            println!("Error: {}", RwGetError());
            let writeResult = RwWriteShape(cs(&format!("{}.rwx", filename)), clump);
            if writeResult == 1 {
                println!("Successfully wrote .rwx!");
            }
        }
        RwCloseStream(stream, ptr::null_mut());
        RwCloseDisplayDevice(displayDevice);
    }
}
