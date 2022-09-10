/**
* Deoxyribonucleic acid, DNA is the primary information storage molecule in 
* biological systems. It is composed of four nucleic acid bases Guanine ('G'), 
* Cytosine ('C'), Adenine ('A'), and Thymine ('T').
* Ribonucleic acid, RNA, is the primary messenger molecule in cells. 
* RNA differs slightly from DNA its chemical structure and contains no Thymine.
* In RNA Thymine is replaced by another nucleic acid Uracil ('U').

Create a function which translates a given DNA string into RNA.

*
*/
#[allow(unused)]
fn dna_to_rna(dna: &str) -> String {
    let dna = dna.chars();
    let mut ans = String::from("");
    for i in dna {
        if i == 'T' {
            ans.push('U')
        } else {
            ans.push(i)
        }

    }

    ans
    
}

#[cfg(test)]
mod tests {
    use super::dna_to_rna;

    #[test]
    fn returns_expected() {
      assert_eq!(dna_to_rna("TTTT"), "UUUU");
      assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }
}
