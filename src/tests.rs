use crypt::Crypt;


#[test]
fn encode_64() {
    assert_eq!(Crypt::encode_64("Lorem ipsum dolor sit amet."), "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQu")
}
#[test]
fn decode_64() {
    assert_eq!(Crypt::decode_64("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQu"), "Lorem ipsum dolor sit amet.");
}

#[test]
fn encode_16() {
    assert_eq!(Crypt::encode_16("Lorem ipsum dolor sit amet."), "4c6f72656d20697073756d20646f6c6f722073697420616d65742e")
}

#[test]
fn decode_16_() {
    assert_eq!(Crypt::decode_16("4c6f72656d20697073756d20646f6c6f722073697420616d65742e"), "Lorem ipsum dolor sit amet.")
}

#[test]
fn encode_rot() {
    assert_eq!(Crypt::encode_rot("Lorem ipsum dolor sit amet."), "Yberz vcfhz qbybe fvg nzrg.")
}

#[test]
fn decode_rot() {
    assert_eq!(Crypt::decode_rot("Yberz vcfhz qbybe fvg nzrg."), "Lorem ipsum dolor sit amet.")
}