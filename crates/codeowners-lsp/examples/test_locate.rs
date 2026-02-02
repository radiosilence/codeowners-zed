fn main() {
    let root = std::path::Path::new("/Users/james.cleveland/workspace/radiosilence/codeowners-zed");
    println!("Looking for CODEOWNERS in {:?}", root);
    
    if let Some(path) = codeowners::locate(root) {
        println!("Found CODEOWNERS at {:?}", path);
        let owners = codeowners::from_path(&path);
        println!("Testing src/lib.rs...");
        if let Some(o) = owners.of("src/lib.rs") {
            let strs: Vec<String> = o.iter().map(|x| x.to_string()).collect();
            println!("Owners: {:?}", strs);
        } else {
            println!("No owners found for src/lib.rs");
        }
        println!("Testing crates/codeowners-lsp/src/main.rs...");
        if let Some(o) = owners.of("crates/codeowners-lsp/src/main.rs") {
            let strs: Vec<String> = o.iter().map(|x| x.to_string()).collect();
            println!("Owners: {:?}", strs);
        } else {
            println!("No owners found for crates path");
        }
    } else {
        println!("CODEOWNERS not found!");
    }
}
