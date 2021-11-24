#include <stdio.h>

#include "casper_client.h"

#define RESPONSE_BUFFER_LEN 1024
#define ERROR_LEN 255
#define NODE_ADDRESS "http://localhost:11101"
#define RPC_ID "1"
#define VERBOSE 0

int main(int argc, char **argv) {
    casper_setup_client();

    casper_deploy_params_t deploy_params = {0};
    deploy_params.secret_key = "../casper-node/utils/nctl/assets/net-1/nodes/node-3/keys/secret_key.pem";
    deploy_params.ttl = "10s";
    deploy_params.chain_name = "casper-net-1";
    deploy_params.gas_price = "11";

    casper_payment_params_t payment_params = {0};
    payment_params.payment_amount = "5000000000";

    const char *payment_args[2] = {
        "name_01:bool='false'",
        "name_02:i32='42'",
    };
    payment_params.payment_args_simple = (const char *const *)&payment_args;
    payment_params.payment_args_simple_len = 2;

    casper_session_params_t session_params = {0};
    session_params.is_session_transfer = true;
    const char *session_args[3] = {
            "amount:u512='2500000000'",
            "target:public_key='015B2B1E0B2632CdbD2B81BA46273bDD0339f4D1206b8854d0ADe53D45a29b2F89'",
            "id:opt_u64='999'",
    };
    session_params.session_args_simple = (const char *const *)&session_args;
    session_params.session_args_simple_len = 3;

    unsigned char response_buffer[RESPONSE_BUFFER_LEN] = {0};
    casper_error_t success = casper_put_deploy(
        RPC_ID, NODE_ADDRESS, VERBOSE, &deploy_params, &session_params,
        &payment_params, response_buffer, RESPONSE_BUFFER_LEN);
    if (success == CASPER_SUCCESS) {
        printf("Got successful response\n%s\n", response_buffer);
    } else {
        unsigned char error[ERROR_LEN] = {0};
        casper_get_last_error(error, ERROR_LEN);
        printf("Got error:\n%s\n", error);
    }
    printf("Done.\n");

    casper_shutdown_client();

    return 0;
}
