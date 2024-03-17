use axum::extract::FromRef;
use jwt_simple::{
    algorithms::{RS384KeyPair, RSAKeyPairLike, RSAPublicKeyLike}, common::VerificationOptions, reexports::coarsetime::Duration
};
use uuid::Uuid;

use crate::RuimContext;

#[derive(Debug, Clone)]
pub struct Jwt {
    key_pair: RS384KeyPair,
}

impl Jwt {
    pub fn new_from_env() -> anyhow::Result<Self> {
        // read env JWT_PRIVATE_KEY
        let private_key_path = std::env::var("JWT_PRIVATE_KEY")?;

        dbg!(&private_key_path);
        // read file
        let private_key = std::fs::read_to_string(private_key_path)?;

        let key_pair = RS384KeyPair::from_pem(&private_key)?;

        Ok(Self { key_pair })
    }

    pub fn generate_token(&self, claim: UserTokenClaims) -> anyhow::Result<String> {
        let claim = jwt_simple::claims::Claims::with_custom_claims(claim, Duration::from_hours(3))
            .with_issuer("ruim");

        let token = self.key_pair.sign(claim)?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> anyhow::Result<UserTokenClaims> {
        let public_key = self.key_pair.public_key();
        let option = {
            #[cfg(debug_assertions)]// development mode, ignore time tolerance
            {
                let mut verification_options = VerificationOptions::default();
                verification_options.time_tolerance = Some(Duration::from_days(365));
                Some(verification_options)
            }

            #[cfg(not(debug_assertions))]
            {
                None
            }
        };
        
        let claim = public_key.verify_token::<UserTokenClaims>(token, option)?;

        Ok(claim.custom)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct UserTokenClaims {
    pub user_id: Uuid,
}

impl FromRef<crate::RuimContext> for Jwt {
    fn from_ref(input: &RuimContext) -> Self {
        input.jwt.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_token() {
        dotenv::dotenv().ok();
        // Create a new Jwt instance
        let jwt = Jwt::new_from_env().unwrap();

        // Create a sample claim
        let claim = UserTokenClaims {
            user_id: Uuid::new_v4(),
        };

        // Generate a token
        let token = jwt.generate_token(claim.clone()).unwrap();

        // Verify the token
        let verified_claim = jwt.verify_token(&token).unwrap();

        // Assert that the verified claim matches the original claim
        assert_eq!(verified_claim, claim);
    }
}
