#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use pgp::Signature;
use pgp::types::Version;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    version_raw: u8,
    data: &'a [u8],
}

impl<'a> FuzzInput<'a> {
    fn get_version(&self) -> Version {
        if self.version_raw % 2 == 0 {
            Version::Old
        } else {
            Version::New
        }
    }
}

fuzz_target!(|input: FuzzInput| {
    let _ = Signature::from_slice(input.get_version(), input.data);
});
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: pgp 
//  - Crate Link: https://github.com/rpgp/rpgp 
//  - Crate Version: 0.15.0 
//  - From Crate: crate_batch_4 
//  - From Crate Link: unknown_website 
//  - Module Path: pgp::packet::signature::de::(Struct)Signature 
//  - Use Statement: None 
//  - Function Name: from_slice 
//  - Function Usage: // use pgp::de::Deserialize;
// fn run_6() {
//     // info: pgp
//     println!("running pgp");
//     let data = [5, 2, 2, 11, 0, 2, 0, 0];
//     let _ = pgp::Signature::from_slice(pgp::types::Version::New, &data);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(Version, &[u8]) -> Result<Signature, Error>,
//   type_fields: [pgp::Version, pgp::Signature, pgp::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(Version, &[u8]) -> Result<Signature, Error>,
//   type_fields: [pgp::Version, pgp::Signature, pgp::Error] 
// }
// Struct construction metadata: {
//   type_name: pgp::Version,
//   type_fields: [Old, New] 
// }
// Struct construction metadata: {
//   type_name: pgp::Signature,
//   type_fields: [pgp::Version, pgp::SignatureConfig, [u8; 2], pgp::SignatureBytes] 
// }
// Struct construction metadata: {
//   type_name: pgp::SignatureConfig,
//   type_fields: [pgp::SignatureType, pgp::PublicKeyAlgorithm, pgp::HashAlgorithm, alloc::Vec, alloc::Vec, pgp::SignatureVersionSpecific] 
// }
// Struct construction metadata: {
//   type_name: pgp::SignatureType,
//   type_fields: [Binary, Text, Standalone, CertGeneric, CertPersona, CertCasual, CertPositive, SubkeyBinding, KeyBinding, Key, KeyRevocation, SubkeyRevocation, CertRevocation, Timestamp, ThirdParty, Other(u8)] 
// }
// Struct construction metadata: {
//   type_name: pgp::PublicKeyAlgorithm,
//   type_fields: [RSA, RSAEncrypt, RSASign, ElgamalSign, DSA, ECDH, ECDSA, Elgamal, DiffieHellman, EdDSALegacy, X25519, X448, Ed25519, Ed448, Private100, Private101, Private102, Private103, Private104, Private105, Private106, Private107, Private108, Private109, Private110, Unknown(u8)] 
// }
// Struct construction metadata: {
//   type_name: pgp::HashAlgorithm,
//   type_fields: [None, MD5, SHA1, RIPEMD160, SHA2_256, SHA2_384, SHA2_512, SHA2_224, SHA3_256, SHA3_512, Private10, Other(u8)] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [pdf::XRef, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: pdf::XRef,
//   type_fields: [Free, Raw, Stream, Promised, Invalid] 
// }
// Struct construction metadata: {
//   type_name: pgp::SignatureVersionSpecific,
//   type_fields: [V2, V3, V4, V6] 
// }
// Struct construction metadata: {
//   type_name: pgp::SignatureBytes,
//   type_fields: [Mpis(Vec<Mpi>), Native(Vec<u8>)] 
// }
// Struct construction metadata: {
//   type_name: pgp::Error,
//   type_fields: [ParsingError(nom::error::ErrorKind), InvalidInput, Incomplete(nom::Needed), InvalidArmorWrappers, InvalidChecksum, Base64DecodeError(base64::DecodeError), RequestedSizeTooLarge, NoMatchingPacket, TooManyPackets, RSAError(rsa::errors::Error), EllipticCurve(elliptic_curve::Error), IOError(std::io::Error), MissingPackets, InvalidKeyLength, BlockMode, MissingKey, CfbInvalidKeyIvLength, Unimplemented(String), Unsupported(String), Message(String), PacketError(nom::error::ErrorKind), PacketIncomplete, UnpadError, PadError, Utf8Error(std::str::Utf8Error), ParseIntError(std::num::ParseIntError), InvalidPacketContent(Box<Error>), SignatureError(SignatureError), MdcError, TryFromInt(TryFromIntError), Gcm, Eax, Ocb, Sha1HashCollision, AesKek(aes_kw::Error)] 
// }

