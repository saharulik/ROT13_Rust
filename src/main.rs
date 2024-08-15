use std::io::Read;
use std::io::Result;


struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}


impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {

        let low_asc = |x: &mut u8| {
                let base = if x.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                *x = (*x - base + self.rot) % 26 + base;
        };
        
        match self.input.read(buf) {
            Ok(ok) => {
                buf.iter_mut()
                    .filter(|x| x.is_ascii_alphabetic())
                    .for_each(low_asc);
                Ok(ok)
            },
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let mut rot = RotDecoder {
        input: "Gb trg gb gur bgure fvqr!".as_bytes(),
        rot: 13,
    };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();

    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
