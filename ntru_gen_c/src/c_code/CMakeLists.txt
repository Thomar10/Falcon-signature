cmake_minimum_required(VERSION 3.0)
project(NTRU_GEN C)
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
add_library(ng_mp31 STATIC ng_mp31.c ng_config.h ng_inner.h sha3.h)
add_library(ng_zint31 STATIC ng_zint31.c ng_config.h ng_inner.h sha3.h)
add_library(ng_poly STATIC ng_poly.c ng_config.h ng_inner.h sha3.h)
add_library(ng_fxp STATIC ng_fxp.c ng_config.h ng_inner.h sha3.h)
add_library(ng_ntru STATIC ng_ntru.c ng_fxp.c ng_config.h ng_inner.h sha3.h)
add_library(ng_falcon STATIC ng_falcon.c ng_fxp.c ng_config.h ng_inner.h sha3.h)
add_library(ng_prng STATIC ng_prng.c ng_fxp.c ng_config.h ng_inner.h sha3.h)
add_library(ng_gauss STATIC ng_gauss.c ng_fxp.c ng_config.h ng_inner.h sha3.h)
install(TARGETS ng_mp31 DESTINATION .)
install(TARGETS ng_zint31 DESTINATION .)
install(TARGETS ng_poly DESTINATION .)
install(TARGETS ng_fxp DESTINATION .)
install(TARGETS ng_ntru DESTINATION .)
install(TARGETS ng_falcon DESTINATION .)
install(TARGETS ng_prng DESTINATION .)
install(TARGETS ng_gauss DESTINATION .)