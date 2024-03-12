use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrElement;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;

use ark_bls12_381::{Fr, G1Projective as G1};
use ark_ec::Group;

fn main() {
    let secret_key_hex = 0x6C616D6264617370;

    // Calculate the public key using lambdaworks
    let secret_key_l = FrElement::from(secret_key_hex);
    println!("secret key: {:?}", secret_key_l.to_hex());
    let generator = BLS12381Curve::generator();
    // NOTE: I use operate_with_self according to the documentation: https://github.com/lambdaclass/lambdaworks/tree/main/math/src/elliptic_curve
    // NOTE: But, the results are not the same for lambdaworks and arkworks. At first glance, all the fields are the same and the result
    // NOTE: should be the same, but maybe I'm wrong. I'd like to hear more about it at the bootcamp
    let public_key_l = generator.operate_with_self(secret_key_l.representative());
    println!(
        "public key (lambda): \nx: {:?}\ny: {:?}\nz: {:?}\n",
        public_key_l.x().to_hex(),
        public_key_l.y().to_hex(),
        public_key_l.z().to_hex()
    );

    // Calculate the public key using arkworks
    let secret_key_a = Fr::from(secret_key_hex);
    println!("secret key: {:02X}", secret_key_a.0);
    let public_key_a = G1::generator() * &secret_key_a;
    println!(
        "public key: \nx: {:02X}\ny: {:02X}\nz: {:02X}",
        public_key_a.x.0, public_key_a.y.0, public_key_a.z.0
    );
}
