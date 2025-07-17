use std::io::Error;
use base64::{Engine as _, engine::general_purpose};
use openssl::{encrypt::{Decrypter, Encrypter}, hash::MessageDigest, pkey::PKey, rsa::{Padding, Rsa}};

fn main() {
    let source: &str = "hack the pentagon!!";
    println!("source: {}", source);

    let encrypted = encrypt_string(&source).unwrap_or_default();
    println!("encrypted: {}", &encrypted);

    match decrypt_string(encrypted.as_str()) {
        Ok(result) => println!("decrypted: {}", result),
        Err(err) => println!("Err: {}", err)
    }

    let encrypted = String::from("3wVXeqATU+WjMwl3kZxWl8ORSuLMhqUkBMUliiiGR6Scm7aFUKdag4zpMZO8m8sqrRKIoZ9NEp7ENmeyAE2b1eqm2oKhKsO4JE8xXZCyzr/KS/Dep/e4ATLuwUQkO1Jd9cefb1a3SBgwDEow4/iKAqBkHTsquBZ+BU1L4SynlLoyZydi93nqGcnQXc78wtwc6d8sQmvwLZ85eUCWH56Yh1cvVe+w8y0e49MAlLJMGEQvbCiYoLCKYvgzHOajujo8bCi4zuTsVZ05sRWDss6wMQ2F/kaT6EOhfiVsuWPWer3tdW8h5q0QPRLReRk/pMPDepeUbe2rt6YN8j8hJYp3eN9SjAOQSXwQ4ceM/1vZQF0C3tAdsCuEhQEjdy58oonl+fRBMPD4pEn59h6X32cAcrp5roShzJXEoe9HdQcL91nc+uN44Ru+53Ki42MIpNAqAykJ2FdPY8IT1Om2A8DXFMml7g7DWPA2x2pcw8Uz74Busibdn/84VOiNUL1IdePR2G4Mi1C0jDsKe3MbCdlzNKFi0RsPvNjknp0xuraPGi+ySQJtIfwKTQwx/wRRoi4uoiZ9hJNwLTAfoyHuZzG/uawWzWUZDkICJqF3pqDnSNqTDPH6fetIhjWjc7bF1N3sDEkHMDcXJnz75XsmK12BUswqICjH38fneHsZbM6Pbvc=");
    match decrypt_string(encrypted.as_str()) {
        Ok(result) => println!("decrypted web: {}", result),
        Err(err) => println!("Err: {}", err)
    }

    let encrypted = String::from("haKY4ZIG9DHtWndg179+Owd67kLOC/JEzrx4fyU8YRe8R4ZS6/wVY1Et0Im0OR7CrajYGrF5cK5HIn89rD4fP8ITNqftJ/LFRMbJ7ehWPTMbgG0gD4ESYWZGrgHNr6/OX1U7XQBuvOiJi5xv84YggaQ5ugn116IfPoy/WDidRKJHTJkCtYUxYK+JCKg2OwrNw51rvXrwElMr/99skt3BA+NNI3jgY0hGpEEIMtn54DPnmrHHvY4OYhyxh7HtR3fs0kvXl0Ww+WD8osIfIkSi8AUspLVVGc3hH8uufvvFlQuEUEwanhF8hpMneKI9UxO/yi6dDDvS+wa6SNUi5WfAzQkQUFw7qVVk0f9t0y2wH4xKa6UeojKZLj9fAf4nHI2hszMgs2MmTBnMF5dwY94fHe5i9OPQkcdmdRitl1JO3L4s/wPnzlOYrm4Y7SJ4SCRbXzIOqwmibE6r0BgrvuHBGlqrEAox2gknnYbnUbBY507JixFXY91j0ZGQe5g2A5ixid1kJTB4h8HRdc6bSUdzrTQAwMkvaMebt7duKxjMZDo+UKEr5U+Dd+uWS0lq30ho1ppBNigaELpOEjNmH3kUuJGQXEph5MpyMhmYpFoGyzyqWSin0f3tz1eyg0aEWn54nk9udjymXLDtogksQgrCVMyIkE89sDjmqWqGiid2kGE=");
    match decrypt_string(encrypted.as_str()) {
        Ok(result) => println!("decrypted node: {}", result),
        Err(err) => println!("Err: {}", err)
    }
}


fn encrypt_string(plaintext: &str) -> Result<String, Error> {
    let public_key = include_bytes!("public_key.txt");

    let rsa = Rsa::public_key_from_pem_pkcs1(public_key)?;
    let public_key = PKey::from_rsa(rsa)?;

    let mut encrypter = Encrypter::new(&public_key)?;
    let data = plaintext.as_bytes();

    encrypter.set_rsa_padding(Padding::PKCS1_OAEP)?;
    encrypter.set_rsa_oaep_md(MessageDigest::sha512())?;

    let buffer_len = encrypter.encrypt_len(data)?;
    let mut encrypted = vec![0; buffer_len];

    let encrypted_len = encrypter.encrypt(data, &mut encrypted)?;
    encrypted.truncate(encrypted_len);

    Ok(general_purpose::STANDARD.encode(&encrypted[..encrypted_len]))
}

fn decrypt_string(ciphertext: &str) -> Result<String, Error> {
    let private_key = include_bytes!("private_key.txt");
    let private_pwd = include_bytes!("private_pwd.txt");

    let rsa = Rsa::private_key_from_pem_passphrase(private_key, private_pwd)?;
    let private_key = PKey::from_rsa(rsa)?;

    let mut decrypter = Decrypter::new(&private_key)?;
    let data = match general_purpose::STANDARD.decode(ciphertext) {
        Ok(result) => result,
        Err(err) => return Err(Error::other(
            format!("DecodeError: {}", err)
        ))
    };

    decrypter.set_rsa_padding(Padding::PKCS1_OAEP)?;
    decrypter.set_rsa_oaep_md(MessageDigest::sha512())?;

    let buffer_len = decrypter.decrypt_len(&data)?;
    let mut decrypted = vec![0; buffer_len];

    let decrypted_len = decrypter.decrypt(&data, &mut decrypted)?;
    decrypted.truncate(decrypted_len);

    Ok(String::from_utf8_lossy(&decrypted[..decrypted_len]).to_string())
}

/*
fn encrypt_string(plaintext: &str) -> Result<String, Error> {
    let public_key = include_bytes!("public_key.txt");
    let rsa = Rsa::public_key_from_pem_pkcs1(public_key)?;
    let mut buf = vec![0; rsa.size() as usize];
    let len = rsa.public_encrypt(plaintext.as_bytes(), &mut buf, Padding::PKCS1_OAEP)?;
    Ok(general_purpose::STANDARD.encode(&buf[..len]))
}

fn decrypt_string(ciphertext: &str) -> Result<String, Error> {
    let private_key = include_bytes!("private_key.txt");
    let private_pwd = include_bytes!("private_pwd.txt");
    println!("ciphertext: {}", ciphertext);
    println!("1");
    let rsa = Rsa::private_key_from_pem_passphrase(private_key, private_pwd)?;
    println!("2");
    let mut buf = vec![0; rsa.size() as usize];
    println!("3");
    let len = rsa.private_decrypt(&general_purpose::STANDARD.decode(ciphertext).unwrap(), &mut buf, Padding::PKCS1_OAEP)?;
    println!("4");
    Ok(String::from_utf8_lossy(&buf[..len]).to_string())
}
*/


/*
use std::io::Error;
use base64::{Engine as _, engine::general_purpose};
use ring::{
    aead::{
        Aead, AeadBase, BoundKey, Nonce, OpeningKey, SealingKey,
    },
    rand::{SecureRandom, SystemRandom},
    rand_core::RngCore,
};

fn encrypt_string(plaintext: &str) -> String {
    let public_key = include_bytes!("public_key.txt");
    let mut rng = SystemRandom::new();
    let nonce = {
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes);
        Nonce::assume_unique_for_key(nonce_bytes)
    };
    let sealing_key = SealingKey::new(
        &ring::aead::AES_256_GCM,
        public_key,
    )
    .unwrap();
    let ciphertext = sealing_key.seal_in_place_detached(
        nonce,
        &[],
        plaintext.as_bytes(),
    )
    .unwrap();
    general_purpose::STANDARD.encode(ciphertext)
}

fn decrypt_string(ciphertext: &str) -> String {
    let private_key = include_bytes!("private_key.txt");
    let private_pwd = include_bytes!("private_pwd.txt");
    let ciphertext_bytes = general_purpose::STANDARD.decode(ciphertext).unwrap();
    let nonce = Nonce::assume_unique_for_key(ciphertext_bytes[..12]);
    let opening_key = OpeningKey::new(
        &ring::aead::AES_256_GCM,
        private_key,
        private_pwd,
    )
    .unwrap();
    let plaintext = opening_key
        .open_in_place(&nonce, &[], &mut ciphertext_bytes)
        .unwrap();
    String::from_utf8_lossy(plaintext).to_string()
}
*/






/*
fn main() {
    let pub_bytes = include_str!("public_key.txt");
    let der_bytes: &[u8] = &pem_parser::pem_to_der(pub_bytes);
    let binding = String::from("hello");
    let message_bytes = binding.as_bytes();
    let token = rsa_public_encrypt_pkcs1::encrypt(&der_bytes, &message_bytes);

    match token {
        Ok(result) => {
            let message = String::from_utf8(result).unwrap_or_default();
            println!("{}", message);
        }
        Err(err) => {
            println!("Err: {}", err);
        }
    }
}
*/

/*
use pkcs8::{der::{Encode, Writer}, Document};
use sha3::{Digest, Sha3_512};
use sha2::Sha512;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use pkcs1::RsaPublicKey;
extern crate pkcs1;
*/
/*
fn main() {
    let pub_bytes = include_str!("public_key.txt");
    let der_bytes: &[u8] = &pem_parser::pem_to_der(pub_bytes);
    let binding = String::from("hello");
    let message_bytes = binding.as_bytes();
    let token = rsa_public_encrypt_pkcs1::encrypt(&der_bytes, &message_bytes);

    match token {
        Ok(result) => {
            let message = String::from_utf8(result).unwrap_or_default();
            println!("{}", message);
        }
        Err(err) => {
            println!("Err: {}", err);
        }
    }
}
*/

/*
fn main() {
    let mut rng = rand::thread_rng();

    let pub_bytes = include_str!("public_key.txt");
    let der_bytes: &[u8] = &pem_parser::pem_to_der(pub_bytes);
    let public_key = RsaPublicKey::try_from(der_bytes).unwrap();

    let mut data = b"hello world";
    let encrypted_data = public_key.encode(data);
    

    //println!("{:?}", public.to_public_key().to_public_key_pem(LineEnding::CRLF))
}
*/

/*
fn main3() {
    let priv_key = include_str!("public_key.txt");
    let pub_key = RsaPrivateKey::from_pkcs8_pem(priv_key).unwrap();

    println!("{:?}", pub_key.to_public_key().to_public_key_pem(LineEnding::CRLF))
}
*/

/*
fn main2() {
    println!("Hello, world!");
    println!("u8::MAX: {}", u8::MAX);
    println!("u16::MAX: {}", u16::MAX);
    println!("u32::MAX: {}", u32::MAX);
    println!("u64::MAX: {}", u64::MAX);

    let mut hasher = Sha3_512::new();
    hasher.update(b"hello");
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);
    let base64_hash = STANDARD.encode(&hash);

    println!("sha3-512 hex: {}", hex_hash);
    //assert_eq!(hex_hash, "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043");

    println!("sha3-512 base64: {}", base64_hash);

    let mut hasher = Sha512::new();
    hasher.update(b"hello");
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    println!("Sha2-512 hex: {}", hex_hash);
}
*/