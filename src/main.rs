use std::io;
use std::fs;
use std::fs::File;
use age::{Decryptor, Encryptor};
use serde::{Serialize, Deserialize};
use secrecy::{Secret};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

enum VaultOption {
    EnterVault,
    CreateVault,
    DeleteVault,
    Exit,
    InvalidInput
}

impl VaultOption {
    fn from_usize(number: usize) -> Option<Self> {
        match number{
            1 => Some(VaultOption::EnterVault),
            2 => Some(VaultOption::CreateVault),
            3 => Some(VaultOption::DeleteVault),
            4 => Some(VaultOption::Exit),
            _ => Some(VaultOption::InvalidInput)
        }
    }
}

enum InteriorVaultOption{
    AddEntry,
    ViewEntries,
    DeleteEntry,
    ExitSave,
}

impl InteriorVaultOption {
    fn from_usize(number: usize) -> Option<Self> {
        match number {
            1 => Some(InteriorVaultOption::AddEntry),
            2 => Some(InteriorVaultOption::ViewEntries),
            3 => Some(InteriorVaultOption::DeleteEntry),
            4 => Some(InteriorVaultOption::ExitSave),
            _ => None
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
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
    
    fn remove_entry(&mut self, entry_remove: usize) {
        self.entries.remove(entry_remove);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry{
    identifier: String,
    password: String,
}
impl Entry{
    fn new(identifier: String, password: String) -> Entry{
        Entry{identifier, password }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>>  {
    
    println!("Welcome to the password vault accessor system");
    //main menu system
    loop{
        println!("Please select an option \n\
        1.Enter vault to use\n\
        2.Create new vault\n\
        3.Delete a vault\n\
        4.Exit password vault");
        
        let mut option = String::new();
        
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        
        let option : usize =  match option.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let option: VaultOption = match VaultOption::from_usize(option){
            Some(option) => option,
            None => continue,
        };
        
        match option {
            VaultOption::EnterVault => {
                println!("Please select desired vault to add to or read from");
                let vaults = match get_all_vaults(){
                    Ok(vaults) => {
                        let mut vault_number = 1;
                        for vault in &vaults {
                            println!("{}. {}", vault_number,vault.file_name().unwrap().to_str().unwrap());
                            vault_number += 1;
                        }
                        vaults
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                
                let mut vault_option = String::new();
                
                io::stdin().read_line(
                    &mut vault_option)
                    .expect("Failed to read line");
                
                let vault_option :usize = match vault_option.trim().parse(){
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let vault_option = match vault_option.checked_sub(1) {
                    Some(idx) if idx < vaults.len() => idx,  // Valid index case
                    _ => continue,  // Handles both subtraction underflow and invalid index
                };

                let selected_vault = match vaults.get(vault_option) {
                    Some(vault) => vault,
                    None => continue,
                };


                println!("You selected {}",  selected_vault.file_name().unwrap().to_str().unwrap());
                
                let passphrase = Secret::new(
                    rpassword::prompt_password("Enter passphrase for this vault: ")
                        .expect("Failed to read passphrase")
                );
                
                let mut loaded_vault = match load_vault_from_file(selected_vault, &passphrase){
                    Ok(vault) => {
                        println!("Vault successfully loaded to memory");
                        vault
                    },
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                
                loop {
                    println!("What would you like to do with you vault:\n\
                            1.Add new entry\n\
                            2.View all entries\n\
                            3.Delete entry\n\
                            4.Exit and save current vault");
                    let mut entry_option = String::new();
                    
                    io::stdin()
                        .read_line(&mut entry_option)
                        .expect("Failed to read line");

                    let entry_option : usize =  match entry_option.trim().parse(){
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                    
                    let entry_option : InteriorVaultOption = match InteriorVaultOption::from_usize(entry_option){
                        Some(option) => option,
                        None  => continue,
                    };
                    
                    match entry_option {
                        InteriorVaultOption::AddEntry => add_new_entry(&mut loaded_vault),
                        InteriorVaultOption::ViewEntries => view_entries(&loaded_vault),
                        InteriorVaultOption::DeleteEntry => {
                            println!("Select entry to remove");
                            
                            view_entries(&loaded_vault);
                            
                            let mut entry_deletion_option = String::new();
                            io::stdin().read_line(&mut entry_deletion_option).expect("Failed to read line");


                            let entry_deletion_option : usize =  match entry_deletion_option.trim().parse(){
                                Ok(num) => num,
                                Err(_) => continue,
                            };
                            
                            match entry_deletion_option.checked_sub(1) {
                                Some(idx) if idx < loaded_vault.entries.len() => {
                                    loaded_vault.remove_entry(idx);
                                }
                                _ => continue
                            }
                        }
                        InteriorVaultOption::ExitSave => {
                            save_vault_to_file(&loaded_vault, passphrase , selected_vault.file_name().unwrap().to_str().unwrap())?;
                            break
                        },
                    }
                }
            }
            VaultOption::CreateVault => {
                println!("Please select name for new vault");
                let mut new_name = String::new();
                
                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read line");
                
                let new_vault = Vault::new();

                let passphrase = Secret::new(
                    rpassword::prompt_password("Enter passphrase for your new vault: ")
                        .expect("Failed to read passphrase")
                );
                save_vault_to_file(&new_vault, passphrase, &new_name)?;
            }
            VaultOption::DeleteVault => {
                println!("Please select desired vault to delete:");
                let vaults = match get_all_vaults(){
                    Ok(vaults) => {
                        let mut vault_number = 1;
                        for vault in &vaults {
                            println!("{}. {}", vault_number,vault.file_name().unwrap().to_str().unwrap());
                            vault_number += 1;
                        }
                        vaults
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                let mut vault_option = String::new();

                io::stdin().read_line(
                    &mut vault_option)
                    .expect("Failed to read line");

                let vault_option :usize = match vault_option.trim().parse(){
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let vault_option = match vault_option.checked_sub(1) {
                    Some(idx) if idx < vaults.len() => idx,  // Valid index case
                    _ => continue,  // Handles both subtraction underflow and invalid index
                };

                let selected_vault = match vaults.get(vault_option) {
                    Some(vault) => vault,
                    None => continue,
                };
                
                println!("You selected to delete {}",  selected_vault.file_name().unwrap().to_str().unwrap());
                
                delete_vault(selected_vault);
            }
            VaultOption::Exit => {
                break
            }
            _ => {
                continue
            }
        }
        
        
        
    }
    
    
    Ok(())
}


fn get_all_vaults() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let mut age_files = Vec::new();

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if file has `.age` extension
        if path.extension().and_then(|ext| ext.to_str()) == Some("age") {
            age_files.push(path);
        }
    }
    
    if age_files.len() > 0 {
        return Ok(age_files)
    };

    Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "No vaults found")))
}


fn save_vault_to_file(
    vault: &impl Serialize,
    passphrase: Secret<String>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let sanitized_filename = filename.trim();
    let vault_path = Path::new(&sanitized_filename).with_extension("age");
    let serialized_data = serde_json::to_vec(vault)?;
    let encrypted_file = File::create(&vault_path)?;
    let encryptor = Encryptor::with_user_passphrase(passphrase);
    let mut writer = encryptor.wrap_output(encrypted_file)?;
    writer.write_all(&serialized_data)?;
    writer.finish()?;
    println!("Vault saved to: {}", vault_path.display());
    Ok(())
}


fn load_vault_from_file(file_path: &PathBuf, passphrase: &Secret<String> ) -> Result<Vault, Box<dyn std::error::Error>> {
    let encrypted_file = File::open(&file_path)?;
    
    let decryptor = match Decryptor::new(encrypted_file)? {
        Decryptor::Passphrase(d) => d,
        _ => return Err("File is not passphrase encrypted".into()),
    };
    
    let mut reader = decryptor.decrypt(&passphrase, None)?;
    
    let mut decrypted_data = Vec::new();
    reader.read_to_end(&mut decrypted_data)?;
    
    let vault = serde_json::from_slice(&decrypted_data)?;

    Ok(vault)
}

fn add_new_entry(vault: &mut Vault){
    println!("Type in the identifier for your new entry:");
    let mut identifier = String::new();
    io::stdin().read_line(&mut identifier).expect("Failed to read line");
    println!("Enter the password relating to your new entry:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line"); 
    
    vault.add_entry(Entry::new(identifier.trim().to_string(), password.trim().to_string()));
    println!("Added new entry!");
}

fn view_entries(vault: &Vault){
    let mut entry_number: u8 = 1;
    for entry in vault.entries.iter() {
        println!("{}. Identifier: {} - Password: {}\n",entry_number, entry.identifier, entry.password);
        entry_number += 1;
    }
}

fn delete_vault(vault: &PathBuf){
    
    println!("Type 'confirm' to delete vault:");
    
    let mut confirm = String::new();
    
    io::stdin().read_line(&mut confirm).expect("Failed to read line");
    
    match confirm .trim() {
        "confirm" => {
            match fs::remove_file(vault) {
                Ok(()) => println!("File deleted successfully!"),
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => println!("File not found!"),
                    _ => println!("Error deleting file: {}", e),
                },
            }
        }
        _ => {
            println!("Invalid confirm, ABORTING")
        }
    }
}


 
