#!/usr/bin/env bash
bindgen                                    \
	--whitelist-type 'nng_.*'              \
	--whitelist-function 'nng_.*'          \
	--whitelist-var 'NNG_.*'               \
	--no-prepend-enum-name                 \
	--default-enum-style "consts"          \
	--opaque-type 'nng_pipe_s'             \
	--opaque-type 'nng_socket_s'           \
	--opaque-type 'nng_dialer_s'           \
	--opaque-type 'nng_listener_s'         \
	--opaque-type 'nng_ctx_s'              \
	--with-derive-default                  \
	--use-core                             \
	--output src/lib.rs                    \
	wrapper.h
