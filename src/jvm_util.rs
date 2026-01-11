use std::ffi::CStr;

use jni_simple::{JNI_GetCreatedJavaVMs_first, JNIEnv, JavaVM, jobject};


// find jvm. i use this helper method suffixed _first(), hotspot enforces either 0 or 1 so its ok? i think
pub unsafe fn get_jvm() -> JavaVM {
    unsafe {
        match JNI_GetCreatedJavaVMs_first() {
            Ok(option_vm) => {
                match option_vm {
                    Some(vm) => vm,
                    None => {
                        panic!("Couldn't find JVM to attach to!");
                    }
                }
            },
            Err(err) => {
                panic!("Couldn't find JVM to attach to! {}", err);
            }
        }
    }
}


// dump fields in class
pub unsafe fn dump_fields(env: &JNIEnv, obj: jobject) {
    println!("|    Field Name    |                 Field Type                 |");
    println!("| ---------------- | ------------------------------------------ |");

    unsafe {
        let class_cls = env.FindClass("java/lang/Class");
        let field_cls = env.FindClass("java/lang/reflect/Field");

        let get_declared_fields = env.GetMethodID(class_cls, "getDeclaredFields", "()[Ljava/lang/reflect/Field;");
        let class_get_name = env.GetMethodID(class_cls, "getName", "()Ljava/lang/String;");

        let get_name = env.GetMethodID(field_cls, "getName", "()Ljava/lang/String;");
        let get_type = env.GetMethodID(field_cls, "getType", "()Ljava/lang/Class;");
        let set_accessible = env.GetMethodID(field_cls, "setAccessible", "(Z)V");

        let obj_cls = env.GetObjectClass(obj);
        let fields = env.CallObjectMethod0(obj_cls, get_declared_fields);
        let len = env.GetArrayLength(fields.into());

        for i in 0..len {
            let field = env.GetObjectArrayElement(fields.into(), i);
            env.CallVoidMethod1(field, set_accessible, true);

            let name = {
                let jstr = env.CallObjectMethod0(field, get_name);
                CStr::from_ptr(env.GetStringUTFChars(jstr.into(), &mut false))
                    .to_string_lossy()
                    .into_owned()
            };

            let type_name = {
                let cls = env.CallObjectMethod0(field, get_type);
                let jstr = env.CallObjectMethod0(cls, class_get_name);
                CStr::from_ptr(env.GetStringUTFChars(jstr.into(), &mut false))
                    .to_string_lossy()
                    .into_owned()
            };



            println!(
                "| {}{} | {}{} |",
                name,
                " ".repeat(16 - (name.len().min(16))),
                type_name,
                " ".repeat(42 - (type_name.len().min(42))),
            );
        }
    }
}
