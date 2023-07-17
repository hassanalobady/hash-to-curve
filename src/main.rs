use bls12_381::{G1Projective,Scalar};
use ff::Field;
use group::{Curve, GroupEncoding};
use sha2::{Digest, Sha256};
use hex::{self, ToHex};


pub fn map_to_curve(msg: &[u8]) -> G1Projective {
  // Hash the input message using SHA-256 to obtain a scalar

 let mut hasher: Sha256 = Sha256::new();

 hasher.update(msg);

  let hash_result: bls12_381_plus::elliptic_curve::generic_array::GenericArray<u8, typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UTerm, typenum::B1>, typenum::B0>, typenum::B0>, typenum::B0>, typenum::B0>, typenum::B0>> = hasher.finalize();

    // Convert the hash result to a fixed-size array of 32 bytes (scalar_bytes)
    let mut scalar_bytes: [u8; 32] = [0u8; 32];
    scalar_bytes.copy_from_slice(&hash_result[..32]);

    // Convert the scalar_bytes to a Scalar value
    let scalar: Scalar = Scalar::from_bytes(&scalar_bytes).unwrap();

  // Generate a point on the curve by multiplying the base point with the scalar

let base_point: G1Projective = G1Projective::generator();

let point_on_curve: G1Projective = base_point * scalar;

point_on_curve
 
}


// Function to generate the commitment C = r*G + x*H
// Generate Pedersen commitment
fn generate_commitment(x: Scalar, r: Scalar, commitment_base: G1Projective) -> G1Projective {
  G1Projective::generator() * r + commitment_base * x
}

// Function to verify the commitment C = r*G + x*H
// Verify the commitment and return true if valid, false otherwise
fn verify_commitment(commitment: G1Projective, x: Scalar, r: Scalar, commitment_base: G1Projective) -> bool {
  let expected_commitment: G1Projective = generate_commitment(x, r, commitment_base);
  commitment == expected_commitment
}

fn main() {
  let mut rng = rand::thread_rng();
  let x: Scalar = Scalar::random(&mut rng);

  // Map the hash result to a point on the curve
  let msg: &[u8; 14] = b"CommitmentBase";
  let commitment_base: G1Projective = map_to_curve(msg);

  // Generate Pedersen commitment
  let r: Scalar = Scalar::random(&mut rng);
  let commitment: G1Projective = generate_commitment(x, r, commitment_base);

  // Verify the commitment
  let is_valid: bool = verify_commitment(commitment, x, r, commitment_base);
  let is_valid_str: &str = if is_valid { "valid" } else { "invalid" };
  println!("Commitment is {}", is_valid_str);

  // Output randomness and point on the curve in byte hex format
  println!("Randomness x (in byte hex format): {}", x.to_bytes().encode_hex::<String>());
  println!("Commitment (in byte hex format): {}", commitment.to_affine().to_bytes().encode_hex::<String>());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_commitment_verification() {
      let mut rng = rand::thread_rng();
      let x: Scalar = Scalar::random(&mut rng);
      let r: Scalar = Scalar::random(&mut rng);

      // Map the hash result to a point on the curve
      let msg = b"CommitmentBase";
      let commitment_base = map_to_curve(msg);

      // Generate Pedersen commitment
      let commitment = generate_commitment(x, r, commitment_base);

      // Verify the commitment
      assert!(verify_commitment(commitment, x, r, commitment_base));
  }
}