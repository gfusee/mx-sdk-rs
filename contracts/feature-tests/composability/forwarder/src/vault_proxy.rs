////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct VaultProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for VaultProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = VaultProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        VaultProxyMethods { wrapped_tx: tx }
    }
}

pub struct VaultProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> VaultProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        opt_arg_to_echo: Arg0,
    ) -> TxProxyDeploy<Env, From, Gas, OptionalValue<ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&opt_arg_to_echo)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> VaultProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade<
        Arg0: CodecInto<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        opt_arg_to_echo: Arg0,
    ) -> TxProxyUpgrade<Env, From, To, Gas, MultiValue2<Box<str>, OptionalValue<ManagedBuffer<Env::Api>>>> {
        self.wrapped_tx
            .raw_upgrade()
            .argument(&opt_arg_to_echo)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> VaultProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn echo_arguments<
        Arg0: CodecInto<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        args: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .raw_call()
            .function_name("echo_arguments")
            .argument(&args)
            .original_result()
    }

    pub fn echo_arguments_without_storage<
        Arg0: CodecInto<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        args: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .raw_call()
            .function_name("echo_arguments_without_storage")
            .argument(&args)
            .original_result()
    }

    pub fn echo_caller(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call()
            .function_name("echo_caller")
            .original_result()
    }

    pub fn accept_funds(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("accept_funds")
            .original_result()
    }

    pub fn accept_funds_echo_payment(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<BigUint<Env::Api>, MultiValueEncoded<Env::Api, EsdtTokenPaymentMultiValue<Env::Api>>>> {
        self.wrapped_tx
            .raw_call()
            .function_name("accept_funds_echo_payment")
            .original_result()
    }

    pub fn accept_funds_single_esdt_transfer(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("accept_funds_single_esdt_transfer")
            .original_result()
    }

    pub fn reject_funds(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("reject_funds")
            .original_result()
    }

    pub fn retrieve_funds_with_transfer_exec<
        Arg0: CodecInto<TokenIdentifier<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
        Arg2: CodecInto<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        token: Arg0,
        amount: Arg1,
        opt_receive_func: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("retrieve_funds_with_transfer_exec")
            .argument(&token)
            .argument(&amount)
            .argument(&opt_receive_func)
            .original_result()
    }

    pub fn retrieve_funds_promises<
        Arg0: CodecInto<OptionalValue<u64>>,
        Arg1: CodecInto<OptionalValue<BigUint<Env::Api>>>,
    >(
        self,
        back_transfers: Arg0,
        back_transfer_value: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("retrieve_funds_promises")
            .argument(&back_transfers)
            .argument(&back_transfer_value)
            .original_result()
    }

    pub fn retrieve_funds<
        Arg0: CodecInto<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg1: CodecInto<u64>,
        Arg2: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        token: Arg0,
        nonce: Arg1,
        amount: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("retrieve_funds")
            .argument(&token)
            .argument(&nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn retrieve_multi_funds_async<
        Arg0: CodecInto<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        token_payments: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("retrieve_multi_funds_async")
            .argument(&token_payments)
            .original_result()
    }

    pub fn burn_and_create_retrive_async(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("burn_and_create_retrive_async")
            .original_result()
    }

    pub fn get_owner_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call()
            .function_name("get_owner_address")
            .original_result()
    }

    /// We already leave a trace of the calls using the event logs; 
    /// this additional counter has the role of showing that storage also gets saved correctly. 
    pub fn call_counts<
        Arg0: CodecInto<ManagedBuffer<Env::Api>>,
    >(
        self,
        endpoint: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, usize> {
        self.wrapped_tx
            .raw_call()
            .function_name("call_counts")
            .argument(&endpoint)
            .original_result()
    }

    pub fn num_called_retrieve_funds_promises(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, usize> {
        self.wrapped_tx
            .raw_call()
            .function_name("num_called_retrieve_funds_promises")
            .original_result()
    }

    pub fn num_async_calls_sent_from_child(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, usize> {
        self.wrapped_tx
            .raw_call()
            .function_name("num_async_calls_sent_from_child")
            .original_result()
    }
}
