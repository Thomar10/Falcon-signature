# FALCON signature scheme

This project contains the FALCON signature scheme implementation in Rust along with a masked version of the static signature generation process using a non-masked version of the gaussian sampler. 

A testing framework that test the implementations for susceptibility to side channel attacks is also included. This utilizes a ChipWhisperer Pro to capture power traces and performs a Welch t-test.