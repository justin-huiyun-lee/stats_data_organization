fn main() {
    let data = read_file();
    match data {
        Ok(data) => println!("{}", data),
        Err(e) => println!("Error: {}", e),
    }
    println!("{}", data);
}


fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
