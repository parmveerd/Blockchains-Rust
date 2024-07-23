use a3::block::Block;

fn main() {
    // Nothing is required here, but it may be useful for testing.
    let mut b0 = Block::initial(16);
    // b0.set_proof(56231);
    b0.mine(1);
    let mut b1 = Block::next(&b0, String::from("message"));
    b1.mine(1); // Mine b1 to set the correct proof
    // b1.set_proof(2159);
    let mut b2 = Block::next(&b1, String::from("message2"));
    // b2.set_proof(2159);
    b2.mine(1);

    println!("Hash of b0: {}", b0.hash_string());
    println!("Hash of b1: {}", b1.hash_string());
    println!("Hash of b2: {}", b2.hash_string());


    let mut b0 = Block::initial(16);
    b0.set_proof(56231);
    let mut b1 = Block::next(&b0, String::from("message"));
    b1.set_proof(2159);

    println!("{:02x}", b0.hash_for_proof(56231)); // Output: 6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000
    println!("{:02x}", b1.hash_for_proof(2159)); // Output: 9b4417b36afa6d31c728eed7abc14dd84468fdb055d8f3cbe308b0179df40000

    // let mut b0 = Block::initial(19);
    // b0.set_proof(87745);
    // let mut b1 = Block::next(&b0, String::from("hash example 1234"));
    // b1.set_proof(346082);
    // println!("{}", b0.hash_string());
    // println!("{}", b1.hash_string());

    // b0 = Block::initial(16);
    // println!("{}", b0.is_valid_for_proof(0));
    // println!("{}", b0.is_valid_for_proof(56231));
    // b0.set_proof(56231);
    // println!("{}", b0.is_valid());
    
    // println!("\n");
    // test_mining(); 
//     let mut b0 = Block::initial(7);
//     b0.mine(1);
//     assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:7::385");
//     assert_eq!(
//         format!("{:02x}", b0.hash()),
//         "379bf2fb1a558872f09442a45e300e72f00f03f2c6f4dd29971f67ea4f3d5300"
//     );

//     let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
//     b1.mine(1);
//     assert_eq!(
//         b1.hash_string(),
//         "379bf2fb1a558872f09442a45e300e72f00f03f2c6f4dd29971f67ea4f3d5300:1:7:this is an interesting message:20"
//     );
//     assert_eq!(
//         format!("{:02x}", b1.hash()),
//         "4a1c722d8021346fa2f440d7f0bbaa585e632f68fd20fed812fc944613b92500"
//     );

//     let mut b2 = Block::next(&b1, String::from("this is not interesting"));
//     b2.mine(1);
//     assert_eq!(
//         b2.hash_string(),
//         "4a1c722d8021346fa2f440d7f0bbaa585e632f68fd20fed812fc944613b92500:2:7:this is not interesting:40"
//     );
//     assert_eq!(
//         format!("{:02x}", b2.hash()),
//         "ba2f9bf0f9ec629db726f1a5fe7312eb76270459e3f5bfdc4e213df9e47cd380"
//     );   

// }

// fn test_mining() {
//     let mut b0 = Block::initial(7);
//     b0.mine(1);
//     assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:7::385");
//     assert_eq!(
//         format!("{:02x}", b0.hash()),
//         "379bf2fb1a558872f09442a45e300e72f00f03f2c6f4dd29971f67ea4f3d5300"
//     );

//     let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
//     b1.mine(1);
//     assert_eq!(
//         b1.hash_string(),
//         "379bf2fb1a558872f09442a45e300e72f00f03f2c6f4dd29971f67ea4f3d5300:1:7:this is an interesting message:20"
//     );
//     assert_eq!(
//         format!("{:02x}", b1.hash()),
//         "4a1c722d8021346fa2f440d7f0bbaa585e632f68fd20fed812fc944613b92500"
//     );

//     let mut b2 = Block::next(&b1, String::from("this is not interesting"));
//     b2.mine(1);
//     assert_eq!(
//         b2.hash_string(),
//         "4a1c722d8021346fa2f440d7f0bbaa585e632f68fd20fed812fc944613b92500:2:7:this is not interesting:40"
//     );
//     assert_eq!(
//         format!("{:02x}", b2.hash()),
//         "ba2f9bf0f9ec629db726f1a5fe7312eb76270459e3f5bfdc4e213df9e47cd380"
//     );
}