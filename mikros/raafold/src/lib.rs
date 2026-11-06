//! Binary RAA SwitchFold polynomial commitments.
//! The default distance profile is explicitly assumed, not finite-length certified.
pub mod ghash;
pub mod raafold;
pub mod raafold_field;
pub mod raafold_short;
pub mod raafold_short_pc;

/// Errors returned by the standalone RAAFold backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    EncodingError,
    InvalidCommitment,
    InvalidNumberOfVariables,
    TranscriptError,
    HashingError,
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

/// Conditional SwitchFold query bound, given the asserted relative distance.
pub fn queries_for_relative_distance(security_bits: usize, distance: f64) -> Result<usize, Error> {
    if security_bits == 0 || !distance.is_finite() || distance <= 0.0 || distance >= 1.0 {
        return Err(Error::EncodingError);
    }
    let queries = -(security_bits as f64) / (1.0 - distance / 3.0).log2();
    if !queries.is_finite() || queries < 0.0 || queries > usize::MAX as f64 {
        return Err(Error::EncodingError);
    }
    Ok(queries.ceil() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blaze_query_budget_is_conditional_and_rounded_up() {
        assert_eq!(queries_for_relative_distance(100, 0.19).unwrap(), 1060);
        assert_eq!(queries_for_relative_distance(128, 0.19).unwrap(), 1357);
        assert!(queries_for_relative_distance(0, 0.19).is_err());
        assert!(queries_for_relative_distance(128, f64::NAN).is_err());
        assert_eq!(raafold::RAA_ACCUMULATION_LAYERS, 2);
        let profile = raafold::RaaDistanceProfile::blaze_assumed(4).unwrap();
        assert_eq!(
            profile.provenance(),
            raafold::RaaDistanceProvenance::Assumed
        );
        assert_eq!(profile.code_relative_distance(), 0.19);
        assert!(raafold::RaaDistanceProfile::blaze_assumed(2).is_err());
    }
}
