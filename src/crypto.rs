pub mod rsa {
    pub fn generate_key_pair(p: i128, q: i128, e: i128) -> ((i128, i128), (i128, i128)){
        ((p * q, e), (p * q, modular_inverse(e, (p - 1) * (q - 1))))
    }

    pub fn encrypt(message: String, public_key: (i128, i128)) -> String {
        let length = public_key.0.to_string().len();

        let mut result = String::new();
        for ch in message.chars() {
            let encrypt_code = modular_exponentiation(ch as i128, public_key.1, public_key.0);
            let encrypt_message = format!("{:0>fill$}", encrypt_code, fill = length);
            result.push_str(&encrypt_message);
        }

        return result;
    }

    pub fn decrypt(message: String, private_key: (i128, i128)) -> String {
        let length = private_key.0.to_string().len();
        
        let mut result = String::new();
        for i in (0..message.len()).step_by(length) {
            let encrypt_code: i128 = message[i..i + length].parse().unwrap();
            let decrypt_code = modular_exponentiation(encrypt_code, private_key.1, private_key.0);
            let decrypt_message = decrypt_code as u8 as char;
            result.push(decrypt_message);
        }
        
        return result;
    }

    fn modular_inverse(a: i128, b: i128) -> i128 {
        let mut s = vec![1, 0];
        let mut t = vec![0, 1];
        let mut r = vec![a, b];
        let mut q = vec![];
    
        loop {
            let r1 = r[r.len() - 1];
            let r2 = r[r.len() - 2];
            
            q.push(r2 / r1);
            r.push(r2 % r1);
    
            if r[r.len() - 1] == 0 {
                break;
            }
    
            let s1 = s[s.len() - 1];
            let s2 = s[s.len() - 2];
    
            let t1 = t[t.len() - 1];
            let t2 = t[t.len() - 2];
    
            let q1 = q[q.len() - 1];
            s.push(s2 - s1 * q1);
            t.push(t2 - t1 * q1);
        }
    
        if s[s.len() - 1] < 0 {
            return s[s.len() - 1] + b;
        }
        else if s[s.len() - 1] == 0 {
            return 1;
        }
        else {
            return s[s.len() - 1];
        }
    }

    fn modular_exponentiation(a: i128, b: i128, m: i128) -> i128 {
        let binary_exponent = format!("{:b}", b);
        let binary_exponent_digit = binary_exponent.len();
        
        let mut a_exponent_b_mod_m = vec![];
        let mut product = 0;
        for i in 0..binary_exponent_digit {
            if i == 0 {
                a_exponent_b_mod_m.push(a % m);
            }
            else {
                a_exponent_b_mod_m.push(a_exponent_b_mod_m[i - 1] * a_exponent_b_mod_m[i - 1] % m);
            }

            if binary_exponent.chars().nth(binary_exponent_digit - i - 1).unwrap() == '1' {
                if product == 0 {
                    product = a_exponent_b_mod_m[i];
                }
                else {
                    product *= a_exponent_b_mod_m[i];
                }
                product = product % m;
            }
        }

        return product % m;
    }
}