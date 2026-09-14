//! Exact source-oracle reuse within one benchmark input and transcript scope.
//!
//! This does not replace proof verification or transcript comparison. The
//! caller must still verify every proof, compare returned claims and the next
//! challenge, and enforce cross-kernel proof-byte equality. Construct a fresh
//! cache for every (dimension, run); its borrowed inputs cannot be mutated.

use mikros::Claims;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceCheckRecord {
    /// `full_oracle` or `exact_claim_cache`; both authenticate the exact query.
    pub method: &'static str,
    /// The system whose successful full oracle established this record.
    pub anchor: String,
}

struct VerifiedClaims {
    claims: Claims,
    anchor: String,
}

pub struct SourceCheckCache<'a> {
    a: &'a [u8],
    b: &'a [u8],
    c: &'a [u8],
    log: usize,
    verified: Option<VerifiedClaims>,
}

impl<'a> SourceCheckCache<'a> {
    pub fn new(a: &'a [u8], b: &'a [u8], c: &'a [u8], log: usize) -> Self {
        Self {
            a,
            b,
            c,
            log,
            verified: None,
        }
    }

    /// Authenticate this exact input/query pair, optionally reusing a prior
    /// successful source check. Input slice identity includes both address and
    /// length; an equal digest, shape, value, or newly allocated copy does not
    /// suffice. A mismatching query in reuse mode is an error, not a fallback.
    /// `reuse=false` always calls the full oracle, even after a prior success.
    #[allow(clippy::too_many_arguments)]
    pub fn check(
        &mut self,
        a: &[u8],
        b: &[u8],
        c: &[u8],
        log: usize,
        claims: &Claims,
        system: &str,
        reuse: bool,
    ) -> Result<SourceCheckRecord, String> {
        if log != self.log
            || !std::ptr::eq(self.a, a)
            || !std::ptr::eq(self.b, b)
            || !std::ptr::eq(self.c, c)
        {
            return Err("source-check cache input identity or dimension changed".into());
        }
        if reuse {
            if let Some(verified) = &self.verified {
                if &verified.claims != claims {
                    return Err("source-check cache requires identical full Claims".into());
                }
                return Ok(SourceCheckRecord {
                    method: "exact_claim_cache",
                    anchor: verified.anchor.clone(),
                });
            }
        }

        // No cache mutation precedes this fallible operation. A rejected first
        // claim cannot establish an anchor; a later rejection cannot replace it.
        mikros::check_claims(a, b, c, log, claims)
            .map_err(|error| format!("source oracle rejected claims: {error}"))?;
        let anchor = system.to_owned();
        self.verified = Some(VerifiedClaims {
            claims: claims.clone(),
            anchor: anchor.clone(),
        });
        Ok(SourceCheckRecord {
            method: "full_oracle",
            anchor,
        })
    }
}
