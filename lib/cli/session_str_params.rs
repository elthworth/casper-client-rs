/// Container for session-related arguments used while constructing a `Deploy`.
///
/// ## `session_args_simple`
///
/// For methods taking `session_args_simple`, this parameter is the session contract arguments, in
/// the form `<NAME:TYPE='VALUE'>` or `<NAME:TYPE=null>`.
///
/// There are further details in
/// [the docs for the equivalent
/// `payment_args_simple`](struct.PaymentStrParams.html#payment_args_simple).
///
/// ## `session_args_complex`
///
/// For methods taking `session_args_complex`, this parameter is the session contract arguments, in
/// the form of a `ToBytes`-encoded file.
///
/// ---
///
/// **Note** while multiple payment args can be specified for a single session code instance, only
/// one of `session_args_simple` and `session_args_complex` may be used.
#[derive(Default)]
pub struct SessionStrParams<'a> {
    pub(super) session_hash: &'a str,
    pub(super) session_name: &'a str,
    pub(super) session_package_hash: &'a str,
    pub(super) session_package_name: &'a str,
    pub(super) session_path: &'a str,
    pub(super) session_args_simple: Vec<&'a str>,
    pub(super) session_args_complex: &'a str,
    pub(super) session_version: &'a str,
    pub(super) session_entry_point: &'a str,
    pub(super) is_session_transfer: bool,
}

impl<'a> SessionStrParams<'a> {
    /// Constructs a `SessionStrParams` using a session smart contract file.
    ///
    /// * `session_path` is the path to the compiled Wasm session code.
    /// * See the struct docs for a description of [`session_args_simple`](#session_args_simple) and
    ///   [`session_args_complex`](#session_args_complex).
    pub fn with_path(
        session_path: &'a str,
        session_args_simple: Vec<&'a str>,
        session_args_complex: &'a str,
    ) -> Self {
        Self {
            session_path,
            session_args_simple,
            session_args_complex,
            ..Default::default()
        }
    }

    /// Constructs a `SessionStrParams` using a stored contract's name.
    ///
    /// * `session_name` is the name of the stored contract (associated with the executing account)
    ///   to be called as the session.
    /// * `session_entry_point` is the name of the method that will be used when calling the session
    ///   contract.
    /// * See the struct docs for a description of [`session_args_simple`](#session_args_simple) and
    ///   [`session_args_complex`](#session_args_complex).
    pub fn with_name(
        session_name: &'a str,
        session_entry_point: &'a str,
        session_args_simple: Vec<&'a str>,
        session_args_complex: &'a str,
    ) -> Self {
        Self {
            session_name,
            session_args_simple,
            session_args_complex,
            session_entry_point,
            ..Default::default()
        }
    }

    /// Constructs a `SessionStrParams` using a stored contract's hex-encoded hash.
    ///
    /// * `session_hash` is the hex-encoded hash of the stored contract to be called as the session.
    /// * `session_entry_point` is the name of the method that will be used when calling the session
    ///   contract.
    /// * See the struct docs for a description of [`session_args_simple`](#session_args_simple) and
    ///   [`session_args_complex`](#session_args_complex).
    pub fn with_hash(
        session_hash: &'a str,
        session_entry_point: &'a str,
        session_args_simple: Vec<&'a str>,
        session_args_complex: &'a str,
    ) -> Self {
        Self {
            session_hash,
            session_args_simple,
            session_args_complex,
            session_entry_point,
            ..Default::default()
        }
    }

    /// Constructs a `SessionStrParams` using a stored contract's package name.
    ///
    /// * `session_package_name` is the name of the stored package to be called as the session.
    /// * `session_version` is the version of the called session contract. The latest will be used
    ///   if `session_version` is empty.
    /// * `session_entry_point` is the name of the method that will be used when calling the session
    ///   contract.
    /// * See the struct docs for a description of [`session_args_simple`](#session_args_simple) and
    ///   [`session_args_complex`](#session_args_complex).
    pub fn with_package_name(
        session_package_name: &'a str,
        session_version: &'a str,
        session_entry_point: &'a str,
        session_args_simple: Vec<&'a str>,
        session_args_complex: &'a str,
    ) -> Self {
        Self {
            session_package_name,
            session_args_simple,
            session_args_complex,
            session_version,
            session_entry_point,
            ..Default::default()
        }
    }

    /// Constructs a `SessionStrParams` using a stored contract's package hash.
    ///
    /// * `session_package_hash` is the hex-encoded hash of the stored package to be called as the
    ///   session.
    /// * `session_version` is the version of the called session contract. The latest will be used
    ///   if `session_version` is empty.
    /// * `session_entry_point` is the name of the method that will be used when calling the session
    ///   contract.
    /// * See the struct docs for a description of [`session_args_simple`](#session_args_simple) and
    ///   [`session_args_complex`](#session_args_complex).
    pub fn with_package_hash(
        session_package_hash: &'a str,
        session_version: &'a str,
        session_entry_point: &'a str,
        session_args_simple: Vec<&'a str>,
        session_args_complex: &'a str,
    ) -> Self {
        Self {
            session_package_hash,
            session_args_simple,
            session_args_complex,
            session_version,
            session_entry_point,
            ..Default::default()
        }
    }

    /// Constructs a `SessionStrParams` representing a `Transfer` type of `Deploy`.
    ///
    /// * See the struct docs for a description of [`session_args_simple`](#session_args_simple) and
    ///   [`session_args_complex`](#session_args_complex).
    pub fn with_transfer(session_args_simple: Vec<&'a str>, session_args_complex: &'a str) -> Self {
        Self {
            is_session_transfer: true,
            session_args_simple,
            session_args_complex,
            ..Default::default()
        }
    }
}
