
#![crate_type = "lib"]
#![crate_name = "ssh"]

extern crate libc;
use std::mem;


/*
struct timeval
		(__time_t) tv_sec [long]
		(__suseconds_t) tv_usec [long]
*/
#[repr(C)]
pub struct timeval {
	tv_sec: libc::c_long,
	tv_usec: libc::c_long,
}

/*
void uuid_clear()
	(uuid_t) uu [unsigned char [16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_clear(uu: [libc::c_uchar; 16]);
}


/*
int uuid_compare()
	(const uuid_t) uu1 [unsigned char const[16]]
	(const uuid_t) uu2 [unsigned char const[16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_compare(uu1: [libc::c_uchar; 16], uu2: [libc::c_uchar; 16]) -> libc::c_int;
}


/*
void uuid_copy()
	(uuid_t) dst [unsigned char [16]]
	(const uuid_t) src [unsigned char const[16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_copy(dst: [libc::c_uchar; 16], src: [libc::c_uchar; 16]);
}


/*
void uuid_generate()
	(uuid_t) out [unsigned char [16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_generate(out: [libc::c_uchar; 16]);
}


/*
void uuid_generate_random()
	(uuid_t) out [unsigned char [16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_generate_random(out: [libc::c_uchar; 16]);
}


/*
void uuid_generate_time()
	(uuid_t) out [unsigned char [16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_generate_time(out: [libc::c_uchar; 16]);
}


/*
int uuid_generate_time_safe()
	(uuid_t) out [unsigned char [16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_generate_time_safe(out: [libc::c_uchar; 16]) -> libc::c_int;
}


/*
int uuid_is_null()
	(const uuid_t) uu [unsigned char const[16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_is_null(uu: [libc::c_uchar; 16]) -> libc::c_int;
}


/*
int uuid_parse()
	(const char *) in
	(uuid_t) uu [unsigned char [16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_parse(in_: *const libc::c_char, uu: [libc::c_uchar; 16]) -> libc::c_int;
}


/*
void uuid_unparse()
	(const uuid_t) uu [unsigned char const[16]]
	(char *) out
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_unparse(uu: [libc::c_uchar; 16], out: *mut libc::c_char);
}


/*
void uuid_unparse_lower()
	(const uuid_t) uu [unsigned char const[16]]
	(char *) out
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_unparse_lower(uu: [libc::c_uchar; 16], out: *mut libc::c_char);
}


/*
void uuid_unparse_upper()
	(const uuid_t) uu [unsigned char const[16]]
	(char *) out
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_unparse_upper(uu: [libc::c_uchar; 16], out: *mut libc::c_char);
}


/*
time_t uuid_time() [long]
	(const uuid_t) uu [unsigned char const[16]]
	(struct timeval *) ret_tv [struct timeval *]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_time(uu: [libc::c_uchar; 16], ret_tv: *mut timeval) -> libc::c_long;
}


/*
int uuid_type()
	(const uuid_t) uu [unsigned char const[16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_type(uu: [libc::c_uchar; 16]) -> libc::c_int;
}


/*
int uuid_variant()
	(const uuid_t) uu [unsigned char const[16]]
*/
#[link(name="libuuid")]
extern "C" {
	pub fn uuid_variant(uu: [libc::c_uchar; 16]) -> libc::c_int;
}


/* _UUID_UUID_H # */

/* UUID_VARIANT_NCS 0 # */
pub const UUID_VARIANT_NCS: i32 = 0;

/* UUID_VARIANT_DCE 1 # */
pub const UUID_VARIANT_DCE: i32 = 1;

/* UUID_VARIANT_MICROSOFT 2 # */
pub const UUID_VARIANT_MICROSOFT: i32 = 2;

/* UUID_VARIANT_OTHER 3 /* UUID Type definitions */ */
pub const UUID_VARIANT_OTHER: i32 = 3;

/* UUID_TYPE_DCE_TIME 1 # */
pub const UUID_TYPE_DCE_TIME: i32 = 1;

/* UUID_TYPE_DCE_RANDOM 4 /* Allow UUID constants to be defined */ */
pub const UUID_TYPE_DCE_RANDOM: i32 = 4;

/* UUID_DEFINE ( name , u0 , u1 , u2 , u3 , u4 , u5 , u6 , u7 , u8 , u9 , u10 , u11 , u12 , u13 , u14 , u15 ) static const uuid_t name __attribute__ ( ( unused ) ) = { u0 , u1 , u2 , u3 , u4 , u5 , u6 , u7 , u8 , u9 , u10 , u11 , u12 , u13 , u14 , u15 } # */

