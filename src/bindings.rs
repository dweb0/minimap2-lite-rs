#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


// This allows libminimap2 to call zlib functions
#[link(name = "z")]
extern "C" {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}

pub const _MM_F_NO_DIAG: u32 = 1;
pub const _MM_F_NO_DUAL: u32 = 2;
pub const _MM_F_CIGAR: u32 = 4;
pub const _MM_F_OUT_SAM: u32 = 8;
pub const _MM_F_NO_QUAL: u32 = 16;
pub const _MM_F_OUT_CG: u32 = 32;
pub const _MM_F_OUT_CS: u32 = 64;
pub const _MM_F_SPLICE: u32 = 128;
pub const _MM_F_SPLICE_FOR: u32 = 256;
pub const _MM_F_SPLICE_REV: u32 = 512;
pub const _MM_F_NO_LJOIN: u32 = 1024;
pub const _MM_F_OUT_CS_LONG: u32 = 2048;
pub const _MM_F_SR: u32 = 4096;
pub const _MM_F_FRAG_MODE: u32 = 8192;
pub const _MM_F_NO_PRINT_2ND: u32 = 16384;
pub const _MM_F_2_IO_THREADS: u32 = 32768;
pub const _MM_F_LONG_CIGAR: u32 = 65536;
pub const _MM_F_INDEPEND_SEG: u32 = 131_072;
pub const _MM_F_SPLICE_FLANK: u32 = 262_144;
pub const _MM_F_SOFTCLIP: u32 = 524_288;
pub const _MM_F_FOR_ONLY: u32 = 1_048_576;
pub const _MM_F_REV_ONLY: u32 = 2_097_152;
pub const _MM_F_HEAP_SORT: u32 = 4_194_304;
pub const _MM_F_ALL_CHAINS: u32 = 8_388_608;
pub const _MM_F_OUT_MD: u32 = 16_777_216;
pub const _MM_F_COPY_COMMENT: u32 = 33_554_432;
pub const _MM_F_EQX: u32 = 67_108_864;
pub const _MM_F_PAF_NO_HIT: u32 = 134_217_728;
pub const _MM_F_NO_END_FLT: u32 = 268_435_456;
pub const _MM_F_HARD_MLEVEL: u32 = 536_870_912;
pub const _MM_F_SAM_HIT_ONLY: u32 = 1_073_741_824;
pub const _MM_I_HPC: u32 = 1;
pub const _MM_I_NO_SEQ: u32 = 2;
pub const _MM_I_NO_NAME: u32 = 4;
pub const _MM_MAX_SEG: u32 = 255;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_idx_seq_t {
    pub name: *mut ::std::os::raw::c_char,
    pub offset: u64,
    pub len: u32,
    pub is_alt: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_idx_t {
    pub b: i32,
    pub w: i32,
    pub k: i32,
    pub flag: i32,
    pub n_seq: u32,
    pub index: i32,
    pub n_alt: i32,
    pub seq: *mut mm_idx_seq_t,
    pub S: *mut u32,
    pub B: *mut mm_idx_bucket_s,
    pub I: *mut mm_idx_intv_s,
    pub km: *mut ::std::os::raw::c_void,
    pub h: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct mm_extra_t {
    pub capacity: u32,
    pub dp_score: i32,
    pub dp_max: i32,
    pub dp_max2: i32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    pub n_cigar: u32,
    pub cigar: [u32; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_reg1_t {
    pub id: i32,
    pub cnt: i32,
    pub rid: i32,
    pub score: i32,
    pub qs: i32,
    pub qe: i32,
    pub rs: i32,
    pub re: i32,
    pub parent: i32,
    pub subsc: i32,
    pub as_: i32,
    pub mlen: i32,
    pub blen: i32,
    pub n_sub: i32,
    pub score0: i32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u8>,
    pub hash: u32,
    pub div: f32,
    pub p: *mut mm_extra_t,
}

impl Default for mm_idxopt_t {
    fn default() -> Self {
        Self {
            k: 15,
            w: 10,
            flag: 0,
            bucket_bits: 14,
            mini_batch_size: 50_000_000,
            batch_size: 4_000_000_000,
        }
    }
}

pub enum Preset {
    NoPreset,
    AvaOnt,
    AvaPb,
    Map10k,
    MapOnt,
    Asm5,
    Asm10,
    Asm20,
    Short,
    Splice,
    SliceHq,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_idxopt_t {
    pub k: ::std::os::raw::c_short,
    pub w: ::std::os::raw::c_short,
    pub flag: ::std::os::raw::c_short,
    pub bucket_bits: ::std::os::raw::c_short,
    pub mini_batch_size: i64,
    pub batch_size: u64,
}

impl mm_mapopt_t {
    pub fn from_preset(preset: &Preset) -> Self {
        let mut mopt = mm_mapopt_t::default();

        match preset {
            Preset::Short => {
                mopt.pe_ori = 1;
                mopt.flag = _MM_F_SR as i64
                    | _MM_F_FRAG_MODE as i64
                    | _MM_F_NO_PRINT_2ND as i64
                    | _MM_F_2_IO_THREADS as i64
                    | _MM_F_HEAP_SORT as i64;
                mopt.a = 2;
                mopt.b = 8;
                mopt.q = 12;
                mopt.e = 2;
                mopt.q2 = 24;
                mopt.e2 = 1;
                mopt.zdrop = 100;
                mopt.zdrop_inv = 100;
                mopt.end_bonus = 10;
                mopt.max_frag_len = 800;
                mopt.max_gap = 100;
                mopt.bw = 100;
                mopt.pri_ratio = 0.5;
                mopt.min_cnt = 2;
                mopt.min_chain_score = 25;
                mopt.min_dp_max = 40;
                mopt.best_n = 20;
                mopt.mid_occ = 1000;
                mopt.max_occ = 5000;
                mopt.mini_batch_size = 50_000_000;
            }
            _ => unimplemented!(),
        }

        mopt
    }
}

impl Default for mm_mapopt_t {
    fn default() -> Self {
        Self {
            // Default for all presets
            seed: 11,
            bw: 500,
            max_gap: 5000,
            max_gap_ref: -1,
            max_chain_skip: 25,
            max_chain_iter: 5000,
            min_cnt: 3,
            min_chain_score: 40,
            chain_gap_scale: 1.0,
            mask_level: 0.5,
            pri_ratio: 0.8,
            best_n: 5,
            max_join_long: 20000,
            max_join_short: 2000,
            min_join_flank_sc: 1000,
            min_join_flank_ratio: 0.5,
            alt_drop: 0.15,
            a: 2,
            b: 4,
            q: 4,
            e: 2,
            q2: 24,
            e2: 1,
            sc_ambi: 1,
            zdrop: 400,
            zdrop_inv: 200,
            min_dp_max: 80,
            min_ksw_len: 200,
            anchor_ext_len: 20,
            anchor_ext_shift: 6,
            max_clip_ratio: 1.0,
            pe_ori: 0,
            pe_bonus: 33,
            mid_occ_frac: 2.0e-4,
            mini_batch_size: 500_000_000,
            // Must be set depending on the preset
            flag: 0,
            sdust_thres: 0,
            max_qlen: 0,
            max_frag_len: 0,
            noncan: 0,
            junc_bonus: 0,
            end_bonus: -1,
            min_mid_occ: 0,
            mid_occ: 0,
            max_occ: 0,
            max_sw_mat: 0,
            split_prefix: std::ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_mapopt_t {
    pub flag: i64,
    pub seed: ::std::os::raw::c_int,
    pub sdust_thres: ::std::os::raw::c_int,
    pub max_qlen: ::std::os::raw::c_int,
    pub bw: ::std::os::raw::c_int,
    pub max_gap: ::std::os::raw::c_int,
    pub max_gap_ref: ::std::os::raw::c_int,
    pub max_frag_len: ::std::os::raw::c_int,
    pub max_chain_skip: ::std::os::raw::c_int,
    pub max_chain_iter: ::std::os::raw::c_int,
    pub min_cnt: ::std::os::raw::c_int,
    pub min_chain_score: ::std::os::raw::c_int,
    pub chain_gap_scale: f32,
    pub mask_level: f32,
    pub pri_ratio: f32,
    pub best_n: ::std::os::raw::c_int,
    pub max_join_long: ::std::os::raw::c_int,
    pub max_join_short: ::std::os::raw::c_int,
    pub min_join_flank_sc: ::std::os::raw::c_int,
    pub min_join_flank_ratio: f32,
    pub alt_drop: f32,
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
    pub q: ::std::os::raw::c_int,
    pub e: ::std::os::raw::c_int,
    pub q2: ::std::os::raw::c_int,
    pub e2: ::std::os::raw::c_int,
    pub sc_ambi: ::std::os::raw::c_int,
    pub noncan: ::std::os::raw::c_int,
    pub junc_bonus: ::std::os::raw::c_int,
    pub zdrop: ::std::os::raw::c_int,
    pub zdrop_inv: ::std::os::raw::c_int,
    pub end_bonus: ::std::os::raw::c_int,
    pub min_dp_max: ::std::os::raw::c_int,
    pub min_ksw_len: ::std::os::raw::c_int,
    pub anchor_ext_len: ::std::os::raw::c_int,
    pub anchor_ext_shift: ::std::os::raw::c_int,
    pub max_clip_ratio: f32,
    pub pe_ori: ::std::os::raw::c_int,
    pub pe_bonus: ::std::os::raw::c_int,
    pub mid_occ_frac: f32,
    pub min_mid_occ: i32,
    pub mid_occ: i32,
    pub max_occ: i32,
    pub mini_batch_size: i64,
    pub max_sw_mat: i64,
    pub split_prefix: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_tbuf_s {
    _unused: [u8; 0],
}
pub type mm_tbuf_t = mm_tbuf_s;

#[link(name = "minimap2")]
extern "C" {
    #[doc = " Create an index from strings in memory"]
    #[doc = ""]
    #[doc = " @param w            minimizer window size"]
    #[doc = " @param k            minimizer k-mer size"]
    #[doc = " @param is_hpc       use HPC k-mer if true"]
    #[doc = " @param bucket_bits  number of bits for the first level of the hash table"]
    #[doc = " @param n            number of sequences"]
    #[doc = " @param seq          sequences in A/C/G/T"]
    #[doc = " @param name         sequence names; could be NULL"]
    #[doc = ""]
    #[doc = " @return minimap2 index"]
    pub fn mm_idx_str(
        w: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        is_hpc: ::std::os::raw::c_int,
        bucket_bits: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        seq: *mut *const ::std::os::raw::c_char,
        name: *mut *const ::std::os::raw::c_char,
    ) -> *mut mm_idx_t;
}

#[link(name = "minimap2")]
extern "C" {
    #[doc = " Destroy/deallocate an index"]
    #[doc = ""]
    #[doc = " @param r          minimap2 index"]
    pub fn mm_idx_destroy(mi: *mut mm_idx_t);
}
#[link(name = "minimap2")]
extern "C" {
    #[doc = " Initialize a thread-local buffer for mapping"]
    #[doc = ""]
    #[doc = " Each mapping thread requires a buffer specific to the thread (see mm_map()"]
    #[doc = " below). The primary purpose of this buffer is to reduce frequent heap"]
    #[doc = " allocations across threads. A buffer shall not be used by two or more"]
    #[doc = " threads."]
    #[doc = ""]
    #[doc = " @return pointer to a thread-local buffer"]
    pub fn mm_tbuf_init() -> *mut mm_tbuf_t;
}
#[link(name = "minimap2")]
extern "C" {
    #[doc = " Destroy/deallocate a thread-local buffer for mapping"]
    #[doc = ""]
    #[doc = " @param b          the buffer"]
    pub fn mm_tbuf_destroy(b: *mut mm_tbuf_t);
}

#[link(name = "minimap2")]
extern "C" {
    #[doc = " Align a query sequence against an index"]
    #[doc = ""]
    #[doc = " This function possibly finds multiple alignments of the query sequence."]
    #[doc = " The returned array and the mm_reg1_t::p field of each element are allocated"]
    #[doc = " with malloc()."]
    #[doc = ""]
    #[doc = " @param mi         minimap2 index"]
    #[doc = " @param l_seq      length of the query sequence"]
    #[doc = " @param seq        the query sequence"]
    #[doc = " @param n_regs     number of hits (out)"]
    #[doc = " @param b          thread-local buffer; two mm_map() calls shall not use one buffer at the same time!"]
    #[doc = " @param opt        mapping parameters"]
    #[doc = " @param name       query name, used for all-vs-all overlapping and debugging"]
    #[doc = ""]
    #[doc = " @return an array of hits which need to be deallocated with free() together"]
    #[doc = "         with mm_reg1_t::p of each element. The size is written to _n_regs_."]
    pub fn mm_map(
        mi: *const mm_idx_t,
        l_seq: ::std::os::raw::c_int,
        seq: *const ::std::os::raw::c_char,
        n_regs: *mut ::std::os::raw::c_int,
        b: *mut mm_tbuf_t,
        opt: *const mm_mapopt_t,
        name: *const ::std::os::raw::c_char,
    ) -> *mut mm_reg1_t;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_idx_bucket_s {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_idx_intv_s {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mm_bseq_file_s {
    pub _address: u8,
}
