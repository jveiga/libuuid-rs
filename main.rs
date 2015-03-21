fn main() {
    let uuid = Uuid::generate().unwrap();
    println!("{:?}", uuid);
    println!("{:?}", Uuid::is_null(&uuid));
    let mut null_uuid = Uuid([0u8; 16]);
    println!("{:?}", null_uuid);
    println!("{:?}", Uuid::is_null(&mut null_uuid));
    let mut uuid_to_clean = Uuid::generate().unwrap();
    Uuid::clear(&mut uuid_to_clean);
    println!("clean {:?}", uuid_to_clean);
}

