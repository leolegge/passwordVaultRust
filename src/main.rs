use std::io;
use age::secrecy::Secret;
use std::io::{Read, Write};

struct Vault {
    entries : Vec<Entry>,
}

struct Entry{
    identifier: String,
    password: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>>  {
    
    println!("Welcome to the password vault accessor");
    
    loop{
        println!("Please enter the option you would like to do \n\
        1.Enter password vault to use:\n\
        2.Create new password vault\n\
        3.Exit password vault");
        
        let mut option = String::new();
        
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        
        let option : u8 =  match option.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match option {
            0 => {
                println!("Please select desired vault");
                //TODO implement function here to find vaults in the projects directory
            }
            1 => {
                println!("Please select new name for new vault");
                //TODO implement function that allows creation of a new vault file
            }
            3 => {
                break
            }
            _ => {
                continue
            }
        }
        
        
        
    }
    
    
    
    
    
    
    
    
    
    
    
    
    let plaintext = b"Hello world!";
    let passphrase = "this is not a good passphrase";
    
    let encyrpted = match encrypt_with_passphrase(passphrase, plaintext)
    {
        Ok(data) => data,
        Err(error) => panic!("Encountered error while encrypting: {}", error),
    };
    
    println!("Encrypted array values: {:?}",  &encyrpted);
    
    let decrypted = match decrypt_with_passphrase(passphrase, &encyrpted){
        Ok(data) => data,
        Err(error) => panic!("Encountered error while decrypting: {}", error),
    };
    println!("Decrypted array values: {:?}",  &decrypted);
    
    
    let plaintext_str = String::from_utf8(decrypted)?;
    println!("Decrypted text: {}", plaintext_str);

    
    Ok(())
}

pub fn encrypt_with_passphrase(passphrase: &str, plaintext: &[u8], ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;

    writer.write_all(plaintext)?;
    writer.finish()?; // finalize encryption

    Ok(encrypted)
}

pub fn decrypt_with_passphrase(passphrase: &str, encrypted: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let decryptor = match age::Decryptor::new(&encrypted[..])? {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&Secret::new(passphrase.to_owned()), None)?;
        reader.read_to_end(&mut decrypted)?;

    Ok(decrypted)
}

