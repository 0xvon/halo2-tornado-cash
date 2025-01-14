use crate::halo2::{
    poly::commitment::Params as params,
    poly::commitment::ParamsProver,
    poly::commitment::ParamsVerifier,
    plonk,
    halo2curves::pasta::{vesta},
};

use crate::circuit::{Circuit};


#[derive(Debug)]
pub struct VerifyingKey {
    // pub params: dyn params<'_, vesta::Affine>,
    // pub params: ParamsVerifier<'a, vesta::Affine>,
    // pub vk: plonk::VerifyingKey<vesta::Affine>,
}

#[derive(Debug)]
pub struct ProvingKey {
    // pub params: params<'a, vesta::Affine>,
    // pub pk: plonk::ProvingKey<vesta::Affine>,
}

impl VerifyingKey {
    // Builds the verifying key.
    // pub fn build(k: u32) -> Self {
    //     let params = crate::halo2::poly::commitment::Params::new(k);
    //     let circuit: Circuit = Default::default();

    //     let vk = plonk::keygen_vk(&params, &circuit).unwrap();

    //     VerifyingKey { params, vk }
    // }

    // pub fn export<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
    //     self.vk.write(writer)
    // }
}

impl ProvingKey {
    // Builds the proving key.
    // pub fn build(k: u32) -> Self {
    //     let params = crate::halo2::poly::commitment::Params::new(k);
    //     let circuit: Circuit = Default::default();

    //     let vk = plonk::keygen_vk(&params, &circuit).unwrap();
    //     let pk = plonk::keygen_pk(&params, vk, &circuit).unwrap();

    //     ProvingKey { params, pk }
    // }
}