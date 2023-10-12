use rand::{thread_rng, Rng};

fn main() {

    let HexBytes = [0x61u8, 0x62u8, 0x63u8, 0x64u8, 0x65u8, 0x66u8, 0x67u8, 0x68u8, 0x69u8, 0x6Au8, 0x6Bu8, 0x6Cu8, 0x6Du8, 0x6Eu8, 0x6Fu8, 0x70u8, 0x71u8, 0x72u8, 0x73u8, 0x74u8, 0x75u8, 0x76u8, 0x77u8, 0x78u8, 0x79u8, 0x7Au8, 0x21u8, 0x23u8, 0x25u8, 0x26u8, 0x2Au8, 0x3Fu8, 0x5Bu8, 0x5Du8];

    // Byte = bit = 8 

    let BinBytes = [0b01100001u8, 0b01100010u8, 0b01100011u8, 0b01100100u8, 0b01100101u8, 0b01100110u8, 0b01100111u8, 0b01101000u8, 0b01101001u8, 0b01101010u8, 0b01101011u8, 0b01101100u8, 0b01101101u8, 0b01101110u8, 0b01101111u8, 0b01110000u8, 0b01110001u8, 0b01110010u8, 0b01110011u8, 0b01110100u8, 0b01110101u8, 0b01110110u8, 0b01110111u8, 0b01111000u8, 0b01111001u8, 0b01111010u8, 0b00100001u8, 0b00100011u8, 0b00100101u8, 0b00100110u8, 0b00101010u8, 0b00111111u8, 0b01011011u8, 0b01011101u8];
    
    // Bin = bit = 8 

    
  
    keyCreate(20, &HexBytes);
}   
fn keyCreate(tam: i32 , bin_or_hex: &[u8]){

    let mut gen = thread_rng();

    let result = (0..tam).map(|_|{

        let index = gen.gen_range(0..bin_or_hex.len());

        bin_or_hex[index] as char
    }).collect::<String>();

    println!("{result}");

}