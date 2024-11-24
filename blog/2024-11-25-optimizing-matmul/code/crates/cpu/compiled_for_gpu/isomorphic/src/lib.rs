// Including the raw bytes generated shader binary in our rust code. This "bloats" the
// binary, but it also means you don't have to worry about the shader file being
// misplaced or deleted.
include!(concat!(env!("OUT_DIR"), "/shader_binary.rs"));
