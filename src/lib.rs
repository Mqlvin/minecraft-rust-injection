use std::{ptr::{null, null_mut}, thread::{self, sleep}, time::Duration};
use jni_simple::{JNI_VERSION_1_8, JavaVMAttachArgs};
use windows::Win32::System::SystemServices::{DLL_PROCESS_DETACH, DLL_PROCESS_ATTACH};

mod jvm_util;


#[cfg(target_os = "linux")]
// run library_init when library loaded
#[used]
#[cfg_attr(target_os="linux", unsafe(link_section = ".init_array"))]
static INIT: unsafe extern "C" fn() = library_init;


// entry point for library
#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
unsafe extern "C" fn library_init() {
    thread::spawn(|| {
        attach();
    });
}

#[cfg(target_os = "windows")]
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn DllMain(_: usize, dw_reason: u32, _: usize) -> i32 {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            attach()
        },
        _ => ()
    }

    1
}


// so this is the real client
fn attach() {
    unsafe {
        let vm = jvm_util::get_jvm(); // get the minecraft jvm
        let mut attach_args = JavaVMAttachArgs::new(JNI_VERSION_1_8, null(), null_mut());
        let env = vm.AttachCurrentThread(&mut attach_args).unwrap(); // attach to the jvm


        // call Minecraft.getMinecraft()
        let mc = env.FindClass("ave");
        let get_mc = env.GetStaticMethodID(mc, "A", "()Lave;");
        let mc_obj = env.CallStaticObjectMethod0(mc, get_mc);

        // get minecraft game settings
        let game_settings_fid = env.GetFieldID(env.GetObjectClass(mc_obj), "t", "Lavh;");
        let gs_obj = env.GetObjectField(mc_obj, game_settings_fid);

        // dump fields in game settings class (obfuscation maps were being weird)
        // jvm_util::dump_fields(&env, gs_obj);

        // find gamma field
        let gamma_fid = env.GetFieldID(env.GetObjectClass(gs_obj), "aJ", "F");
        println!("Gamma float at addr: {:X}", gamma_fid.addr());

        // loop adjusting gamma
        let mut ticks: f32 = 0.;
        loop {
            let gam = (ticks.sin() * 1.) + 1.;
            env.SetFloatField(gs_obj, gamma_fid, gam); // set gamma
            
            ticks += 0.2;
            sleep(Duration::from_millis(50));
        }


        // let _ = vm.DetachCurrentThread();
    }
}
