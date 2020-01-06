mod crypto;
use crypto::rsa;

fn main() {
    let (public_key, private_key) = rsa::generate_key_pair(61, 53, 17);

    let message = String::from("hello world");

    let encrypt_message = rsa::encrypt(message, public_key);
    println!("encrypt: {}", encrypt_message);

    let decrypt_message = rsa::decrypt(encrypt_message, private_key);
    println!("decrypt: {}", decrypt_message);
}