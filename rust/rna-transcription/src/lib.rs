#[derive(Debug, PartialEq, Eq)]
enum DnaNucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: Vec<DnaNucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
enum RnaNucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Uracil,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: Vec<RnaNucleotide>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = dna
            .chars()
            .enumerate()
            .map(|(idx, chr)| match chr {
                'A' => Ok(DnaNucleotide::Adenine),
                'C' => Ok(DnaNucleotide::Cytosine),
                'G' => Ok(DnaNucleotide::Guanine),
                'T' => Ok(DnaNucleotide::Thymine),
                _ => Err(idx),
            })
            .collect::<Result<Vec<DnaNucleotide>, usize>>();
        Ok(Dna {
            nucleotides: nucleotides?,
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            nucleotides: self
                .nucleotides
                .into_iter()
                .map(|dna_nucleotide| match dna_nucleotide {
                    DnaNucleotide::Adenine => RnaNucleotide::Uracil,
                    DnaNucleotide::Cytosine => RnaNucleotide::Guanine,
                    DnaNucleotide::Guanine => RnaNucleotide::Cytosine,
                    DnaNucleotide::Thymine => RnaNucleotide::Adenine,
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides = rna
            .chars()
            .enumerate()
            .map(|(idx, chr)| match chr {
                'A' => Ok(RnaNucleotide::Adenine),
                'C' => Ok(RnaNucleotide::Cytosine),
                'G' => Ok(RnaNucleotide::Guanine),
                'U' => Ok(RnaNucleotide::Uracil),
                _ => Err(idx),
            })
            .collect::<Result<Vec<RnaNucleotide>, usize>>();

        Ok(Rna {
            nucleotides: nucleotides?,
        })
    }
}
