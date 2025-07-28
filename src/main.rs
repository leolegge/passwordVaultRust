use std::io;
use std::fs;
use std::fs::File;
use age::secrecy::Secret;
use age::armor::ArmoredWriter;
use std::io::{Read, Write};
use age::Encryptor;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter};





struct Vault {
    entries : Vec<Entry>,
}

impl Vault {
    fn new() -> Vault {
        Vault { entries: Vec::new() }
    }
    
    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}


struct Entry{
    identifier: Vec<u8>,
    password: Vec<u8>,
}
impl Entry{
    fn new(identifier: Vec<u8>, password: Vec<u8>) -> Entry{
        Entry{identifier, password }
    }
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
            1 => {
                println!("Please select desired vault to add to or read from");
                //TODO implement function here to find vaults in the projects directory
                get_all_vaults();
            }
            2 => {
                println!("Please select new name for new vault");
                //TODO implement function that allows creation of a new vault file
                let mut new_name = String::new();
                
                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read line");
                
                let new_vault = Vault::new();
                
                
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

pub fn get_all_vaults(){
    //TODO example code for getting all directories in root
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

pub fn save_vault_to_file(vault: Vault ,passphrase: &str ,filename: &str) -> Result<(), Box<dyn std::error::Error>> {

    let vault_path = std::env::current_exe()?.parent().unwrap().to_path_buf().join(filename);

    

    Ok(())
}

