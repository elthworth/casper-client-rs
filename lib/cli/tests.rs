use casper_types::SecretKey;

use crate::{
    crypto::AsymmetricKeyExt,
    types::{ExecutableDeployItem, MAX_SERIALIZED_SIZE_OF_DEPLOY},
    Error, OutputKind,
};

use super::*;

const PKG_HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
const ENTRYPOINT: &str = "entrypoint";
const VERSION: &str = "0.1.0";
const SAMPLE_DEPLOY: &str = r#"{
      "hash": "4858bbd79ab7b825244c4e6959cbcd588a05608168ef36518bc6590937191d55",
      "header": {
        "account": "01f60bce2bb1059c41910eac1e7ee6c3ef4c8fcc63a901eb9603c1524cadfb0c18",
        "timestamp": "2021-01-19T01:18:19.120Z",
        "ttl": "10s",
        "gas_price": 1,
        "body_hash": "95f2f2358c4864f01f8b073ae6f5ae67baeaf7747fc0799d0078743c513bc1de",
        "dependencies": [
          "be5fdeea0240e999e376f8ecbce1bd4fd9336f58dae4a5842558a4da6ad35aa8",
          "168d7ea9c88e76b3eef72759f2a7af24663cc871a469c7ba1387ca479e82fb41"
        ],
        "chain_name": "casper-test-chain-name-1"
      },
      "payment": {
        "StoredVersionedContractByHash": {
          "hash": "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6",
          "version": null,
          "entry_point": "entrypoint",
          "args": [
            [
              "name_01",
              {
                "cl_type": "Bool",
                "bytes": "00",
                "parsed": false
              }
            ],
            [
              "name_02",
              {
                "cl_type": "I32",
                "bytes": "2a000000",
                "parsed": 42
              }
            ]
          ]
        }
      },
      "session": {
        "StoredVersionedContractByHash": {
          "hash": "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6",
          "version": null,
          "entry_point": "entrypoint",
          "args": [
            [
              "name_01",
              {
                "cl_type": "Bool",
                "bytes": "00",
                "parsed": false
              }
            ],
            [
              "name_02",
              {
                "cl_type": "I32",
                "bytes": "2a000000",
                "parsed": 42
              }
            ]
          ]
        }
      },
      "approvals": [
        {
          "signer": "01f60bce2bb1059c41910eac1e7ee6c3ef4c8fcc63a901eb9603c1524cadfb0c18",
          "signature": "010f538ef188770cdbf608bc2d7aa9460108b419b2b629f5e0714204a7f29149809a1d52776b0c514e3320494fdf6f9e9747f06f2c14ddf6f924ce218148e2840a"
        },
        {
          "signer": "01e67d6e56ae07eca98b07ecec8cfbe826b4d5bc51f3a86590c0882cdafbd72fcc",
          "signature": "01c4f58d7f6145c1e4397efce766149cde5450cbe74991269161e5e1f30a397e6bc4c484f3c72a645cefd42c55cfde0294bfd91de55ca977798c3c8d2a7e43a40c"
        }
      ]
    }"#;

pub fn deploy_params() -> DeployStrParams<'static> {
    DeployStrParams {
        secret_key: "resources/test.pem",
        ttl: "10s",
        chain_name: "casper-test-chain-name-1",
        ..Default::default()
    }
}

fn args_simple() -> Vec<&'static str> {
    vec!["name_01:bool='false'", "name_02:i32='42'"]
}

#[test]
fn should_create_deploy() {
    let deploy_params = deploy_params();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");

    let mut output = Vec::new();

    let deploy =
        deploy::with_payment_and_session(deploy_params, payment_params, session_params).unwrap();
    crate::write_deploy(&deploy, &mut output).unwrap();

    // The test output can be used to generate data for SAMPLE_DEPLOY:
    // let secret_key = SecretKey::generate_ed25519().unwrap();
    // deploy.sign(&secret_key);
    // println!("{}", serde_json::to_string_pretty(&deploy).unwrap());

    let result = String::from_utf8(output).unwrap();

    let expected = crate::read_deploy(SAMPLE_DEPLOY.as_bytes()).unwrap();
    let actual = crate::read_deploy(result.as_bytes()).unwrap();

    assert_eq!(expected.header().account(), actual.header().account());
    assert_eq!(expected.header().ttl(), actual.header().ttl());
    assert_eq!(expected.header().gas_price(), actual.header().gas_price());
    assert_eq!(expected.header().body_hash(), actual.header().body_hash());
    assert_eq!(expected.payment(), actual.payment());
    assert_eq!(expected.session(), actual.session());
}

#[test]
fn should_fail_to_create_large_deploy() {
    let deploy_params = deploy_params();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");
    // Create a string arg of 1048576 letter 'a's to ensure the deploy is greater than 1048576
    // bytes.
    let large_args_simple = format!("name_01:string='{:a<1048576}'", "");

    let session_params = SessionStrParams::with_package_hash(
        PKG_HASH,
        VERSION,
        ENTRYPOINT,
        vec![large_args_simple.as_str()],
        "",
    );

    match deploy::with_payment_and_session(deploy_params, payment_params, session_params) {
        Err(CliError::Core(Error::DeploySizeTooLarge {
            max_deploy_size,
            actual_deploy_size,
        })) => {
            assert_eq!(max_deploy_size, MAX_SERIALIZED_SIZE_OF_DEPLOY);
            assert!(actual_deploy_size > MAX_SERIALIZED_SIZE_OF_DEPLOY as usize);
        }
        Err(error) => panic!("unexpected error: {}", error),
        Ok(_) => panic!("failed to error while creating an excessively large deploy"),
    }
}

#[test]
fn should_read_deploy() {
    let bytes = SAMPLE_DEPLOY.as_bytes();
    assert!(matches!(crate::read_deploy(bytes), Ok(_)));
}

#[test]
fn should_sign_deploy() {
    let bytes = SAMPLE_DEPLOY.as_bytes();
    let deploy = crate::read_deploy(bytes).unwrap();
    assert_eq!(
        deploy.approvals().len(),
        2,
        "Sample deploy should have 2 approvals."
    );

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("deploy.json");

    crate::output_deploy(OutputKind::file(&path, false), &deploy).unwrap();

    let secret_key = SecretKey::generate_ed25519().unwrap();
    crate::sign_deploy_file(&path, &secret_key, OutputKind::file(&path, true)).unwrap();
    let signed_deploy = crate::read_deploy_file(&path).unwrap();

    assert_eq!(
        signed_deploy.approvals().len(),
        deploy.approvals().len() + 1,
    );
}

#[test]
fn should_create_transfer() {
    use casper_types::{AsymmetricType, PublicKey};

    // with public key.
    let secret_key = SecretKey::generate_ed25519().unwrap();
    let public_key = PublicKey::from(&secret_key).to_hex();
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        &public_key,
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
    );

    assert!(transfer_deploy.is_ok());
    assert!(matches!(
        transfer_deploy.unwrap().session(),
        ExecutableDeployItem::Transfer { .. }
    ));

    // with account hash
    let account_hash =
        "account-hash-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        account_hash,
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
    );

    assert!(transfer_deploy.is_ok());
    assert!(matches!(
        transfer_deploy.unwrap().session(),
        ExecutableDeployItem::Transfer { .. }
    ));

    // with uref.
    let uref = "uref-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-007";
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        uref,
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
    );

    assert!(transfer_deploy.is_ok());
    assert!(matches!(
        transfer_deploy.unwrap().session(),
        ExecutableDeployItem::Transfer { .. }
    ));
}

#[test]
fn should_fail_to_create_transfer_with_bad_args() {
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        "bad public key.",
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
    );

    println!("{:?}", transfer_deploy);

    assert!(matches!(
        transfer_deploy,
        Err(CliError::InvalidArgument {
            context: "new_transfer target_account",
            error: _
        })
    ));
}
