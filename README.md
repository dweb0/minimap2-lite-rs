# minimap2-lite-rs

Very minimal Rust bindings for minimap2

**Work in progress**

## Example usage

```rust

use minimap2_lite::{Index, Preset};
use seq_io::fasta::{self, Record};

fn main() {

    // Read reference sequences
    let mut rdr = fasta::Reader::from_path("reference.fa").unwrap();
    let mut seqs = vec![];
    let mut names = vec![];
    while let Some(record) = rdr.next() {
        let record = record.unwrap();
        let name = record.id().unwrap().to_owned();
        let seq = String::from_utf8(record.full_seq().to_vec()).unwrap();
        names.push(name);
        seqs.push(seq);
    }

    // Build the minimap2 index
    let idx = Index::from_reference_sequences(&seqs, &names, &Preset::Short, None, None);

    // Walk through query fasta and map each sequence using the minimap2 index
    let mut rdr = fasta::Reader::from_path("query.fa").unwrap();
    while let Some(record) = rdr.next() {
        let record = record.unwrap();
        let seq = record.full_seq();
        let seq = std::str::from_utf8(&seq).unwrap();
        let alignment = idx.map_seq(seq);
        if let Some(alignment) = alignment {
            println!("{:?}", alignment);
        }
    }
}
```