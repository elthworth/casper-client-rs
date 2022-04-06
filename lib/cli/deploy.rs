use casper_types::{account::AccountHash, AsymmetricType, PublicKey, UIntParseError, URef, U512};

use super::{parse, CliError, DeployStrParams, PaymentStrParams, SessionStrParams};
use crate::{
    types::{Deploy, DeployBuilder, MAX_SERIALIZED_SIZE_OF_DEPLOY},
    TransferTarget,
};

/// Creates new Deploy with specified payment and session data.
pub fn with_payment_and_session(
    deploy_params: DeployStrParams,
    payment_params: PaymentStrParams,
    session_params: SessionStrParams,
) -> Result<Deploy, CliError> {
    let chain_name = deploy_params.chain_name.to_string();
    let session = parse::session_executable_deploy_item(session_params)?;
    let secret_key = parse::secret_key_from_file(deploy_params.secret_key)?;
    let payment = parse::payment_executable_deploy_item(payment_params)?;
    let timestamp = parse::timestamp(deploy_params.timestamp)?;
    let ttl = parse::ttl(deploy_params.ttl)?;
    let session_account = parse::session_account(deploy_params.session_account)?;

    let mut deploy_builder = DeployBuilder::new(chain_name, session, &secret_key)
        .with_payment(payment)
        .with_timestamp(timestamp)
        .with_ttl(ttl);
    if let Some(account) = session_account {
        deploy_builder = deploy_builder.with_account(account);
    }
    let deploy = deploy_builder.build()?;
    deploy.is_valid_size(MAX_SERIALIZED_SIZE_OF_DEPLOY)?;
    Ok(deploy)
}

/// Creates new Transfer with specified data.
pub fn new_transfer(
    amount: &str,
    source_purse: Option<URef>,
    target_account: &str,
    transfer_id: &str,
    deploy_params: DeployStrParams,
    payment_params: PaymentStrParams,
) -> Result<Deploy, CliError> {
    let chain_name = deploy_params.chain_name.to_string();
    let secret_key = parse::secret_key_from_file(deploy_params.secret_key)?;
    let payment = parse::payment_executable_deploy_item(payment_params)?;

    let amount = U512::from_dec_str(amount).map_err(|err| CliError::FailedToParseUint {
        context: "new_transfer amount",
        error: UIntParseError::FromDecStr(err),
    })?;

    let target = if let Ok(public_key) = PublicKey::from_hex(target_account) {
        TransferTarget::PublicKey(public_key)
    } else if let Ok(account_hash) = AccountHash::from_formatted_str(target_account) {
        TransferTarget::AccountHash(account_hash)
    } else if let Ok(uref) = URef::from_formatted_str(target_account) {
        TransferTarget::URef(uref)
    } else {
        return Err(CliError::InvalidArgument {
            context: "new_transfer target_account",
            error: format!(
                "allowed types: PublicKey, AccountHash or URef, got {}",
                target_account
            ),
        });
    };

    let transfer_id = parse::transfer_id(transfer_id)?;
    let maybe_transfer_id = Some(transfer_id);

    let timestamp = parse::timestamp(deploy_params.timestamp)?;
    let ttl = parse::ttl(deploy_params.ttl)?;
    let session_account = parse::session_account(deploy_params.session_account)?;

    let mut deploy_builder = DeployBuilder::new_transfer(
        chain_name,
        amount,
        source_purse,
        target,
        maybe_transfer_id,
        &secret_key,
    )
    .with_payment(payment)
    .with_timestamp(timestamp)
    .with_ttl(ttl);
    if let Some(account) = session_account {
        deploy_builder = deploy_builder.with_account(account);
    }
    let deploy = deploy_builder.build()?;
    deploy.is_valid_size(MAX_SERIALIZED_SIZE_OF_DEPLOY)?;
    Ok(deploy)
}
