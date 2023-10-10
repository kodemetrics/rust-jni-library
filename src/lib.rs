#[allow(dead_code, unused, warnings)]
#[allow(warnings)]
extern crate jni;
mod spu;
use jni::objects::{JClass, JString};
use jni::sys::{jbyteArray, jint, jstring};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_aedc_spu_Demo_helloWorld(_env: JNIEnv, _class: JClass) {
    println!("Hello World! From Rust");
}

#[no_mangle]
pub extern "C" fn Java_com_aedc_spu_Demo_encodeImageFromFileToBase64(
    _env: JNIEnv,
    _class: JClass,
    path: JString,
    w: jint,
    h: jint,
) -> jstring {
    let width = w as u32;
    let height = h as u32;
    let input: String = _env.get_string(path).unwrap().into();
    let m_path: &str = &input[..];

    let result = spu::_encode_file_to_base64(m_path, width, height);

    //let greeting = format!("Hello!");
    let output = _env.new_string(result).unwrap();
    output.into_inner()
}

#[no_mangle]
pub extern "C" fn Java_com_aedc_spu_Demo_encodeImageFromURLToBase64(
    _env: JNIEnv,
    _class: JClass,
    path: JString,
    w: jint,
    h: jint,
) -> jstring {
    let width = w as u32;
    let height = h as u32;
    let input: String = _env.get_string(path).unwrap().into();
    let m_path: &str = &input[..];

    let result = spu::_encode_image_fromURLToBase64(m_path, width, height);
    let output = _env.new_string(result).unwrap();
    output.into_inner()
}
