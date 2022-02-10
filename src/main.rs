mod tests;

use std::env;
use std::{io, collections::HashMap, fs};
use std::fs::File;
use blake3::Hash;

fn main() {
    let customupmdllarg = "--backuppath";

    let updll = "UnityPlayer.dll";
    let mut upmdll = "UnityPlayer_Mono.dll";
    let upbdll = "UnityPlayer.bak.dll";
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg.as_str() == customupmdllarg && i + 1 < args.len() {

           upmdll = args[i + 1].as_str();
        }
    }
    let files_names = [updll, upmdll, upbdll];

    let mut hashdict:HashMap<String, Option<Hash>> = HashMap::new();
    for file_name in files_names {
        let file = fs::File::open(file_name);
        let mut hash: Option<Hash> = None;
        if file.is_ok() {
            hash = Some(hash_file(&mut file.unwrap()));
        }
        hashdict.insert(file_name.to_string(), hash);
    }

    if fs::metadata(upmdll).is_ok() && hashdict.get(updll).unwrap() != hashdict.get(upmdll).unwrap()  {
        if fs::metadata(upbdll).is_ok() {
            fs::remove_file(upbdll).ok();
            println!("Deleted {}", upbdll);
        }
        fs::rename(updll, upbdll).ok();
        println!("Renamed {} to {}", updll, upbdll);

        fs::copy(upmdll, updll).ok();
        println!("Copied {} to {}", upmdll, updll);
    }
    else {
        println!("No changes found! Launching game!");
    }
}

fn hash_file(file: &mut File) -> Hash {
    let mut hasher = blake3::Hasher::new();
    io::copy( file, &mut hasher).ok();
    return hasher.finalize();
}

