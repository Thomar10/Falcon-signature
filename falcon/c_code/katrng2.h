//
//  katrng.h
//
//  Created by Bassham, Lawrence E (Fed) on 8/29/17.
//  Copyright © 2017 Bassham, Lawrence E (Fed). All rights reserved.
//

#ifndef katrng_h
#define katrng_h

#include <stdio.h>

#define RNG_SUCCESS2      0
#define RNG_BAD_MAXLEN2  -1
#define RNG_BAD_OUTBUF2  -2
#define RNG_BAD_REQ_LEN2 -3

typedef struct {
    unsigned char   buffer[16];
    int             buffer_pos;
    unsigned long   length_remaining;
    unsigned char   key[32];
    unsigned char   ctr[16];
} AES_XOF_struct2;

typedef struct {
    unsigned char   Key[32];
    unsigned char   V[16];
    int             reseed_counter;
} AES256_CTR_DRBG_struct;


void
AES256_CTR_DRBG_Update2(unsigned char *provided_data,
                       unsigned char *Key,
                       unsigned char *V);

int
seedexpander_init2(AES_XOF_struct *ctx,
                  unsigned char *seed,
                  unsigned char *diversifier,
                  unsigned long maxlen);

int
seedexpander2(AES_XOF_struct *ctx, unsigned char *x, unsigned long xlen);

void
randombytes_init2(unsigned char *entropy_input,
                 unsigned char *personalization_string,
                 int security_strength);

int
randombytes2(unsigned char *x, unsigned long long xlen);

#endif /* katrng_h */
