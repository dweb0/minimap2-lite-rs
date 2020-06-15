mod bindings;
use bindings::*;
use std::ffi::{CStr, CString};

pub use bindings::Preset;

pub struct Index {
    n_refseqs: usize,
    inner: *mut mm_idx_t,
    tbuf: ThreadBuffer,
    mopt: mm_mapopt_t,
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe { mm_idx_destroy(self.inner) }
    }
}

pub struct ThreadBuffer {
    inner: *mut mm_tbuf_s,
}

impl ThreadBuffer {
    fn new() -> Self {
        let inner = unsafe { mm_tbuf_init() };
        Self { inner }
    }
}

impl Drop for ThreadBuffer {
    fn drop(&mut self) {
        unsafe { mm_tbuf_destroy(self.inner) }
    }
}

impl Index {
    /// Create a new Minimap2 index from a list of reference sequences
    /// and their names
    /// 
    /// k = kmer-size [default: 15]  
    /// w = minimizer window size [default: 10]
    pub fn from_reference_sequences<T, K, W>(
        seqs: &[T],
        names: &[T],
        preset: &Preset,
        k: K,
        w: W,
    ) -> Self
    where
        T: AsRef<str>,
        K: Into<Option<i32>>,
        W: Into<Option<i32>>
    {
        assert_eq!(seqs.len(), names.len());
        let n_refseqs = seqs.len();

        let k = match k.into() {
            Some(k) => k,
            None => 15
        };

        let w = match w.into() {
            Some(w) => w,
            None => 10
        };

        let seqs: Vec<_> = seqs
            .iter()
            .map(|x| CString::new(x.as_ref()).unwrap())
            .collect();

        let mut seqs: Vec<_> = seqs.iter().map(|x| x.as_ptr()).collect();

        let names: Vec<_> = names
            .iter()
            .map(|x| CString::new(x.as_ref()).unwrap())
            .collect();

        let mut names: Vec<_> = names.iter().map(|x| x.as_ptr()).collect();

        let inner = unsafe {
            mm_idx_str(
                k,
                w,
                0,
                14,
                n_refseqs as i32,
                seqs.as_mut_ptr(),
                names.as_mut_ptr(),
            )
        };
        let tbuf = ThreadBuffer::new();
        let mopt = mm_mapopt_t::from_preset(preset);

        Self {
            n_refseqs,
            inner,
            tbuf,
            mopt,
        }
    }

    /// Map a query sequence against the Minimap2 index and return the
    /// top alignment
    pub fn map_seq(&self, seq: &str) -> Option<Alignment> {
        let mut n_regs = 0;
        let reg = unsafe {
            mm_map(
                self.inner,
                seq.len() as i32,
                seq.as_ptr() as *const i8,
                &mut n_regs,
                self.tbuf.inner,
                &self.mopt,
                CString::default().as_ptr(),
            )
        };

        if n_regs > 0 {
            let aln = unsafe { *reg.offset(1) };

            if aln.score == 0 {
                return None;
            }

            // TODO: Weird bug
            if aln.rid < 0 || aln.rid > self.n_refseqs as i32 {
                return None;
            }

            let target_name = unsafe {
                let seqs = self.inner.as_ref().unwrap().seq;
                let seq = *seqs.offset(aln.rid as isize);
                CStr::from_ptr(seq.name).to_str().unwrap().to_owned()
            };

            let alignment = Alignment {
                score: aln.score,
                target_name,
                xstart: aln.qs as usize,
                xend: aln.qe as usize,
                ystart: aln.rs as usize,
                yend: aln.re as usize,
            };

            Some(alignment)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alignment {
    pub score: i32,
    pub target_name: String,
    pub xstart: usize,
    pub xend: usize,
    pub ystart: usize,
    pub yend: usize,
}
