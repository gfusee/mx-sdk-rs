use crate::{
    api::VMApi,
    types::{ManagedAddress, ManagedOption, TxScEnv, TxTo},
};

pub trait ProxyObjBase {
    type Api: VMApi;
    type To: TxTo<TxScEnv<Self::Api>>;

    /// Extracts the address contained in the proxy object and replaces it with None.
    ///
    /// Will just return `ManagedOption::none()` if no address was specified.
    #[doc(hidden)]
    fn extract_opt_address(&mut self) -> ManagedOption<Self::Api, ManagedAddress<Self::Api>>;

    /// Extracts the address contained in the proxy object and replaces it with None.
    ///
    /// Will crash if no address was specified.
    #[doc(hidden)]
    fn extract_address(&mut self) -> ManagedAddress<Self::Api>;

    #[doc(hidden)]
    fn extract_proxy_to(&mut self) -> Self::To;
}

pub trait ProxyObjNew: ProxyObjBase {
    type ProxyTo: ProxyObjBase;

    #[doc(hidden)]
    fn new_proxy_obj() -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    #[must_use]
    fn contract(self, address: ManagedAddress<Self::Api>) -> Self::ProxyTo;
}
