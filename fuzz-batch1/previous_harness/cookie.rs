#![no_main]
use libfuzzer_sys::fuzz_target;
use cookie::{ CookieJar, Key, };

fuzz_target!(|input: &str| {

    if let Ok(parsed) = cookie::Cookie::parse(input) {
        let c = parsed.into_owned(); // 💡 Own the contents, lift to 'static

        let mut jar = CookieJar::new();
        let key = Key::from(&[0u8; 64]);
        let signed_jar = jar.signed_mut(&key);

        let _ = signed_jar.verify(c); // Now works, Cookie<'static>
    }
});
