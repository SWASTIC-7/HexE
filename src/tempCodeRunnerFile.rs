    let mut file = File::open("program.asm").expect("Failed to open the file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("{buffer}");
    Ok(())