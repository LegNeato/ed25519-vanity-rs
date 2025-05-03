#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use core::ops::{Add, Sub, Mul};

pub type fiat_25519_u1 = u8;
pub type fiat_25519_i1 = i8;
pub type fiat_25519_i2 = i8;

macro_rules! load_8u {
    ($s:expr, $offset:expr) => {
        ($s[$offset] as u64)
            | (($s[$offset + 1] as u64) << 8)
            | (($s[$offset + 2] as u64) << 16)
            | (($s[$offset + 3] as u64) << 24)
            | (($s[$offset + 4] as u64) << 32)
            | (($s[$offset + 5] as u64) << 40)
            | (($s[$offset + 6] as u64) << 48)
            | (($s[$offset + 7] as u64) << 56)
    };
}

#[inline(always)]
fn fiat_25519_addcarryx_u51(
    out1: &mut u64,
    out2: &mut fiat_25519_u1,
    arg1: fiat_25519_u1,
    arg2: u64,
    arg3: u64,
) {
    let x1: u64 = (((arg1 as u64).wrapping_add(arg2)).wrapping_add(arg3));
    let x2: u64 = (x1 & 0x7ffffffffffff);
    let x3: fiat_25519_u1 = ((x1 >> 51) as fiat_25519_u1);
    *out1 = x2;
    *out2 = x3;
}

#[inline(always)]
fn fiat_25519_subborrowx_u51(
    out1: &mut u64,
    out2: &mut fiat_25519_u1,
    arg1: fiat_25519_u1,
    arg2: u64,
    arg3: u64,
) {
    let x1: i64 = ((((((arg2 as i128).wrapping_sub((arg1 as i128))) as i64) as i128)
        .wrapping_sub((arg3 as i128))) as i64);
    let x2: fiat_25519_i1 = ((x1 >> 51) as fiat_25519_i1);
    let x3: u64 = (((x1 as i128) & 0x7ffffffffffff_i128) as u64);
    *out1 = x3;
    *out2 = ((0x0_i8.wrapping_sub((x2 as fiat_25519_i2))) as fiat_25519_u1);
}

#[inline(always)]
fn fiat_25519_cmovznz_u64(out1: &mut u64, arg1: fiat_25519_u1, arg2: u64, arg3: u64) {
    let x1: fiat_25519_u1 = (!(!arg1));
    let x2: u64 = (((((0x0_i8.wrapping_sub((x1 as fiat_25519_i2))) as fiat_25519_i1) as i128)
        & 0xffffffffffffffff_i128) as u64);
    let x3: u64 = ((x2 & arg3) | ((!x2) & arg2));
    *out1 = x3;
}

fn fiat_25519_to_bytes(out1: &mut [u8; 32], arg1: &[u64; 5]) {
    let mut x1: u64 = 0;
    let mut x2: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x1, &mut x2, 0x0, (arg1[0]), 0x7ffffffffffed);
    let mut x3: u64 = 0;
    let mut x4: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x3, &mut x4, x2, (arg1[1]), 0x7ffffffffffff);
    let mut x5: u64 = 0;
    let mut x6: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x5, &mut x6, x4, (arg1[2]), 0x7ffffffffffff);
    let mut x7: u64 = 0;
    let mut x8: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x7, &mut x8, x6, (arg1[3]), 0x7ffffffffffff);
    let mut x9: u64 = 0;
    let mut x10: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x9, &mut x10, x8, (arg1[4]), 0x7ffffffffffff);
    let mut x11: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x11, x10, 0x0_u64, 0xffffffffffffffff);
    let mut x12: u64 = 0;
    let mut x13: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x12, &mut x13, 0x0, x1, (x11 & 0x7ffffffffffed));
    let mut x14: u64 = 0;
    let mut x15: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x14, &mut x15, x13, x3, (x11 & 0x7ffffffffffff));
    let mut x16: u64 = 0;
    let mut x17: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x16, &mut x17, x15, x5, (x11 & 0x7ffffffffffff));
    let mut x18: u64 = 0;
    let mut x19: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x18, &mut x19, x17, x7, (x11 & 0x7ffffffffffff));
    let mut x20: u64 = 0;
    let mut x21: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x20, &mut x21, x19, x9, (x11 & 0x7ffffffffffff));
    let x22: u64 = (x20 << 4);
    let x23: u64 = (x18.wrapping_mul(0x2_u64));
    let x24: u64 = (x16 << 6);
    let x25: u64 = (x14 << 3);
    let x26: u8 = ((x12 & 0xff_u64) as u8);
    let x27: u64 = (x12 >> 8);
    let x28: u8 = ((x27 & 0xff_u64) as u8);
    let x29: u64 = (x27 >> 8);
    let x30: u8 = ((x29 & 0xff_u64) as u8);
    let x31: u64 = (x29 >> 8);
    let x32: u8 = ((x31 & 0xff_u64) as u8);
    let x33: u64 = (x31 >> 8);
    let x34: u8 = ((x33 & 0xff_u64) as u8);
    let x35: u64 = (x33 >> 8);
    let x36: u8 = ((x35 & 0xff_u64) as u8);
    let x37: u8 = ((x35 >> 8) as u8);
    let x38: u64 = (x25.wrapping_add((x37 as u64)));
    let x39: u8 = ((x38 & 0xff_u64) as u8);
    let x40: u64 = (x38 >> 8);
    let x41: u8 = ((x40 & 0xff_u64) as u8);
    let x42: u64 = (x40 >> 8);
    let x43: u8 = ((x42 & 0xff_u64) as u8);
    let x44: u64 = (x42 >> 8);
    let x45: u8 = ((x44 & 0xff_u64) as u8);
    let x46: u64 = (x44 >> 8);
    let x47: u8 = ((x46 & 0xff_u64) as u8);
    let x48: u64 = (x46 >> 8);
    let x49: u8 = ((x48 & 0xff_u64) as u8);
    let x50: u8 = ((x48 >> 8) as u8);
    let x51: u64 = (x24.wrapping_add((x50 as u64)));
    let x52: u8 = ((x51 & 0xff_u64) as u8);
    let x53: u64 = (x51 >> 8);
    let x54: u8 = ((x53 & 0xff_u64) as u8);
    let x55: u64 = (x53 >> 8);
    let x56: u8 = ((x55 & 0xff_u64) as u8);
    let x57: u64 = (x55 >> 8);
    let x58: u8 = ((x57 & 0xff_u64) as u8);
    let x59: u64 = (x57 >> 8);
    let x60: u8 = ((x59 & 0xff_u64) as u8);
    let x61: u64 = (x59 >> 8);
    let x62: u8 = ((x61 & 0xff_u64) as u8);
    let x63: u64 = (x61 >> 8);
    let x64: u8 = ((x63 & 0xff_u64) as u8);
    let x65: fiat_25519_u1 = ((x63 >> 8) as fiat_25519_u1);
    let x66: u64 = (x23.wrapping_add((x65 as u64)));
    let x67: u8 = ((x66 & 0xff_u64) as u8);
    let x68: u64 = (x66 >> 8);
    let x69: u8 = ((x68 & 0xff_u64) as u8);
    let x70: u64 = (x68 >> 8);
    let x71: u8 = ((x70 & 0xff_u64) as u8);
    let x72: u64 = (x70 >> 8);
    let x73: u8 = ((x72 & 0xff_u64) as u8);
    let x74: u64 = (x72 >> 8);
    let x75: u8 = ((x74 & 0xff_u64) as u8);
    let x76: u64 = (x74 >> 8);
    let x77: u8 = ((x76 & 0xff_u64) as u8);
    let x78: u8 = ((x76 >> 8) as u8);
    let x79: u64 = (x22.wrapping_add((x78 as u64)));
    let x80: u8 = ((x79 & 0xff_u64) as u8);
    let x81: u64 = (x79 >> 8);
    let x82: u8 = ((x81 & 0xff_u64) as u8);
    let x83: u64 = (x81 >> 8);
    let x84: u8 = ((x83 & 0xff_u64) as u8);
    let x85: u64 = (x83 >> 8);
    let x86: u8 = ((x85 & 0xff_u64) as u8);
    let x87: u64 = (x85 >> 8);
    let x88: u8 = ((x87 & 0xff_u64) as u8);
    let x89: u64 = (x87 >> 8);
    let x90: u8 = ((x89 & 0xff_u64) as u8);
    let x91: u8 = ((x89 >> 8) as u8);
    out1[0] = x26;
    out1[1] = x28;
    out1[2] = x30;
    out1[3] = x32;
    out1[4] = x34;
    out1[5] = x36;
    out1[6] = x39;
    out1[7] = x41;
    out1[8] = x43;
    out1[9] = x45;
    out1[10] = x47;
    out1[11] = x49;
    out1[12] = x52;
    out1[13] = x54;
    out1[14] = x56;
    out1[15] = x58;
    out1[16] = x60;
    out1[17] = x62;
    out1[18] = x64;
    out1[19] = x67;
    out1[20] = x69;
    out1[21] = x71;
    out1[22] = x73;
    out1[23] = x75;
    out1[24] = x77;
    out1[25] = x80;
    out1[26] = x82;
    out1[27] = x84;
    out1[28] = x86;
    out1[29] = x88;
    out1[30] = x90;
    out1[31] = x91;
}

#[inline(always)]
fn fiat_25519_carry(out1: &mut [u64; 5], arg1: &[u64; 5]) {
    let x1: u64 = (arg1[0]);
    let x2: u64 = ((x1 >> 51).wrapping_add((arg1[1])));
    let x3: u64 = ((x2 >> 51).wrapping_add((arg1[2])));
    let x4: u64 = ((x3 >> 51).wrapping_add((arg1[3])));
    let x5: u64 = ((x4 >> 51).wrapping_add((arg1[4])));
    let x6: u64 = ((x1 & 0x7ffffffffffff).wrapping_add(((x5 >> 51).wrapping_mul(0x13))));
    let x7: u64 = ((((x6 >> 51) as fiat_25519_u1) as u64).wrapping_add((x2 & 0x7ffffffffffff)));
    let x8: u64 = (x6 & 0x7ffffffffffff);
    let x9: u64 = (x7 & 0x7ffffffffffff);
    let x10: u64 = ((((x7 >> 51) as fiat_25519_u1) as u64).wrapping_add((x3 & 0x7ffffffffffff)));
    let x11: u64 = (x4 & 0x7ffffffffffff);
    let x12: u64 = (x5 & 0x7ffffffffffff);
    out1[0] = x8;
    out1[1] = x9;
    out1[2] = x10;
    out1[3] = x11;
    out1[4] = x12;
}

#[inline(always)]
fn fiat_25519_carry_square(out1: &mut [u64; 5], arg1: &[u64; 5]) {
    let x1: u64 = ((arg1[4]).wrapping_mul(0x13));
    let x2: u64 = (x1.wrapping_mul(0x2));
    let x3: u64 = ((arg1[4]).wrapping_mul(0x2));
    let x4: u64 = ((arg1[3]).wrapping_mul(0x13));
    let x5: u64 = (x4.wrapping_mul(0x2));
    let x6: u64 = ((arg1[3]).wrapping_mul(0x2));
    let x7: u64 = ((arg1[2]).wrapping_mul(0x2));
    let x8: u64 = ((arg1[1]).wrapping_mul(0x2));
    let x9: u128 = (((arg1[4]) as u128).wrapping_mul((x1 as u128)));
    let x10: u128 = (((arg1[3]) as u128).wrapping_mul((x2 as u128)));
    let x11: u128 = (((arg1[3]) as u128).wrapping_mul((x4 as u128)));
    let x12: u128 = (((arg1[2]) as u128).wrapping_mul((x2 as u128)));
    let x13: u128 = (((arg1[2]) as u128).wrapping_mul((x5 as u128)));
    let x14: u128 = (((arg1[2]) as u128).wrapping_mul(((arg1[2]) as u128)));
    let x15: u128 = (((arg1[1]) as u128).wrapping_mul((x2 as u128)));
    let x16: u128 = (((arg1[1]) as u128).wrapping_mul((x6 as u128)));
    let x17: u128 = (((arg1[1]) as u128).wrapping_mul((x7 as u128)));
    let x18: u128 = (((arg1[1]) as u128).wrapping_mul(((arg1[1]) as u128)));
    let x19: u128 = (((arg1[0]) as u128).wrapping_mul((x3 as u128)));
    let x20: u128 = (((arg1[0]) as u128).wrapping_mul((x6 as u128)));
    let x21: u128 = (((arg1[0]) as u128).wrapping_mul((x7 as u128)));
    let x22: u128 = (((arg1[0]) as u128).wrapping_mul((x8 as u128)));
    let x23: u128 = (((arg1[0]) as u128).wrapping_mul(((arg1[0]) as u128)));
    let x24: u128 = (x23.wrapping_add((x15.wrapping_add(x13))));
    let x25: u64 = ((x24 >> 51) as u64);
    let x26: u64 = ((x24 & 0x7ffffffffffff_u128) as u64);
    let x27: u128 = (x19.wrapping_add((x16.wrapping_add(x14))));
    let x28: u128 = (x20.wrapping_add((x17.wrapping_add(x9))));
    let x29: u128 = (x21.wrapping_add((x18.wrapping_add(x10))));
    let x30: u128 = (x22.wrapping_add((x12.wrapping_add(x11))));
    let x31: u128 = ((x25 as u128).wrapping_add(x30));
    let x32: u64 = ((x31 >> 51) as u64);
    let x33: u64 = ((x31 & 0x7ffffffffffff_u128) as u64);
    let x34: u128 = ((x32 as u128).wrapping_add(x29));
    let x35: u64 = ((x34 >> 51) as u64);
    let x36: u64 = ((x34 & 0x7ffffffffffff_u128) as u64);
    let x37: u128 = ((x35 as u128).wrapping_add(x28));
    let x38: u64 = ((x37 >> 51) as u64);
    let x39: u64 = ((x37 & 0x7ffffffffffff_u128) as u64);
    let x40: u128 = ((x38 as u128).wrapping_add(x27));
    let x41: u64 = ((x40 >> 51) as u64);
    let x42: u64 = ((x40 & 0x7ffffffffffff_u128) as u64);
    let x43: u64 = (x41.wrapping_mul(0x13));
    let x44: u64 = (x26.wrapping_add(x43));
    let x45: u64 = (x44 >> 51);
    let x46: u64 = (x44 & 0x7ffffffffffff);
    let x47: u64 = (x45.wrapping_add(x33));
    let x48: fiat_25519_u1 = ((x47 >> 51) as fiat_25519_u1);
    let x49: u64 = (x47 & 0x7ffffffffffff);
    let x50: u64 = ((x48 as u64).wrapping_add(x36));
    out1[0] = x46;
    out1[1] = x49;
    out1[2] = x50;
    out1[3] = x39;
    out1[4] = x42;
}

#[inline(always)]
fn fiat_25519_add(out1: &mut [u64; 5], arg1: &[u64; 5], arg2: &[u64; 5]) {
    let x1: u64 = ((arg1[0]).wrapping_add((arg2[0])));
    let x2: u64 = ((arg1[1]).wrapping_add((arg2[1])));
    let x3: u64 = ((arg1[2]).wrapping_add((arg2[2])));
    let x4: u64 = ((arg1[3]).wrapping_add((arg2[3])));
    let x5: u64 = ((arg1[4]).wrapping_add((arg2[4])));
    out1[0] = x1;
    out1[1] = x2;
    out1[2] = x3;
    out1[3] = x4;
    out1[4] = x5;
}

#[inline(always)]
fn fiat_25519_sub(out1: &mut [u64; 5], arg1: &[u64; 5], arg2: &[u64; 5]) {
    let x1: u64 = ((0xfffffffffffdau64.wrapping_add((arg1[0]))).wrapping_sub((arg2[0])));
    let x2: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[1]))).wrapping_sub((arg2[1])));
    let x3: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[2]))).wrapping_sub((arg2[2])));
    let x4: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[3]))).wrapping_sub((arg2[3])));
    let x5: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[4]))).wrapping_sub((arg2[4])));
    out1[0] = x1;
    out1[1] = x2;
    out1[2] = x3;
    out1[3] = x4;
    out1[4] = x5;
}

#[inline(always)]
fn fiat_25519_carry_mul(out1: &mut [u64; 5], arg1: &[u64; 5], arg2: &[u64; 5]) {
    let x1: u128 = (((arg1[4]) as u128).wrapping_mul((((arg2[4]).wrapping_mul(0x13)) as u128)));
    let x2: u128 = (((arg1[4]) as u128).wrapping_mul((((arg2[3]).wrapping_mul(0x13)) as u128)));
    let x3: u128 = (((arg1[4]) as u128).wrapping_mul((((arg2[2]).wrapping_mul(0x13)) as u128)));
    let x4: u128 = (((arg1[4]) as u128).wrapping_mul((((arg2[1]).wrapping_mul(0x13)) as u128)));
    let x5: u128 = (((arg1[3]) as u128).wrapping_mul((((arg2[4]).wrapping_mul(0x13)) as u128)));
    let x6: u128 = (((arg1[3]) as u128).wrapping_mul((((arg2[3]).wrapping_mul(0x13)) as u128)));
    let x7: u128 = (((arg1[3]) as u128).wrapping_mul((((arg2[2]).wrapping_mul(0x13)) as u128)));
    let x8: u128 = (((arg1[2]) as u128).wrapping_mul((((arg2[4]).wrapping_mul(0x13)) as u128)));
    let x9: u128 = (((arg1[2]) as u128).wrapping_mul((((arg2[3]).wrapping_mul(0x13)) as u128)));
    let x10: u128 = (((arg1[1]) as u128).wrapping_mul((((arg2[4]).wrapping_mul(0x13)) as u128)));
    let x11: u128 = (((arg1[4]) as u128).wrapping_mul(((arg2[0]) as u128)));
    let x12: u128 = (((arg1[3]) as u128).wrapping_mul(((arg2[1]) as u128)));
    let x13: u128 = (((arg1[3]) as u128).wrapping_mul(((arg2[0]) as u128)));
    let x14: u128 = (((arg1[2]) as u128).wrapping_mul(((arg2[2]) as u128)));
    let x15: u128 = (((arg1[2]) as u128).wrapping_mul(((arg2[1]) as u128)));
    let x16: u128 = (((arg1[2]) as u128).wrapping_mul(((arg2[0]) as u128)));
    let x17: u128 = (((arg1[1]) as u128).wrapping_mul(((arg2[3]) as u128)));
    let x18: u128 = (((arg1[1]) as u128).wrapping_mul(((arg2[2]) as u128)));
    let x19: u128 = (((arg1[1]) as u128).wrapping_mul(((arg2[1]) as u128)));
    let x20: u128 = (((arg1[1]) as u128).wrapping_mul(((arg2[0]) as u128)));
    let x21: u128 = (((arg1[0]) as u128).wrapping_mul(((arg2[4]) as u128)));
    let x22: u128 = (((arg1[0]) as u128).wrapping_mul(((arg2[3]) as u128)));
    let x23: u128 = (((arg1[0]) as u128).wrapping_mul(((arg2[2]) as u128)));
    let x24: u128 = (((arg1[0]) as u128).wrapping_mul(((arg2[1]) as u128)));
    let x25: u128 = (((arg1[0]) as u128).wrapping_mul(((arg2[0]) as u128)));
    let x26: u128 =
        (x25.wrapping_add((x10.wrapping_add((x9.wrapping_add((x7.wrapping_add(x4))))))));
    let x27: u64 = ((x26 >> 51) as u64);
    let x28: u64 = ((x26 & 0x7ffffffffffff_u128) as u64);
    let x29: u128 =
        (x21.wrapping_add((x17.wrapping_add((x14.wrapping_add((x12.wrapping_add(x11))))))));
    let x30: u128 =
        (x22.wrapping_add((x18.wrapping_add((x15.wrapping_add((x13.wrapping_add(x1))))))));
    let x31: u128 =
        (x23.wrapping_add((x19.wrapping_add((x16.wrapping_add((x5.wrapping_add(x2))))))));
    let x32: u128 =
        (x24.wrapping_add((x20.wrapping_add((x8.wrapping_add((x6.wrapping_add(x3))))))));
    let x33: u128 = ((x27 as u128).wrapping_add(x32));
    let x34: u64 = ((x33 >> 51) as u64);
    let x35: u64 = ((x33 & 0x7ffffffffffff_u128) as u64);
    let x36: u128 = ((x34 as u128).wrapping_add(x31));
    let x37: u64 = ((x36 >> 51) as u64);
    let x38: u64 = ((x36 & 0x7ffffffffffff_u128) as u64);
    let x39: u128 = ((x37 as u128).wrapping_add(x30));
    let x40: u64 = ((x39 >> 51) as u64);
    let x41: u64 = ((x39 & 0x7ffffffffffff_u128) as u64);
    let x42: u128 = ((x40 as u128).wrapping_add(x29));
    let x43: u64 = ((x42 >> 51) as u64);
    let x44: u64 = ((x42 & 0x7ffffffffffff_u128) as u64);
    let x45: u64 = (x43.wrapping_mul(0x13));
    let x46: u64 = (x28.wrapping_add(x45));
    let x47: u64 = (x46 >> 51);
    let x48: u64 = (x46 & 0x7ffffffffffff);
    let x49: u64 = (x47.wrapping_add(x35));
    let x50: fiat_25519_u1 = ((x49 >> 51) as fiat_25519_u1);
    let x51: u64 = (x49 & 0x7ffffffffffff);
    let x52: u64 = ((x50 as u64).wrapping_add(x38));
    out1[0] = x48;
    out1[1] = x51;
    out1[2] = x52;
    out1[3] = x41;
    out1[4] = x44;
}

#[inline(always)]
fn fiat_25519_selectznz(
    out1: &mut [u64; 5],
    arg1: fiat_25519_u1,
    arg2: &[u64; 5],
    arg3: &[u64; 5],
) {
    let mut x1: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x1, arg1, (arg2[0]), (arg3[0]));
    let mut x2: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x2, arg1, (arg2[1]), (arg3[1]));
    let mut x3: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x3, arg1, (arg2[2]), (arg3[2]));
    let mut x4: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x4, arg1, (arg2[3]), (arg3[3]));
    let mut x5: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x5, arg1, (arg2[4]), (arg3[4]));
    out1[0] = x1;
    out1[1] = x2;
    out1[2] = x3;
    out1[3] = x4;
    out1[4] = x5;
}

#[derive(Clone, Default, Copy)]
pub struct Fe(pub [u64; 5]);

impl Fe {
    const fn from_bytes_const(s: &[u8; 32]) -> Fe {
        let mut h = [0u64; 5];
        let mask = 0x7ffffffffffff;
        
        h[0] = load_8u!(s, 0) & mask;
        h[1] = (load_8u!(s, 6) >> 3) & mask;
        h[2] = (load_8u!(s, 12) >> 6) & mask;
        h[3] = (load_8u!(s, 19) >> 1) & mask;
        h[4] = (load_8u!(s, 24) >> 12) & mask;
        
        Fe(h)
    }

    fn carry(&self) -> Fe {
        let mut h = Fe::default();
        fiat_25519_carry(&mut h.0, &self.0);
        h
    }

    fn to_bytes(&self) -> [u8; 32] {
        let &Fe(es) = &self.carry();
        let mut s_ = [0u8; 32];
        fiat_25519_to_bytes(&mut s_, &es);
        s_
    }

    fn is_negative(&self) -> bool {
        (self.to_bytes()[0] & 1) != 0
    }

    fn invert(&self) -> Fe {
        let z1 = *self;
        let z2 = z1.square();
        let z8 = z2.square().square();
        let z9 = z1 * z8;
        let z11 = z2 * z9;
        let z22 = z11.square();
        let z_5_0 = z9 * z22;
        let z_10_5 = (0..5).fold(z_5_0, |z_5_n, _| z_5_n.square());
        let z_10_0 = z_10_5 * z_5_0;
        let z_20_10 = (0..10).fold(z_10_0, |x, _| x.square());
        let z_20_0 = z_20_10 * z_10_0;
        let z_40_20 = (0..20).fold(z_20_0, |x, _| x.square());
        let z_40_0 = z_40_20 * z_20_0;
        let z_50_10 = (0..10).fold(z_40_0, |x, _| x.square());
        let z_50_0 = z_50_10 * z_10_0;
        let z_100_50 = (0..50).fold(z_50_0, |x, _| x.square());
        let z_100_0 = z_100_50 * z_50_0;
        let z_200_100 = (0..100).fold(z_100_0, |x, _| x.square());
        let z_200_0 = z_200_100 * z_100_0;
        let z_250_50 = (0..50).fold(z_200_0, |x, _| x.square());
        let z_250_0 = z_250_50 * z_50_0;
        let z_255_5 = (0..5).fold(z_250_0, |x, _| x.square());
        let z_255_21 = z_255_5 * z11;
        z_255_21
    }

    fn square(&self) -> Fe {
        let &Fe(f) = &self;
        let mut h = Fe::default();
        fiat_25519_carry_square(&mut h.0, f);
        h
    }

    fn square_and_double(&self) -> Fe {
        let h = self.square();
        (h + h)
    }

    fn maybe_set(&mut self, other: &Fe, do_swap: u8) {
        let &mut Fe(f) = self;
        let &Fe(g) = other;
        let mut t = [0u64; 5];
        fiat_25519_selectznz(&mut t, do_swap, &f, &g);
        self.0 = t
    }
}

impl Add for Fe {
    type Output = Fe;

    fn add(self, _rhs: Fe) -> Fe {
        let Fe(f) = self;
        let Fe(g) = _rhs;
        let mut h = Fe::default();
        fiat_25519_add(&mut h.0, &f, &g);
        h
    }
}

impl Sub for Fe {
    type Output = Fe;

    fn sub(self, _rhs: Fe) -> Fe {
        let Fe(f) = self;
        let Fe(g) = _rhs;
        let mut h = Fe::default();
        fiat_25519_sub(&mut h.0, &f, &g);
        h.carry()
    }
}

impl Mul for Fe {
    type Output = Fe;

    fn mul(self, _rhs: Fe) -> Fe {
        let Fe(f) = self;
        let Fe(g) = _rhs;
        let mut h = Fe::default();
        fiat_25519_carry_mul(&mut h.0, &f, &g);
        h
    }
}

static BXP: [u8; 32] = [
    0x1a, 0xd5, 0x25, 0x8f, 0x60, 0x2d, 0x56, 0xc9, 0xb2, 0xa7, 0x25, 0x95, 0x60, 0xc7, 0x2c,
    0x69, 0x5c, 0xdc, 0xd6, 0xfd, 0x31, 0xe2, 0xa4, 0xc0, 0xfe, 0x53, 0x6e, 0xcd, 0xd3, 0x36,
    0x69, 0x21,
];

static BYP: [u8; 32] = [
    0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
    0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
    0x66, 0x66,
];

pub static FE_ZERO: Fe = Fe([0, 0, 0, 0, 0]);

static FE_ONE: Fe = Fe([1, 0, 0, 0, 0]);

pub(crate) static FE_D2: Fe = Fe([
    1859910466990425,
    932731440258426,
    1072319116312658,
    1815898335770999,
    633789495995903,
]);

#[derive(Clone, Copy, Default)]
pub struct GeP1P1 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}

#[derive(Clone, Copy)]
pub struct GeP2 {
    x: Fe,
    y: Fe,
    z: Fe,
}

#[derive(Clone, Copy)]
pub struct GeP3 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}

impl GeP3 {
    fn zero() -> GeP3 {
        GeP3 {
            x: FE_ZERO,
            y: FE_ONE,
            z: FE_ONE,
            t: FE_ZERO,
        }
    }

    fn to_cached(&self) -> GeCached {
        GeCached {
            y_plus_x: self.y + self.x,
            y_minus_x: self.y - self.x,
            z: self.z,
            t2d: self.t * FE_D2,
        }
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let recip = self.z.invert();
        let x = self.x * recip;
        let y = self.y * recip;
        let mut bs = y.to_bytes();
        bs[31] ^= (if x.is_negative() { 1 } else { 0 }) << 7;
        bs
    }

    fn dbl(&self) -> GeP1P1 {
        self.to_p2().dbl()
    }

    fn to_p2(&self) -> GeP2 {
        GeP2 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add<GeCached> for GeP3 {
    type Output = GeP1P1;

    fn add(self, _rhs: GeCached) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_plus_x;
        let b = y1_minus_x1 * _rhs.y_minus_x;
        let c = _rhs.t2d * self.t;
        let zz = self.z * _rhs.z;
        let d = zz + zz;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d + c;
        let t3 = d - c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl Add<GeP3> for GeP3 {
    type Output = GeP3;

    fn add(self, other: GeP3) -> GeP3 {
        (self + other.to_cached()).to_p3()
    }
}

impl GeP2 {
    fn dbl(&self) -> GeP1P1 {
        let xx = self.x.square();
        let yy = self.y.square();
        let b = self.z.square_and_double();
        let a = self.x + self.y;
        let aa = a.square();
        let y3 = yy + xx;
        let z3 = yy - xx;
        let x3 = aa - y3;
        let t3 = b - z3;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl GeP1P1 {
    fn to_p3(&self) -> GeP3 {
        GeP3 {
            x: self.x * self.t,
            y: self.y * self.z,
            z: self.z * self.t,
            t: self.x * self.y,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct GeCached {
    y_plus_x: Fe,
    y_minus_x: Fe,
    z: Fe,
    t2d: Fe,
}

impl GeCached {
    fn maybe_set(&mut self, other: &GeCached, do_swap: u8) {
        self.y_plus_x.maybe_set(&other.y_plus_x, do_swap);
        self.y_minus_x.maybe_set(&other.y_minus_x, do_swap);
        self.z.maybe_set(&other.z, do_swap);
        self.t2d.maybe_set(&other.t2d, do_swap);
    }

    pub const fn from_bytes_const(s: &[[u8; 32]; 4]) -> Self {
        GeCached {
            y_plus_x: Fe::from_bytes_const(&s[0]),
            y_minus_x: Fe::from_bytes_const(&s[1]),
            z: Fe::from_bytes_const(&s[2]),
            t2d: Fe::from_bytes_const(&s[3]),
        }
    }

    pub fn to_bytes(&self) -> [[u8; 32]; 4] {
        [
            self.y_plus_x.to_bytes(),
            self.y_minus_x.to_bytes(),
            self.z.to_bytes(),
            self.t2d.to_bytes(),
        ]
    }
}

fn ge_precompute(base: &GeP3) -> [GeCached; 16] {
    let base_cached = base.to_cached();
    let mut pc = [GeP3::zero(); 16];
    pc[1] = *base;
    for i in 2..16 {
        pc[i] = if i % 2 == 0 {
            pc[i / 2].dbl().to_p3()
        } else {
            pc[i - 1].add(base_cached).to_p3()
        }
    }
    let mut pc_cached: [GeCached; 16] = Default::default();
    for i in 0..16 {
        pc_cached[i] = pc[i].to_cached();
    }
    pc_cached
}

pub fn calculate_precompute() -> [GeCached; 16] {
    let bx = Fe::from_bytes_const(&BXP);
    let by = Fe::from_bytes_const(&BYP);
    let base = GeP3 {
        x: bx,
        y: by,
        z: FE_ONE,
        t: bx * by,
    };
    let precomputed = ge_precompute(&base);
    precomputed
}

pub fn ge_scalarmult(scalar: &[u8], precomputed: &[GeCached; 16]) -> GeP3 {
    let mut q = GeP3::zero();
    let mut pos = 252;
    loop {
        let slot = ((scalar[pos >> 3] >> (pos & 7)) & 15) as usize;
        let mut t = precomputed[0];
        for i in 1..16 {
            t.maybe_set(&precomputed[i], (((slot ^ i).wrapping_sub(1)) >> 8) as u8 & 1);
        }
        q = q.add(t).to_p3();
        if pos == 0 {
            break;
        }
        q = q.dbl().to_p3().dbl().to_p3().dbl().to_p3().dbl().to_p3();
        pos -= 4;
    }
    q
}
